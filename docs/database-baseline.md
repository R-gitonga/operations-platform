# Database baseline (15 October 2026)

## Scope and historical audit

This is a **fresh-install migration-history replacement**, not an upgrade for an
existing database. The backend directory in this checkout is
`operations-platform-backend/`; its `migrations/` is SQLx's active source.
The 43 original SQL files remain unchanged in `sql/`, now explicitly an archive.

The audit used the SQL history, Rust models, repository queries, services,
handlers/routes, startup and notification worker, business-rule documentation,
and frontend TypeScript contracts. No prior integration fixtures existed.

| Historical scripts | Finding and baseline decision |
| --- | --- |
| 000–003 | Overlapping definitions of orders, lines and categories; incompatible `quantity`/`qty_raised` layouts. Define each final table once. |
| 004, 012, 013, 036, 039 | Incremental attachment, stage configuration, item status, event module, audit and sender additions now belong in their table definitions. |
| 010–011, 041 | Item history inserts never supply `wso_id`; 011 left that legacy column required. The baseline has only required `wso_item_id`. Header category/design/fabric/current-stage fields are obsolete and omitted. |
| Both 017 files | Duplicate SQLx version; preserve both reference-data requirements under one uniquely versioned seed migration. |
| 014, 019 | Destructive operational resets, excluded entirely from active migrations. |
| 007, 016, 017 | Environment-specific email recipients are omitted. Administrators configure recipients after installation. |
| 018, 032 | Preserve case-insensitive email uniqueness and hashed reset-token persistence. Roles are nonempty text: current Rust validation allows more than `admin`/`user`. No invented roles/permissions tables. |
| 020–021, 037 | Enforce singleton settings (`id = 1`); retain one unresolved partial-receiving period per item. Remove the duplicate active-period index. |
| 023–026 | Preserve configurable branding and receipt-event history; no historical branding seeds exist, so none are invented. |
| 027–035 | Preserve supplier/PO hierarchy, append-only delivery ledger, defect lifecycle and computed fulfilment. Add positive ordered quantities and one line per receipt. |
| 040 | Event timestamps are created as TIMESTAMPTZ from the outset; no timezone conversion or historical data rewrite is needed. |

Additional findings: standalone WSO line creation omitted a still-required order
ID; stage-history reads bound one parameter twice; notification-log updates
targeted jobs (including an extra retry increment); a legacy enqueue helper
omitted sender name. Some dashboard/attention DTOs require strings even though
item descriptions/design/fabric are nullable. These have small repository fixes.

## Final schema

There are 29 application tables, plus SQLx's own `_sqlx_migrations`.

| Domain | Tables and relationships |
| --- | --- |
| Accounts | `users` → `password_reset_tokens`; account role and active flag live on users; audit actor names remain text snapshots. |
| Shared reference data | `categories`, `production_stages`, `branding_types`, `branding_locations`. |
| WSO | `wso_orders` → `wso_items` → `wso_line_items`; items link to category/current stage, `wso_stage_history`, and `wso_item_branding`. |
| WSO receiving | `wso_partial_receipt_events` references the exact line/item pair; `partial_receiving_tracking` holds unresolved periods; `partial_receiving_settings` holds the threshold. |
| Notifications | `notification_events` → `notification_settings`, `notification_recipients`, `notification_logs` → `notification_jobs`; `attention_required_notifications` deduplicates item/stage/start-time occurrences. |
| Purchasing | `suppliers` → `purchase_orders` → `po_items` → `po_line_items`; items also have `po_item_notes`; orders have `po_receipts` → `po_receipt_lines`; lines have `po_defects`; `po_settings` is a singleton. |

Decisions:

- WSO stage history is strictly item-based. Durations are calculated from history
  timestamps; no extra duration table or redundant order link is introduced.
- `wso_line_items.wso_order_id` remains because the API and dashboard aggregates
  use it. A composite FK enforces that it agrees with the owning item's order.
  Standalone inserts derive it from the item. Receipt events similarly enforce
  their line/item pair. Neither relationship can drift silently.
- Required creation timestamps have `NOT NULL DEFAULT now()`. Event timestamps
  map to `DateTime<Utc>`; optional event timestamps remain nullable. Existing
  optional Rust WSO creation/update fields can decode required database values;
  the API's optional field types are retained. Business dates remain `DATE`.
- Nullable item text is preserved. Required string projections use `COALESCE`
  instead of changing API response types or forbidding valid null values.
- Preserve restrictive WSO line/history/receipt relationships to avoid deleting
  operational history through existing delete endpoints. Owned configuration and
  tracking relationships retain their historical cascade behavior. Category
  deletion clears optional item categories. Supplier deletion is restricted.
- PO status stores only `active`/`cancelled`; fulfilment is derived from receipts
  minus non-accepted defects. No mutable PO received counter is added. PO receipt
  ownership across orders and aggregate quantity limits remain service rules;
  this baseline does not add cross-table triggers or redesign concurrency.
- Indexes support parent joins, latest stage per item, pending jobs, active reset
  tokens, categories, dates and status filters. Unique constraints already cover
  order references and branding triples; duplicate indexes are omitted. Stage
  code uniqueness is case-insensitive, matching repository lookup behavior.
