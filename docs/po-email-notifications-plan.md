# Purchase Order email notifications: audit and sprint plan

Status: approved and implemented. This document preserves the pre-sprint audit and agreed plan; the [implementation and rollout guide](po-email-notifications.md) describes current behavior. Scope: email only; in-app delivery remains deferred.

## Investigation result

PO creation currently commits the order and returns its detail without dispatching a notification (`src/services/purchase_order.rs`, `create_complete`). Cancellation and reactivation also only persist their status changes. Receipt recording and defect reporting/resolution do not dispatch PO notifications. Receipt recording accepts `_config` but does not use it.

WSO creation, by comparison, explicitly builds a `NotificationContext` and calls `notifications::dispatch` after committing. That dispatch resolves settings/recipients, renders a template, and creates an email log/job for the existing SMTP worker.

The renderer only registers WSO templates. Every `po_*` event would reach its unsupported-template error if dispatch were called with enabled recipients. Thus adding recipients or rerunning migrations alone cannot enable PO email.

This is a source-code audit, consistent with the reported web-app behavior. No live database settings, deployed binary, SMTP delivery or recipient inboxes were inspected, and no emails were sent during this investigation.

## Existing foundation

- Ten PO event definitions and corresponding notification settings are seeded.
- Event-module grouping and per-event recipient management already exist in the frontend.
- Shared sender identity, SMTP transport, logs, queue and ten-second worker exist.
- PO records contain references, supplier, products, deadlines, receipts, defects and actor names needed for email content.
- PO settings expose grace days (0), approaching window (2), stalled days (3), unresolved-defect days (3), and sweep interval (60 minutes).
- Existing integration tests cover PO business operations and generic notification persistence, but do not assert PO event dispatch or email delivery.

## Event coverage

All ten events have definitions/settings but lack PO templates and end-to-end dispatch.

| Event | Missing trigger and intended scope |
| --- | --- |
| `po_created` | Successful complete order creation; once per PO creation. |
| `po_cancelled` | Successful active-to-cancelled transition; once per transition. |
| `po_reactivated` | Successful cancelled-to-active transition; once per transition. |
| `po_completed` | Transition into derived completion after receipts, defect acceptance, or other relevant changes; once per completion episode. |
| `po_defect_reported` | Successful defect creation; once per defect record. |
| `po_branding_goods_ready` | Branded item becomes fully accepted; once per readiness episode. |
| `po_delivery_approaching` | Scheduled evaluation of outstanding item deadlines within the configured window. |
| `po_delivery_overdue` | Scheduled evaluation of outstanding item deadlines after configured grace. |
| `po_partial_stalled` | Scheduled evaluation of partially received items with outstanding quantities and no receipt within the configured period. |
| `po_defect_unresolved` | Scheduled evaluation of open defects beyond the configured age. |

No generic PO receipt, attachment or defect-resolved notification event is currently seeded. Do not introduce these implicitly; defect resolution can still cause an existing completion/readiness event.

## Shared gaps to address

1. Dispatch honors the master enabled flag, but does not check `email_enabled` before creating email jobs. Enforce both flags and enabled recipients. In-app settings must never cause email delivery.
2. Log and job inserts are separate operations, so enqueue failure can leave a pending log without a job. Queue creation must be transactional.
3. There is no durable PO event occurrence/transition tracking. Repeated scans, restarts and concurrent requests need database-enforced deduplication.
4. The worker reads all pending jobs without a claim/lease. Multiple instances can send the same job. Failed jobs have no retry schedule, and successful SMTP acceptance followed by a crash can still lead to duplicate delivery.
5. Logs identify event and recipient but not the PO/item/defect or originating occurrence. Add correlation for investigation and recovery.
6. The only startup worker sends pending jobs. There is no PO alert sweep. The sweep-interval setting is unused, and the dashboard overdue query ignores configured grace days.
7. PO completion is derived, not stored. Receipt-only hooks miss changes from accepted defects, quantity edits, item additions, cancellation/reactivation and branding edits.
8. The frontend exposes an operative-looking in-app switch. Mark it deferred/disabled while preserving stored values and the existing API contract.

## Proposed implementation sequence

### 1. Define event semantics and add a forward migration

Use a new migration after the existing baseline; do not edit applied migrations or reset an existing database. Establish its actual migration state before rollout if this installation still uses manually initialized SQL.

Add durable PO email occurrences with event code, PO/item/defect references, actor, payload snapshot, occurrence key and processing state. Unique occurrence keys prevent repeated enqueueing. Add transition state for completion/readiness and episode tracking for timed conditions. Link recipient jobs to occurrences, enforce one job per occurrence/normalized recipient, and add queue claim/retry fields as needed.

