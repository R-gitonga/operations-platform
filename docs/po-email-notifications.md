# Purchase order email notifications

Implemented scope: all ten existing PO email events. In-app notifications remain deferred; their stored settings are preserved, and the frontend labels that channel “Deferred”.

## Event rules

| Event | When it is recorded | Episode |
| --- | --- | --- |
| `po_created` | A complete PO creation transaction commits | Creation |
| `po_cancelled` | Active changes to cancelled | Status transition |
| `po_reactivated` | Cancelled changes to active | Status transition |
| `po_completed` | Every size line is fully accepted on a non-cancelled PO | Each transition into completion |
| `po_branding_goods_ready` | Every size line of a branding-required item is fully accepted | Each transition into readiness |
| `po_defect_reported` | A defect is recorded | Defect record |
| `po_delivery_approaching` | Outstanding item deadline is between today and today + configured window, inclusive | Item deadline revision |
| `po_delivery_overdue` | Outstanding item deadline + grace days is before today | Item deadline revision |
| `po_partial_stalled` | Some quantity was delivered, a size remains outstanding, and the latest receipt affecting the item is at least the configured age | Latest item receipt |
| `po_defect_unresolved` | An open defect is at least the configured age | Defect record |

Deadline dates use **Africa/Nairobi**. Receipt/defect age uses elapsed 24-hour days, with equality qualifying. Cancelled orders do not qualify for timed alerts or completion/readiness. Non-accepted defects reduce accepted quantity, including defects resolved as replaced, returned for credit, or written off; an accepted resolution restores that quantity. Existing receipt rules are unchanged.

There are no daily reminders. Threshold edits do not repeat an already queued alert for the same episode. A due-date edit increments a revision, including changing it back to an earlier date. A new receipt resets the stalled clock. Completion/readiness can notify again after a real reversal, including a defect, quantity edit, item addition or cancellation. The derived PO status now uses per-line completion too, preventing an excess in one size from masking a shortage in another.

## Persistence and delivery

Migration `202610150006_po_email_notifications.sql` adds occurrences, transition state, timed history, queue leases/retry fields, and two SQL views. It leaves the five original baseline migrations unchanged.

Deferred PostgreSQL constraint triggers capture immediate events and reconcile readiness at the end of the business transaction. They see the complete set of receipt/item lines and roll back with the mutation. A PO row lock serializes reconciliation; unique occurrence keys and normalized recipient indexes prevent duplicate occurrences/jobs. Triggers cover order status, items, size lines, receipt lines, and defects. Actor names are passed transaction-locally for cancellation, reactivation, receipt and defect operations; creator names are included separately. Changes without an authenticated actor context omit the action-by field rather than attributing them to the creator.

The email worker checks every ten seconds. It renders pending PO occurrences with escaped HTML and creates recipient jobs and logs in one transaction. PO emails include references, supplier, creator, item quantities and deadlines, applicable branding/defect/threshold details, and a link built from `FRONTEND_BASE_URL`. Files are not attached, and supplier contact information is never an implicit recipient.

The scheduler runs independently of HTTP/dashboard visits. It sweeps on startup and then reloads `po_settings.alert_sweep_interval_minutes` after each sweep. Multiple instances may sweep safely. Settings changes take effect on the next cycle; the interval currently being slept is not interrupted. The overdue dashboard uses the same Nairobi date and grace rule.

Both master and email switches must be enabled. Only enabled configured recipients are used; addresses are trimmed and deduplicated without case sensitivity. Immediate occurrences lacking eligibility are suppressed, with no later historical replay. Timed conditions lacking eligibility remain eligible for reevaluation on a later sweep; they are checked again before queueing. Initial migration separately baselines existing qualifying timed conditions and readiness without emitting emails.

Before every SMTP attempt, the worker rechecks switches and recipient eligibility. It marks an ineligible job/log `suppressed`. In-app flags never cause email delivery. This check also applies to existing WSO email jobs.