- `updated_at` is maintained by existing repository updates, not a new trigger.
- Existing PO event definitions and settings are retained, even though current
  code does not implement all PO alert dispatches/sweeps or templates. The only
  startup worker processes notification jobs every ten seconds. Failed jobs have
  an attempt count but no automatic retry scheduling. No new features are implied.

## Migration files and reference data

1. `202610150001_reference_and_users.sql`: shared configuration and accounts.
2. `202610150002_wso.sql`: orders, products, stages, branding and receiving.
3. `202610150003_notifications.sql`: events, delivery persistence and tracking.
4. `202610150004_purchasing.sql`: suppliers, purchasing, delivery ledger and defects.
5. `202610150005_reference_data.sql`: five categories, eleven stages (including
   `NOT_STARTED`, 24 hours), eight WSO events, ten PO events, a settings row for
   every event, and both singleton settings rows.

Other production stages retain their historical null duration. All stage active
and attention flags default true. Notification enabled/email/in-app defaults are
true, partial attention is 3 days, and PO thresholds are 0/2/3/3 days with a
60-minute sweep setting. Inserts use conflict-do-nothing; they do not overwrite
administrator settings. No users, suppliers, orders, recipients, credentials or
environment-specific sender values are seeded.

## Fresh installation

Run from the backend root. Install PostgreSQL and a Rust toolchain first. If SQLx
CLI is missing, install a version compatible with the project's SQLx 0.8 series:

```powershell
cargo install sqlx-cli --version '~0.8' --locked --no-default-features --features postgres,rustls
Set-Location 'C:\Users\USER\Operations Platform\operations-platform-backend'
$env:DATABASE_URL = 'postgres://DB_USER:URL_ENCODED_PASSWORD@localhost:5432/wso_tracker_new'
sqlx database create
sqlx migrate run
sqlx migrate info
cargo check --locked
```

Replace the example username/password with your own role credentials and choose
a **new database name**. The role needs database creation permission for the first
command and ownership/schema privileges for migrations. Keep the URL in your
process environment or untracked local configuration; never commit credentials.
SQLx creates and manages `_sqlx_migrations`; never populate it manually.

Configure JWT, SMTP and system notification environment variables as documented
in the main README, then run `cargo run`. Startup connects to the database but
does not automatically migrate it. Use the existing `/auth/bootstrap-admin`
flow with your own credentials while the users table is empty; no SQL seed user
is needed. Configure branding choices and email recipients through Settings.
Restrict access during first-admin bootstrap. No SMTP messages are sent by the
migrations themselves.

From outside the backend root, provide `--source` with the absolute path to this
backend's `migrations` directory. Do not run the historical `sql/` files.

## Existing installation

Preserve the database and its application configuration. Do **not** point this
baseline at an existing manually initialized or populated database, even if it
lacks `_sqlx_migrations`. Do not fake migration rows, rename old versions, or
edit checksums. Back up the database and verify restoration first.

Choose an explicitly reviewed transition: either retain the old installation
while planning a schema/data adoption migration against a restored copy, or
authorize a separate fresh database and a tested data transfer/reset plan.
Compare catalog/schema differences, validate timezone interpretation and orphan
records, preserve identifiers and audit history, and rehearse the cutover before
changing the application's URL. This task provides no automatic adoption,
truncation, drop or populated-database reset procedure.

## Validation and repeatable checks

See `database-validation.md` for this run's results. Integration tests are ignored
by default because they require PostgreSQL. To run them, explicitly point
`DATABASE_URL` at a disposable server with CREATE DATABASE privileges:

```powershell
$env:DATABASE_URL = 'postgres://migration_test@127.0.0.1:55439/wso_baseline_final'
cargo test --locked --test baseline -- --ignored --nocapture
```

SQLx creates isolated test databases and applies the active migrations. Tests
call the real services/repositories and never start the SMTP worker. Do not use
production database credentials for tests. `cargo check` alone cannot verify
runtime `query_as` SQL, so fresh-database tests are essential.

## Reusable migration conventions

- Use a single active `migrations/` source with unique, increasing numeric
  versions and descriptive snake_case filenames (`sqlx migrate add description`).
- Once published/applied, never edit a migration: add a forward migration.
  This one-time development baseline reset is an explicitly scoped exception.
- Order by dependencies; let SQLx handle transactions, checksums and tracking.
  Keep database creation and operational resets outside migrations.
- Match SQL types/nullability to Rust decoding and API semantics. Prefer
  TIMESTAMPTZ for instants and DATE for business dates.
- Encode local invariants and ownership in meaningful keys/checks, use deliberate
  deletion policies, and index actual query paths without duplicating key indexes.
- Separate required reference seeds from test fixtures and environment settings.
- Validate empty installation, second-run no-op, catalog constraints, Rust build
  and real service workflows on disposable PostgreSQL before shipping.

## Git change summary

Created five active SQL migrations, this design/install guide, the validation
report, `sql/README.md`, and `tests/baseline.rs`. Updated the main README and six
repository files: line item, stage history, notification log, dashboard,
production stage and partial-receiving attention. Original SQL remains in place
as an explicitly inactive archive; no original scripts were deleted or rewritten.
Pre-existing auth/user/module edits, `rust-toolchain.toml`, and untracked
039–041 scripts were preserved. No commit, history rewrite or push was performed.
