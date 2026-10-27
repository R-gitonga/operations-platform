# Paid orders sprint plan

Date: 2026-10-26
Status: Implemented; see paid-orders.md for usage, validation and rollout notes.
Branches: `feature/paid-orders` in both backend and frontend repositories.

## Outcome

Track expedited customer orders separately from bulk work while preserving the existing PO and WSO workflows. A branch can place a paid order for a made-to-measure garment or a standard item that was out of stock. Staff should find these orders quickly, follow existing progress and receipts, and filter them by branch, order reason, and product category.

“Paid order” is an operational classification in this sprint. It does not confirm a payment, record an amount, or introduce accounting or payment collection.

## Recommended experience

Add a **Paid order** checkbox to the header of both creation forms. Leave it unchecked by default. When checked, reveal a required Branch dropdown at order level. Each item then shows its own required reason dropdown:

- **Branch:** active branches configured in Settings.
- **Item reason:** Make to measure (MTM), or Stock-out.

Order reason is separate from the existing product category: a shirt can be either an MTM or a stock-out order. Storing both dimensions now enables meaningful reporting later. The two reasons are fixed values in this sprint; branch configuration is editable.

Unchecking clears the conditional fields. If there are no active branches, show a clear configuration message and prevent paid-order submission; ordinary order creation still works. Keep every existing product, size, supplier, branding, attachment, production and receiving step intact. An order may contain several products but belongs to one branch, with a separate reason for each item. Mixed MTM and stock-out items are supported in both WSOs and POs; split only mixed-branch work into separate orders.

After saving a paid order, open its existing detail page, where paid classification, branch, and each item reason appear. Provide a link back to Paid Orders. Existing detail pages remain the place to manage production, receipts, defects, cancellation and reactivation.

Add **Paid Orders** to the sidebar. This is one combined follow-up page for POs and WSOs, with an All / WSO / PO filter. Use the existing order reference and a source-type badge to distinguish records.

Recommended separation: existing PO and WSO lists default to Bulk, with an explicit All / Bulk / Paid filter. Paid Orders lists only paid orders. Existing dashboards continue to include all orders unless a classification filter is selected; label the scope clearly so existing totals do not silently change.

## Data and integrity

Use a new forward SQLx migration after `202610150006`; do not edit applied migrations.

Create a shared `branches` table with an ID, unique stable code, display name, active flag, and creation/update timestamps. Enforce trimmed nonempty codes/names and case-insensitive unique branch codes. Add no invented production branch records.

Add the same header fields to `wso_orders` and `purchase_orders`:

| Field | Meaning |
| --- | --- |
| `is_paid_order` | Required boolean, default false |
| `branch_id` | Nullable foreign key to branches |

Add `paid_order_reason` to `wso_items` and `po_items`, constrained to `mtm` or `stock_out`. It is not an order-header field.

Database constraints require a branch on paid headers and a reason on every paid item; bulk headers have no branch and bulk items have no reason. Cross-table validation runs at transaction commit to permit atomic corrections. Backfill all existing orders as bulk; do not infer classification from descriptions or historical references. Foreign keys prevent orphaned branch assignments. Add indexes based on the combined follow-up query, including branch and creation date for paid orders.

Allow authorized users to correct classification, branch and reason from existing order details. Record the old/new metadata, actor and timestamp in a small audit table, including initial paid classification. This preserves correction history for reporting and prevents silent reclassification. Save metadata and its audit event in the same transaction.

Branch deactivation prevents new assignments but preserves existing orders and filters. Editing unrelated fields on an order assigned to an inactive branch remains valid; changing its branch requires an active destination. Use explicit update semantics so omitted fields preserve existing data and clearing a paid classification clears both dependent fields atomically.

Branch rename retains the stable ID/code relationship; this sprint displays the current branch name on historical orders. Audit branch configuration changes if reusing existing audit infrastructure is practical; historical name snapshots are not required for the initial reports.

## Backend work

1. Implement branch models, repository, service, handlers and routes following the existing supplier/settings conventions. Authenticated users can read active branches; administrators can create, rename, deactivate and reactivate them. Enforce permissions on the server, not just in the UI. Avoid hard deletion.
2. Extend every supported WSO/PO creation path, update payload, repository query, list/summary response and detail response with the classification fields. Older clients omitting the new fields create bulk orders. Validate combinations and branch eligibility server-side.
3. Add explicit classification filters to existing list endpoints, keeping their API default compatible with existing callers. The frontend requests Bulk for the ordinary lists. Existing dashboard calculations retain their scope.
4. Add a paginated, server-filtered `GET /paid-orders` endpoint. Normalize the two order types into one read model without duplicating orders. Use `(order_type, order_id)` as the identity; numeric IDs can overlap across tables.
5. Reuse existing fulfilment/status rules. Do not sum receipt, item and defect joins in a way that multiplies quantities. Count orders once when category filtering matches several items. Return total matching count and stable ordering before pagination.

Suggested unified row: source type and ID, references, branch ID/code/name, reason, distinct product categories, created date, current status, ordered/raised quantity, received/accepted quantity, outstanding quantity, and detail-page destination. Define the WSO and PO quantity labels explicitly; received and accepted are not interchangeable.

Filters: branch (including inactive branches with historical orders), order reason, product category, WSO/PO, status/open versus closed, creation-date range, and reference search. Apply date boundaries using Africa/Nairobi. Use `EXISTS` or equivalent category matching to prevent duplicate orders. An order without a product category remains visible when no category filter is set.