Each worker atomically claims one job with a two-minute lease. SMTP has a 60-second timeout. Expired claims can be recovered, and an attempt token prevents an expired worker from overwriting a newer result. A claim updates its log to `processing`; completion updates the job/log atomically. Transient failures retry up to five attempts with 60, 120, 240 and 480 second delays. Invalid addresses/message construction and permanent SMTP responses fail immediately. Terminal failures retain their error for investigation; there is no automatic infinite retry or resend UI.

SMTP delivery is **at least once**: acceptance by the mail server followed by a crash before recording success can result in a duplicate on recovery. “Sent” means SMTP accepted the message, not that the recipient inbox delivered/read it.

## Rollout

1. Back up the target database and stop the old API/worker so it cannot compete with the upgraded queue worker.
2. Confirm the connection and migration history with `sqlx migrate info`. This forward migration expects the five SQLx baseline migrations already installed. A manually initialized legacy database requires the baseline adoption procedure first; do not reset it or run the baseline over existing tables.
3. Run `sqlx migrate run`, then run it again to confirm there is nothing pending. No live application database was migrated as part of implementation validation.
4. Build/start the updated backend and frontend. Keep the normal SMTP configuration and set `FRONTEND_BASE_URL` to the browser-facing application origin.
5. In Notification Settings, enable the intended PO events and email channel, then configure recipients for each event. Review Procurement Settings thresholds and sweep interval.
6. Create a new test PO using the web application and inspect its correlated job/log. This is the deployment smoke test; real mailbox delivery was not exercised by the automated tests.

Migration baselining does not create a backlog of existing completed, ready, overdue, stalled or old unresolved-defect emails. New actions and subsequently qualifying episodes work normally. Existing legacy `failed` jobs are not automatically revived.

## Diagnostics

Inspect recent email outcomes and their PO correlation:

```sql
SELECT j.id, e.code, o.purchase_order_id, o.occurrence_key,
       j.recipient_email, j.status, j.attempts, j.available_at,
       j.lease_until, j.error_message
FROM notification_jobs j
JOIN notification_logs l ON l.id = j.notification_log_id
JOIN notification_events e ON e.id = l.notification_event_id
LEFT JOIN po_notification_occurrences o ON o.id = j.occurrence_id
ORDER BY j.id DESC LIMIT 100;

SELECT id, event_code, purchase_order_id, state, created_at
FROM po_notification_occurrences ORDER BY id DESC LIMIT 100;
```

`pending` occurrences await rendering; `queued` means recipient jobs were committed; `suppressed` means eligibility was absent/stale when evaluated. A queued occurrence is not proof of delivery: inspect job status. Timed history records the one-per-episode decision on queueing, not SMTP success. Failed or suppressed jobs do not cause daily repeat alerts. Investigate terminal failure before any deliberate manual resend, considering the SMTP ambiguity above.

## Validation

Against a disposable PostgreSQL server whose role can create test databases:

```powershell
$env:DATABASE_URL = 'postgres://TEST_USER:TEST_PASSWORD@localhost:5432/postgres'
cargo test --locked --test baseline --test po_email -- --include-ignored --nocapture
```

If a running Windows API locks `target/debug/wso-tracker.exe`, use `--target-dir target/notification-tests` to build tests separately.

Coverage includes all templates/HTML escaping, lifecycle snapshots and actors, rollback after trigger execution, defect acceptance and quantity-edit reentry, concurrent receipts and queueing/claims, missing/duplicate/disabled recipients, master/email switches, WSO dispatch, atomic enqueue failure recovery, lease fencing, bounded retries, permanent failures, timed boundaries/grace and episode deduplication, stale-condition suppression, and upgrading existing historical records. The SMTP test captures mail on loopback using a test-only plain transport; production still requires STARTTLS. No external emails are sent.

Verified on 2026-10-15 using disposable PostgreSQL 18.6: both test targets passed (12 test executions, including the template test included in both targets); 123 standalone repository SQL statements/fragments prepared successfully. All six migrations applied to an empty database, a second migration run was a no-op, and the resulting schema contained 32 application tables. The populated-baseline upgrade test passed without historical email creation. Backend compilation and the frontend production build passed. ESLint passed for every changed frontend file; repository-wide lint remains at 28 errors in other files. Build output retains existing Rust warnings and a frontend chunk-size warning.
