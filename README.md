# Operations Platform

### WSO Tracker Backend

Operations Platform manages workshop production and external procurement in one operational workflow. Teams can raise Workshop Orders (WSOs), track each product through production, record partial deliveries, manage Purchase Orders (POs), and investigate outstanding quantities and defects.

This repository contains the Rust HTTP API, PostgreSQL migrations, and backend integration tests. The [React frontend](https://github.com/R-gitonga/operations-platform-frontend) is maintained separately.

## Contents

- [Capabilities](#capabilities)
- [Architecture and data model](#architecture-and-data-model)
- [Prerequisites](#prerequisites)
- [Local setup](#local-setup)
- [Configuration reference](#configuration-reference)
- [API overview](#api-overview)
- [Development and testing](#development-and-testing)
- [Database changes](#database-changes)
- [Deployment considerations](#deployment-considerations)
- [Troubleshooting](#troubleshooting)
- [Further documentation](#further-documentation)

## Capabilities

| Area | Implemented functionality |
| --- | --- |
| Accounts | First-admin setup, password login, JWT cookies, active/inactive accounts, administrative user management, and password reset tokens. |
| Workshop orders | Multiple products per WSO, size/quantity lines, attachments, cancellation, reactivation, and receiving-based completion. |
| Production | Configurable stages, initial `NOT_STARTED` assignment, item-level stage history, elapsed time, and attention-required views. |
| Receiving | Partial and full WSO receipts, outstanding balances, receipt history, and configurable partial-receiving attention thresholds. |
| Branding | Configurable branding types and locations, with quantities attached to WSO product requirements. |
| Procurement | Suppliers, PO products and size lines, delivery receipts, defects and resolutions, item notes, attachments, and procurement timelines. |
| Dashboards | WSO production summaries, recent activity, outstanding quantities, PO fulfilment summaries, and overdue delivery views. |
| Notifications | Event settings, recipients, email jobs, delivery logs, and WSO attention-notification tracking. |

The application is under active development. PO notification event definitions and thresholds exist, but not all PO notification templates and alert sweeps are implemented. The email worker processes pending jobs every ten seconds; failed jobs record an attempt count but are not automatically scheduled for retry. In-app notification settings exist without a complete in-app delivery workflow.

## Architecture and data model

```text
HTTP request
    → Axum routes
    → Handlers: request extraction, authentication and responses
    → Services: business rules and workflow coordination
    → Repositories: SQLx queries and persistence
    → PostgreSQL
```

The backend uses Rust, Axum 0.8, Tokio, SQLx 0.8, PostgreSQL, Serde, Chrono, Argon2, JSON Web Tokens, and Lettre for SMTP email. Dependency versions are recorded in `Cargo.toml` and `Cargo.lock`.

Two order hierarchies underpin the system:

```text
Workshop Order → WSO Item → Size/quantity line
                     ├── Production-stage history
                     ├── Branding requirements
                     └── Receiving events and attention tracking

Supplier → Purchase Order → PO Item → Size/quantity line
                              │               ├── Receipt lines
                              │               └── Defects
                              └── Item notes
```

Production stages belong to individual WSO items. WSO lines store raised and received quantities, with balance calculated as their difference. PO receiving uses a delivery ledger: delivered, accepted, defective, and outstanding quantities are derived from receipt and defect records. PO completion is a calculated display status; the stored order status is `active` or `cancelled`.

Event timestamps use PostgreSQL `TIMESTAMPTZ` and Rust `DateTime<Utc>`. Date-only business values use `DATE`.

```text
.
├── src/
│   ├── main.rs              # API startup, route registration and email worker
│   ├── config.rs            # Environment configuration and validation
│   ├── authenticated_user.rs # Cookie authentication and active-account lookup
│   ├── routes/              # HTTP endpoint definitions
│   ├── handlers/            # HTTP request/response handling
│   ├── services/            # Business workflows and email templates
│   ├── repositories/        # SQLx database queries
│   ├── models/              # Database projections and API payloads
│   ├── database/            # Database pool type
│   └── errors/              # Shared application errors
├── migrations/              # Official SQLx migration history
├── tests/baseline.rs        # PostgreSQL-backed workflow and SQL checks
├── docs/                    # Architecture, schema and business-rule documentation
├── sql/                     # Historical SQL archive — do not execute
├── uploads/                 # Local attachment storage
├── Cargo.toml
├── Cargo.lock
└── rust-toolchain.toml      # Pinned Rust toolchain
```

## Prerequisites

- Git and access to this repository.
- Rust installed through rustup. `rust-toolchain.toml` currently selects **1.96.1**.
- Native build tools for your Rust target. On Windows, use the MSVC C++ build tools and Windows SDK. On Linux, the native TLS dependency may require OpenSSL development packages and `pkg-config`.
- A running PostgreSQL server and a database role with the necessary permissions. The baseline was validated on PostgreSQL **18.6**; other versions have not been validated in this project.
- SQLx CLI for database creation and migrations.
- SMTP credentials for actual email delivery.
- For the optional frontend: Node.js matching its Vite requirement (`^20.19.0` or `>=22.12.0`) and npm.

## Local setup

The commands below use PowerShell and run from the backend repository root unless stated otherwise. On Bash, use `export NAME='value'` instead of `$env:NAME = 'value'`.

### 1. Clone and install the migration CLI

```powershell
git clone https://github.com/R-gitonga/operations-platform.git
cd operations-platform
rustup show
cargo install sqlx-cli --version '~0.8' --locked --no-default-features --features postgres,rustls
```

The remote repository is named `operations-platform`; it contains the backend directly. There is no additional `backend/` directory to enter after cloning.

### 2. Configure the backend

Supply the following variables in the terminal that will run the backend. Replace all placeholders with your local configuration. Use a new database name for a fresh installation and URL-encode special characters in the database password.

```powershell
$env:DATABASE_URL = 'postgres://DB_USER:URL_ENCODED_PASSWORD@localhost:5432/wso_tracker_new'
$env:JWT_SECRET = 'REPLACE_WITH_A_LONG_RANDOM_SECRET'
$env:JWT_EXPIRATION_HOURS = '8'
$env:SMTP_HOST = 'YOUR_SMTP_HOST'
$env:SMTP_PORT = '587'
$env:SMTP_USERNAME = 'YOUR_SMTP_USERNAME'
$env:SMTP_PASSWORD = 'YOUR_SMTP_PASSWORD'
$env:SMTP_FROM = 'sender@example.com'
$env:SYSTEM_NOTIFICATION_NAME = 'Operations Platform'
$env:SYSTEM_NOTIFICATION_EMAIL = 'sender@example.com'
$env:FRONTEND_BASE_URL = 'http://localhost:5173'
```

The application also loads a `.env` file through dotenvy. Explicit process environment variables take precedence. Local `.env` files and their variants are excluded by `.gitignore`; sanitized `.env.example`, `.env.sample`, and `.env.template` files remain trackable. Keep credentials out of commits. Use process environment variables for the setup above, or create a local `.env` file with the same variable names.

SMTP fields are required at startup even when no recipients are configured. For development without email delivery, nonempty local placeholder values allow startup, but delivery and password-reset emails will not work. No notification recipients are seeded. Keep recipients empty until a working development SMTP destination is configured.

### 3. Initialize a fresh database

Ensure PostgreSQL is running. The role in `DATABASE_URL` needs `CREATEDB` permission for database creation and ownership/schema privileges for migrations.

```powershell
sqlx database create
sqlx migrate run
sqlx migrate info
```

The five migrations create all 29 application tables and seed categories, production stages, notification events/settings, and receiving/procurement defaults. SQLx manages its own `_sqlx_migrations` table. Running `sqlx migrate run` again leaves applied migrations unchanged.

No sample users, suppliers, orders, email recipients, or credentials are seeded. Branding types and locations are configured through the application.

**Existing database:** The current baseline replaces experimental migration history and is intended for fresh databases. Do not run it over a populated or manually initialized installation. Preserve that database and follow the [existing-installation guidance](docs/database-baseline.md#existing-installation). Do not run scripts from `sql/` or fabricate SQLx migration records.

### 4. Build and start the API

```powershell
cargo check --locked
cargo run --locked
```

The API listens on `0.0.0.0:3000`. The bind address and port are currently defined in `src/main.rs`, rather than environment variables. Startup connects to PostgreSQL but does not apply migrations automatically.

In a second terminal:

```powershell
Invoke-RestMethod http://localhost:3000/
```

Expected response:

```json
{"message":"WSO Tracker API"}
```

This root response confirms that the HTTP server is reachable; it is not a comprehensive database readiness check.

### 5. Start the frontend and create an administrator

In a separate terminal, clone the frontend into a sibling directory:

```powershell
cd ..
git clone https://github.com/R-gitonga/operations-platform-frontend.git
cd operations-platform-frontend
npm ci
npm run dev
```

Open the address printed by Vite, normally `http://localhost:5173`. The frontend defaults to `/api`; its development proxy forwards that prefix to `http://localhost:3000` and removes the prefix. This allows browser requests and authentication cookies to use the frontend origin. `VITE_API_BASE_URL` overrides the API base URL when needed.

Open `/setup` in the frontend and create the first administrator. The backend permits bootstrap only while the users table is empty. Alternatively, call the API directly using your own credentials:

```powershell
$admin = @{
    name = 'YOUR_NAME'
    email = 'YOUR_EMAIL'
    password = 'YOUR_CHOSEN_PASSWORD'
} | ConvertTo-Json
Invoke-RestMethod -Method Post -Uri http://localhost:3000/auth/bootstrap-admin -ContentType 'application/json' -Body $admin
```

Keep the installation accessible only to the setup operator until bootstrap is complete. Sign in, review the seeded categories and stages, configure branding choices, and add suppliers as needed. Add notification recipients only after checking email configuration.

## Configuration reference

| Variable | Required / default | Purpose |
| --- | --- | --- |
| `DATABASE_URL` | Required | PostgreSQL connection URL used by the API and SQLx CLI. |
| `JWT_SECRET` | Required, nonempty | Signs and validates authentication tokens. |
| `JWT_EXPIRATION_HOURS` | `8`; must be positive | Token lifetime. |
| `SMTP_HOST` | Required, nonempty | SMTP server used by Lettre's STARTTLS transport. |
| `SMTP_PORT` | `587` | SMTP port. |
| `SMTP_USERNAME` | Required, nonempty | SMTP authentication username. |
| `SMTP_PASSWORD` | Required, nonempty | SMTP authentication password. |
| `SMTP_FROM` | Required, nonempty | Currently validated configuration; queued sender identity uses the system notification fields below. |
| `SYSTEM_NOTIFICATION_NAME` | Required, nonempty | Sender display name for system emails. |
| `SYSTEM_NOTIFICATION_EMAIL` | Required, nonempty | Sender email address for system emails. |
| `FRONTEND_BASE_URL` | Set explicitly | Public frontend origin for password-reset links. Configuration defaults to port 5173, but the reset service has a separate port-3000 fallback. Explicit configuration avoids that discrepancy. |

Database reference settings such as stage durations, receiving thresholds, and notification recipients belong in the database and are managed through application settings. Secrets and environment-specific connection details belong in environment configuration.

## API overview

Paths below are backend paths. The frontend development proxy adds `/api` before forwarding requests. Consult `src/routes/` and `src/models/` for the exact methods and payloads.

| Route group | Purpose |
| --- | --- |
| `/auth/login`, `/auth/logout`, `/auth/me` | Login, logout and current user. |
| `/auth/bootstrap-admin`, `/auth/forgot-password`, `/auth/reset-password` | Initial account setup and password recovery. |
| `/users` | Administrative user management. |
| `/wso`, `/wso/{id}`, `/wso/summary` | WSO creation, details, updates and totals. |
| `/wso-items/{id}/line-items`, `/line-items/{id}/receive` | WSO size lines and receiving. |
| `/production-stages`, `/wso-items/{id}/stage`, `/wso-items/{id}/stage-history` | Stage configuration, transitions and history. |
| `/categories`, `/branding/types`, `/branding/locations` | Shared reference data. |
| `/wso-items/{wso_item_id}/branding` | WSO product branding requirements. |
| `/dashboard`, `/dashboard/attention-required`, `/partial-receiving` | WSO operational views and receiving settings. |
| `/suppliers`, `/purchase-orders`, `/po-items`, `/po-line-items` | Procurement records and product lines. |
| `/purchase-orders/{id}/receipts`, `/po-line-items/{id}/defects`, `/po-defects/{id}/resolve` | Deliveries and quality handling. |
| `/purchase-orders/dashboard`, `/po-items/{id}/procurement-timeline`, `/po-settings` | Procurement reporting, history and thresholds. |
| `/settings/notifications` | Notification settings, events and recipients. |
| `/uploads` | Files served from local attachment storage. |

Protected handlers read the `auth_token` HTTP-only cookie, validate the token, and load the active user. Administrative checks are implemented in individual handlers; the database stores the user's role as text. API clients must retain cookies between login and subsequent requests.

## Development and testing

```powershell
cargo check --locked
cargo build --locked
cargo test --locked
```

Database integration tests are **ignored by default**. Run them explicitly against a disposable PostgreSQL server with a role permitted to create test databases. Replace the example connection below with your test server's credentials; never use a production connection.

```powershell
$env:DATABASE_URL = 'postgres://TEST_USER:URL_ENCODED_PASSWORD@localhost:5432/wso_tracker_test'
sqlx database create
cargo test --locked --test baseline -- --ignored --nocapture
```

SQLx creates isolated test databases and applies the migrations. The tests cover authentication/password reset, WSO creation and item-stage history, branding, partial receiving, PO receipts and defects, notification persistence, database constraints, and preparation of standalone repository SQL. They do not start the email worker or send SMTP messages.

Restore `DATABASE_URL` to your development database before starting the API again. The [baseline validation report](docs/database-validation.md) records the tested environment, results and coverage limits. Runtime SQL queries need database tests; a successful `cargo check` alone does not prove schema compatibility.

For frontend checks, run `npm run build` and `npm run lint` from the separate frontend checkout.

## Database changes

Use `migrations/` as the only active schema history:

```powershell
sqlx migrate add describe_the_change
sqlx migrate run
```

Review the generated version for uniqueness and dependency order. Once a migration is shared or applied, add a new forward migration instead of editing its contents. Keep required reference data separate from test fixtures and secrets. Validate changes on an empty disposable database and rerun the migration command to confirm it is a no-op.

The [database design and migration guide](docs/database-baseline.md) explains relationships, deletion policies, reference data, timestamp conventions, and safe handling of older installations.

## Deployment considerations

Run migrations as an explicit deployment step before starting a compatible API build. Persist and back up both PostgreSQL and `uploads/`; attachment files are stored on disk, with paths in the database.

The current server configuration is designed around local development. Before exposing an installation, review the actual deployment requirements, including:

- HTTPS and cookie security: login currently sets `Secure=false`.
- Access to attachments: `/uploads` is mounted as a static file service.
- Routing: the backend does not configure CORS middleware; production browser access needs appropriate same-origin proxying or an explicitly implemented cross-origin policy.
- First-admin bootstrap, handler authorization, and the debug notification endpoint.
- Email delivery monitoring and recovery of failed jobs.
- Environment-secret management and rotation of any credentials previously committed to repository history.

The Vite development proxy is not included in a production frontend build. Configure equivalent API routing in the production web server. This repository does not currently provide a complete deployment or container orchestration configuration.

## Troubleshooting

| Symptom | Check |
| --- | --- |
| Startup reports a missing configuration value | Set every required variable, including SMTP fields, in the API process's environment. |
| Database connection fails | Confirm PostgreSQL is running, the URL/port is correct, credentials are URL-encoded, and the role has access. |
| A table or column is missing | Confirm the API and SQLx CLI use the same `DATABASE_URL`, then inspect `sqlx migrate info`. |
| Migration reports an existing relation or checksum mismatch | Stop and determine whether this is an older installation or an edited migration. Do not delete data or alter SQLx tracking to bypass the error. |
| Bootstrap is forbidden | The database already contains a user; use an existing administrator account. |
| Browser requests return 401 | Sign in, retain cookies, and confirm the account is active. Check the frontend proxy/base URL. |
| No email arrives | Check recipients, event settings, SMTP configuration, and notification jobs/logs. Failed jobs are not retried automatically. |
| Reset link opens the wrong application | Set `FRONTEND_BASE_URL` explicitly to the frontend origin. |
| Port 3000 is unavailable | Stop the conflicting local process or deliberately update the backend bind address and frontend proxy together. |
| Integration tests are skipped | Provide a disposable test database connection and run with `-- --ignored --nocapture`. |

## Further documentation

- [Database baseline, schema audit and installation](docs/database-baseline.md)
- [Database validation report](docs/database-validation.md)
- [WSO business rules](docs/business_rules/business_rules.md)
- [Repository pattern](docs/architecture/002_repository_pattern.md)
- [Error handling](docs/architecture/003_error_handling.md)
- [Notification module](docs/architecture/notification-module.md)

Some architecture documents describe earlier development phases. Use current Rust routes, models, services, and active migrations as the implementation reference when older documentation differs.