Default follow-up view: open paid orders, oldest created first, with deterministic source/ID tie-breaking. Completed/cancelled orders remain available through filters. “Age” means time since order creation, not time since last edit or reclassification. Any last-activity display must use a documented operational event source rather than imply that header edits are production progress.

## Frontend work

1. Add a Settings → Branches page with list, create/edit, and deactivate/reactivate controls. Match existing admin settings patterns and error handling.
2. Build a reusable paid-order field group for both creation forms and header correction controls. Update TypeScript request/response models, hooks and API clients.
3. Add branch/reason badges and metadata to PO and WSO detail screens and relevant list rows. Use visible labels rather than colour alone.
4. Build `/paid-orders` with loading, empty, error and retry states; paginated results; filter state in the URL; and links to existing details. Reset pagination when filters change.
5. Add the sidebar entry and All / Bulk / Paid filters to ordinary lists. Keep back-navigation and deep links predictable. Invalidate both ordinary and paid-order queries after creation, correction and workflow changes.

Likely integration points include `CreateOrderForm.tsx`, `WsoHeaderForm.tsx`, `CreatePurchaseOrderForm.tsx`, `SettingsPage.tsx`, `config/navigation.ts`, existing order detail/list pages, and the WSO/PO backend model, service and repository modules. Inspect all endpoint variants before implementation rather than relying only on the visible forms.

## Reporting foundations and boundaries

Persist branch ID, explicit reason, classification, original creation timestamps, and classification audit events now. Keep product categories at item level, where they already belong. Keep a unified read model reusable by later reporting/export endpoints.

Initial follow-up information supports counts and outstanding quantities by branch, reason, category and source. Category totals are not necessarily additive because an order can contain multiple categories; reports must distinguish distinct order counts from item/quantity counts. Report WSO and PO quantities with their own fulfilment semantics.

Later reporting candidates: intake volume, current open backlog, backlog age, branch demand, MTM versus stock-out mix, and turnaround/on-time performance. Reliable turnaround metrics require durable completion/reopening events and a documented policy; current status or `updated_at` alone is insufficient. Historical backlog snapshots and historical branch-name reporting also need deliberate data capture. Do not promise those metrics from the first sprint's current-state page.

This sprint does not add payment processing, customer/contact records, measurement capture, a separate production workflow, new notification event types, automatic queue reprioritization, new SLA targets, exports, or a general report builder. Expediting is supported through visibility and follow-up; existing workflow rules continue to apply. These can be separate follow-on scopes when required.

### Receiving follow-up (27 October 2026)

Confirmed scope: existing workshop/supplier receipts only, with no separate branch confirmation. Paid Orders now provides a receiving panel using the source module's receipt dialogs and validation, alongside clickable order cards and styled creation buttons. The panel shows all source items regardless of list filters. Existing WSO/PO notification events and recipient settings apply, with paid-order branch and reason context included in emails. A forward migration enriches future PO notification snapshots. Automated integration coverage verifies paid receipt totals, completion, notification queueing and settings; frontend build and focused lint checks validate the receiving entry points.

## Delivery sequence

| Checkpoint | Deliverable | Exit condition |
| --- | --- | --- |
| 1. Schema and branches | Forward migration, constraints, branch API/settings | Existing data preserved; branch permissions and lifecycle verified |
| 2. Order classification | PO/WSO create, detail, correction and audit | Both order types save valid metadata and preserve normal workflows |
| 3. Follow-up view | Unified API, Paid Orders page, filters and navigation | Combined pagination and filtering are correct; no duplicated orders |
| 4. Regression and handover | Integration checks, UI walkthrough, documentation | Acceptance scenarios pass and deployment steps are documented |

Deploy the backward-compatible backend migration/API before the frontend that sends new fields. Configure actual branches before enabling paid-order entry for the team. No existing order is automatically reclassified. If rollback is needed, revert the frontend first and preserve the added data/schema rather than dropping paid-order records.

## Acceptance and validation

- Create a bulk PO and WSO with the checkbox off; existing behaviour and payload compatibility remain intact.
- Create MTM and stock-out orders for each source type; required branch/reason validation works in both UI and API.
- Reject missing/inactive branches and invalid combinations; database constraints also reject invalid direct writes.
- Branch settings require admin access. Duplicate codes are rejected. Deactivation preserves historical assignments and filters.
- Correct classification/branch/reason with an audit trail; ordinary edits preserve metadata. Unchecking clears dependent fields atomically.
- Paid Orders shows each order once, opens the correct detail page despite overlapping PO/WSO IDs, and accurately filters across both sources before pagination.
- Category filtering works with multi-item orders and does not inflate quantities or counts. Empty filter results and inactive historical branches behave correctly.
- Cancellation, reactivation, partial receiving, defects and completion continue to follow existing rules and refresh the paid-order view.
- Existing lists default to Bulk with an explicit way to view All/Paid; dashboards retain their documented all-order scope.
- Migrate a disposable database containing existing orders and verify unchanged historical quantities/statuses and bulk defaults. Run backend integration tests against disposable PostgreSQL, frontend type/build checks, and a manual create-to-follow-up walkthrough.
- Document branch configuration, the meaning of Paid order, correction rules, filter behaviour, and the difference between order reason and product category in the training guide.

## Agreed implementation defaults

The recommended defaults are one branch per order and one reason per item, two fixed reasons, bulk-only ordinary list defaults, all-order dashboard totals, oldest-open-first follow-up, and editable classification with audit history. These defaults are implemented. Mixed-reason orders are explicitly supported. Reason and category filters must match the same item, list its parent once, and distinguish matching-item quantities from whole-order totals. Do not overload product category to represent reason.