Recommended reliability model: persist the business change and its notification occurrence in the same database transaction. Render/enqueue from that durable occurrence asynchronously. SMTP failure never undoes a saved PO; a failed business transaction never emits an email. Failure to persist the occurrence fails the transaction rather than silently losing the notification. Refactor the required repository transaction boundaries without changing business rules.

### 2. Deliver the first vertical slice: PO created email

Implement shared PO email context and escaped HTML rendering, then the `po_created` template and renderer registration. Include both order references, supplier, creator, relevant item/quantity summary, and a link to `/purchase-orders/{id}` using the configured frontend origin. Do not attach uploaded files automatically or treat supplier contact information as a recipient.

Connect the PO creation service to occurrence persistence; pass authenticated actor information through the handler. Resolve configured recipients and enforce master/email flags. Store log/job records atomically. Verify creation through the web app queues and sends to the configured development recipient.

### 3. Add immediate lifecycle and derived-state events

Implement cancellation, reactivation and defect-report events with the same infrastructure. Add a shared PO state evaluator for completion and branded-item readiness, preserving existing fulfilment arithmetic. Invoke it at relevant mutations, including receipt creation, defect reporting/resolution, item/line changes, and reactivation. Maintain episode state when completion/readiness becomes false so a later transition can notify once again.

Do not emit notifications from detail/dashboard GET endpoints. Templates should show the event's committed snapshot, not silently describe a later edited state.

### 4. Add scheduled attention emails

Create repository candidate queries and a PO alert service. Run a separate startup task using `alert_sweep_interval_minutes`, reloading settings each cycle. Reuse the same occurrence/queue pipeline. Exclude cancelled orders; suppress delivery alerts for fully accepted items and unresolved-defect alerts for resolved defects. Group size lines into item-level notices to avoid one email per size.

Apply configured grace/window/age thresholds and establish an explicit business timezone for date boundaries. Update the dashboard overdue calculation to agree with notification semantics. A scan must not mark an occurrence notified when disabled or without recipients; retain a diagnosable suppressed/deferred outcome and re-evaluate current eligibility before later queueing.

### 5. Harden delivery and add regression coverage

Claim jobs atomically with bounded leases, retry transient delivery failures with bounded backoff, preserve final failure details, and update job/log status consistently. Distinguish queued, claimed, sent, retryable and terminal failure outcomes in persistence and diagnostic logs. Do not promise exactly-once SMTP delivery: a crash after SMTP acceptance but before recording success remains ambiguous.

Test all event templates and escaped user text; each immediate trigger; disabled/master/email flags; inactive and duplicate recipients; rollback; enqueue failure; concurrent claims; retries; repeated scans/restarts; threshold boundaries; settings edits; cancellation; and completion/readiness re-entry. Include WSO regressions because delivery infrastructure is shared. Use a local SMTP capture service for automated delivery checks; perform a controlled configured-recipient web-app test only when explicitly authorized.

## Approved policy defaults

- Timed alerts: one email per qualifying episode, no daily reminders this sprint.
- Deadline edits: a changed due date starts a new deadline occurrence if it qualifies; threshold changes alone do not resend an already delivered alert for the same episode.
- Stalled delivery: measure from the latest receipt affecting that item, and require some delivery plus positive outstanding quantity.
- Completion/readiness: re-entry can notify again after a real state reversal; evaluate per-line readiness to avoid masking outstanding sizes, subject to reconciliation with the existing aggregate status calculation.
- Calendar boundaries: use Africa/Nairobi as the proposed business timezone; specify whether age thresholds use elapsed 24-hour periods or calendar days in tests.
- Initial activation: send immediate events only for new actions; for existing overdue/stalled/unresolved records, use an explicit rollout policy to avoid an unexpected backlog burst.
- Settings changes: suppress future queueing when email is disabled; decide whether already queued jobs are also cancelled. Recommended: recheck settings/recipient eligibility before first delivery and retries, recording suppression.

## Acceptance criteria

- Creating a PO with email enabled and configured recipients produces a correlated email; disabling email produces none regardless of the in-app setting.
- All ten existing PO events have tested trigger semantics and templates.
- Scheduled events work without visiting dashboards and honor the configured thresholds.
- Repeated scans and concurrent operations do not create duplicate occurrence/recipient jobs.
- SMTP failure is visible and retryable within a bounded policy; no orphan pending logs.
- Existing WSO emails continue to work and now honor the email toggle.
- No in-app delivery, inbox, badge, websocket or push-notification feature is implemented.

Recommended delivery checkpoints: foundation + `po_created`; remaining immediate/derived events; scheduled alerts + full delivery regression. These are ordered increments of the same email sprint, not competing designs.
