# Paid orders

Paid orders identify customer requests that need branch follow-up. They use the existing workshop or purchasing workflow; this label does not record or verify a payment.

## Configure branches

An administrator opens Settings → Branches and creates a branch with a unique code and name. Codes remain fixed reporting identifiers. Names and Active status can be changed. Deactivation prevents new assignments but preserves existing orders, their branch names, and historical filters. Existing assignments to inactive branches can remain when other order data is edited.

## Create and follow up

1. Create a WSO or PO normally and check **Paid order**.
2. Choose the branch once at order level.
3. Select **Make to measure (MTM)** or **Stock-out** on each item. A single order can contain both reasons: for example, MTM trousers and a stock-out fleece.
4. Enter categories, sizes, quantities, branding, and other information as usual. Save opens the existing detail page.
5. Use **Paid Orders** in the sidebar for follow-up. Open an order to manage its normal production, receipt, defect, cancellation or reactivation actions.

The page defaults to open orders, oldest first. Filters include branch, item reason, product category, order type, status, reference and creation dates. Dates use Nairobi boundaries. Inactive branches remain available in filters. Filters are saved in the URL, and results refresh every minute or with Refresh.

An order appears once even if several items match. Reason and category filters must match the same item: filtering MTM + Fleece will not match an order whose only MTM item is trousers. Displayed quantities cover the **matching items**, not necessarily the full order. WSO fulfilment means received quantity; PO fulfilment means accepted quantity after non-accepted defects. Order status remains the status of the whole order.

Ordinary WSO/PO lists default to Bulk and offer Bulk / Paid / All. Dashboards retain all-order totals and say so explicitly. Product-category drill-downs on the existing WSO list remain cross-order category views; use Paid Orders for combined paid/branch/category filtering.

## Correct classification

Open an existing order and choose **Edit classification**. Select its paid/bulk classification, branch, and each item's reason together. Save updates the metadata atomically and records old/new values, actor and timestamp in `paid_order_audit`. Switching to Bulk clears branch and all reasons. Cancelled orders must be reactivated before classification can be corrected. If the item list changed during editing, refresh before saving again.

The dedicated endpoint is `PUT /order-classification/{wso|po}/{id}`. Send `is_paid_order`, `branch_id`, and all current items as `{id, paid_order_reason}` exactly once. Ordinary header/item update endpoints preserve this metadata. New standalone PO items require a reason when their parent is paid. Branch administration is restricted to administrators on the server; authenticated users can read branch choices and paid-order data.

## Migration and verification

Run from the backend directory before starting the updated application:

```powershell
sqlx migrate info
sqlx migrate run
sqlx migrate info
```

`202610260001_paid_orders.sql` adds branches, header classification, item reasons, integrity/audit triggers, indexes, and the combined item reporting view. Existing orders default to Bulk. It does not reset business data or invent branch names.

Migration files use LF line endings enforced by `.gitattributes`. SQLx checks exact file bytes: a Windows CRLF conversion can cause a checksum mismatch even when the SQL text is unchanged. Do not edit database migration checksums or bypass validation; compare file hashes and restore the originally applied bytes. On 27 October 2026, the six earlier migrations were restored to their exact applied LF checksums and the paid-orders migration was applied to the configured application database successfully.

Automated validation uses disposable PostgreSQL:

```powershell
# Use a disposable server connection, never the application database.
$env:DATABASE_URL = 'postgres://TEST_USER@127.0.0.1:TEST_PORT/postgres'
cargo test --locked --test baseline --test po_email --test paid_orders -- --include-ignored
```

Tests cover mixed reasons in both order types, same-item category/reason filters, branch lifecycle, audit and atomic correction, invalid writes, standalone PO item validation, receipt/defect status, cancellation/reactivation, Nairobi date boundaries, pagination and upgrading existing records. Frontend verification uses `npm run build`; the browser walkthrough created a test branch and mixed-reason paid WSO and verified its follow-up listing against a separate test database.

## Reporting limits

Branch IDs, item reasons, categories, creation dates and classification audit history provide reporting dimensions now. Counts across reasons/categories are not additive when one order has several kinds of item. Later reports must distinguish distinct orders from item quantities. Turnaround, historical backlog and SLA metrics need explicit completion/reopening history and policies; they must not infer completion from `updated_at`. This release does not implement payment capture, measurements, customer records, export/report builders, new notifications or automatic production prioritization.

Validation completed on 27 October 2026: all 17 tests across baseline, PO email and paid-order suites passed against disposable PostgreSQL, with none ignored. The final run used target/paid-validation to avoid replacing the running application executable. Frontend production build, TypeScript checks and ESLint on the new feature files passed. Existing Rust unused/dead-code and frontend bundle-size warnings remain.
