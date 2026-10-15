# Baseline validation — 15 October 2026

## Environment and isolation

Windows; PostgreSQL 18.6; installed SQLx CLI 0.9.0; application SQLx 0.8;
Cargo 1.99.0. All database work used a newly initialized cluster under the
backend's ignored `target/migration-test-pg`, with role `migration_test` and
loopback port 55439. No existing application database or `.env` credentials
were used. Port 55432 was occupied and left untouched. Windows sandbox server
startup failed; an approved elevated tool execution started the isolated cluster.

## Commands and results

Commands ran from `operations-platform-backend/`:

```powershell
initdb -D target/migration-test-pg -U migration_test -A trust --encoding=UTF8 --locale=C
pg_ctl -D target/migration-test-pg -l target/migration-test-pg/server.log -o '-h 127.0.0.1 -p 55439' -w start
$env:DATABASE_URL = 'postgres://migration_test@127.0.0.1:55439/wso_baseline_final'
sqlx database create
sqlx migrate run
sqlx migrate run
cargo test --locked --test baseline -- --ignored --nocapture
cargo check --locked
git diff --check
```

| Check | Result |
| --- | --- |
| Fresh database creation | Passed. |
| First migration run | All five final migrations applied successfully. |
| Second migration run | Success, no migrations reapplied. |
| SQLx tracking | Five successful versions, 202610150001–202610150005. No fabricated tracking entries. |
| Catalog inspection via psql and schema-only pg_dump | 29 application tables plus `_sqlx_migrations`; 30 foreign keys, 29 CHECK constraints, 13 UNIQUE constraints (plus unique indexes); no timestamp-without-time-zone columns. Counts include the SQLx table where applicable. |
| `baseline_workflows` | Passed against a SQLx-created isolated test database. |
| `repository_sql_prepares` | Passed: 123 standalone raw repository SQL statements/fragments prepared on PostgreSQL. |
| `cargo check --locked` | Passed with 34 existing unused/dead-code warnings; no compile errors. |
| `git diff --check` | Passed; Git emitted line-ending conversion notices only. |

The first draft was also applied to a separate `wso_baseline_validation`
database. After removing a redundant stage-code index, the final files were
validated from scratch on `wso_baseline_final`, rather than editing tracking
checksums or applying a changed history over that first database.

## Workflow assertions

The integration test calls actual services/repositories and verifies:

- First-admin bootstrap, correct/incorrect password authentication, hashed reset
  token consumption, password update and rejection of reused tokens.
- Required event/settings seeds and singleton defaults.
- WSO header/item/size creation, initial `NOT_STARTED` assignment, item-only stage
  history insert/read, subsequent stage change, standalone size-line creation,
  branding relationships and quantities.
- Partial WSO receiving, balance and tracking creation, merged receipt/stage
  timeline, full receiving, tracking resolution and order completion.
- Nullable item text decoding in stage lists, dashboards and attention views;
  overdue stage calculation and duplicate notification occurrence suppression.
- Supplier creation/update, complete PO creation, two partial delivery records,
  defect reporting, outstanding quantity, defect acceptance, computed completion,
  item notes, procurement timeline and PO dashboard/overdue queries.
- Notification job sender fields, failed-job attempt count, log failure/success
  updates and isolation between log IDs and job updates. No email is sent.
- Rejection of invalid singleton IDs, over-received WSO quantities and mismatched
  item/order links; absence of `wso_stage_history.wso_id` and naive timestamps.

The source-query test prepares standalone raw SQL directly from repository files;
it also wraps activity subqueries and completes the PO status aggregate. It skips
format placeholders/column fragments. Dynamic queries are reviewed against the
schema and representative paths run in the workflow test; this is not a claim
that every repository branch or every possible data state was executed.

## Limits and remaining application behavior

No blockers remain for the fresh-database baseline. Browser/UI flows, HTTP cookie
handling, live SMTP delivery, load/concurrency tests and migration adoption of an
existing populated database were not executed. Tests exercise Rust services and
repositories directly, not the HTTP server. No SMTP credentials are required.

Existing PO alert settings/events are preserved, but the application does not
currently run all corresponding sweeps/templates. Existing cross-record receiving
validation and transaction/concurrency behavior are unchanged. Those are separate
application concerns, not capabilities added by these migrations.

Local evidence is retained in ignored `target/baseline-test.log`,
`target/baseline-check.log`, `target/baseline-schema.sql`, and the isolated server
log. The test server is stopped after validation; its disposable cluster remains
under `target/` for inspection. SQLx cleans up successful per-test databases.
