# Operations Platform
## WSO Tracker Backend

The **Operations Platform** is a web-based workshop-order management system designed to support the end-to-end lifecycle of Workshop Orders (WSOs), from creation and production tracking through receiving, completion, cancellation, notifications, and operational monitoring.

The system is built around business workflows rather than individual database operations. A Workshop Order may contain multiple line items, each of which can move independently through production stages while the platform maintains the associated history, quantities, notifications, and operational state.

---

# 1. System Overview

The Operations Platform provides a centralized system for managing workshop production orders and monitoring their progress.

The current system supports:

- User authentication and authorization
- Role-based access
- Workshop Order management
- Multiple line items per WSO
- Production-stage tracking
- Quantity raised and received tracking
- Product receiving
- WSO completion and cancellation
- WSO reactivation
- File attachments
- Production-stage history
- Audit and operational history
- Search and filtering
- Pagination
- Dashboard statistics
- Attention-required monitoring
- Configurable notification events
- Configurable notification recipients
- Email notifications through Microsoft 365 SMTP
- Background notification processing
- Notification delivery logging
- Notification failure tracking

The backend exposes a REST API consumed by the React frontend.

---

# 2. Technology Stack

## Backend

| Technology | Purpose |
|---|---|
| Rust | Application language |
| Axum | HTTP API framework |
| Tokio | Async runtime |
| SQLx | PostgreSQL database access |
| PostgreSQL | Primary database |
| Serde | Serialization and deserialization |
| Chrono | Date and time handling |
| dotenvy | Environment configuration |
| JSON Web Tokens | Authentication |
| Lettre | SMTP email delivery |
| Tower HTTP | Static file serving and HTTP middleware |

## Frontend

The frontend is maintained as a separate React application.

| Technology | Purpose |
|---|---|
| React | UI framework |
| TypeScript | Type safety |
| Vite | Development/build tooling |
| React Router | Client-side routing |
| Tailwind CSS | Styling |
| shadcn/ui | UI components |
| TanStack Query | Server-state management |
| Axios | HTTP client |
| Lucide React | Icons |

---

# 3. Architecture

The backend follows a layered architecture.

```text
HTTP Request
     │
     ▼
   Routes
     │
     ▼
  Handlers
     │
     ▼
  Services
     │
     ▼
 Repositories
     │
     ▼
 PostgreSQL
```

Supporting these layers are shared models, application state, authentication, configuration, and error handling.

### Routes

Define HTTP endpoints and connect them to handlers.

### Handlers

Responsible for HTTP concerns such as:

- Extracting request data
- Authentication
- Calling services
- Returning HTTP responses

Handlers should not contain database queries or business logic.

### Services

Contain business workflows and coordinate multiple repositories where necessary.

Examples include:

- WSO creation workflows
- Notification dispatch
- Dashboard aggregation
- Production-stage processing

### Repositories

Contain database access and SQL queries.

This keeps SQL and database concerns separated from HTTP handlers and business logic.

### Models

Contain:

- Database models
- Request structures
- Response structures
- Business-domain structures

### Application State

The application shares common state through `AppState`, including:

- PostgreSQL connection pool
- Application configuration

---

# 4. Backend Structure

```text
src/
├── app_state.rs
├── authenticated_user.rs
├── config.rs
├── main.rs
│
├── database/
│
├── errors/
│
├── handlers/
│
├── models/
│
├── repositories/
│
├── routes/
│
└── services/

sql/
└── database migrations

docs/
└── architecture/
```

### Important directories

**`handlers/`**

HTTP request handlers.

**`services/`**

Business workflows and application services.

**`repositories/`**

PostgreSQL queries and database access.

**`models/`**

Request, response, and domain structures.

**`routes/`**

HTTP route definitions.

**`errors/`**

Centralized application error handling.

**`database/`**

Database-related abstractions.

**`sql/`**

Database migration scripts.

**`docs/architecture/`**

Architecture and design documentation.

---

# 5. Core Business Workflow

The platform is designed around the following business workflow:

```text
Create Workshop Order
        │
        ▼
Add Multiple Line Items
        │
        ▼
Assign / Track Production Stage
        │
        ▼
Monitor Production Progress
        │
        ├───────────────┐
        ▼               ▼
Update Quantities    Attention Required
        │               │
        ▼               ▼
Receive Products    Notify Relevant Users
        │
        ▼
All Products Received
        │
        ▼
WSO Completed
```

A WSO can also be:

```text
Active
  │
  ├──► Cancelled
  │       │
  │       └──► Reactivated
  │
  └──► Completed
```

---

# 6. Workshop Orders

A Workshop Order represents a production request submitted to the workshop.

A WSO contains a header and one or more line items.

The system supports:

- Creating WSOs
- Viewing WSOs
- Updating WSOs
- Cancelling WSOs
- Reactivating WSOs
- Completing WSOs
- Searching WSOs
- Filtering WSOs
- Paginating WSO lists

WSO information includes business information such as:

- WSO number
- Requisition number
- Date signed
- Description
- Design code
- Fabric code
- Category
- Status

---

# 7. WSO Line Items

Each WSO may contain multiple products or line items.

Line items support:

- Product descriptions
- Design information
- Fabric information
- Quantity raised
- Quantity received
- Receiving dates
- Production stages
- Branding requirements
- Production history

The system treats line items as independent production entities while maintaining their relationship to the parent WSO.

This allows different products within the same WSO to progress through production at different rates.

---

# 8. Production Stages

Production stages represent the workshop production workflow.

Line items can be moved between stages and each movement is recorded in the production history.

The system records:

- Previous production state
- Current production stage
- Time of change
- User responsible for the change
- Associated WSO and line item

This history powers the dashboard's **Recent Production Activity** section.

---

# 9. Production Receiving

The system tracks both:

```text
Quantity Raised
Quantity Received
Outstanding Quantity
```

The dashboard aggregates these values across active production.

When all required quantities for a product have been received, the system can trigger the corresponding notification workflow.

When all relevant products within a WSO have been received, the WSO can transition to the completed state.

---

# 10. Attachments

WSOs support file attachments.

Uploaded files are exposed through the backend's uploads route.

The application currently serves uploaded files under:

```text
/uploads
```

The backend uses Axum/Tower HTTP static-file serving for this purpose.

---

# 11. Authentication and Authorization

The API uses JWT-based authentication.

Authentication includes:

- User login
- JWT token generation
- Token expiration
- Authenticated request extraction
- User identity propagation
- Role-based authorization

Protected endpoints use the authenticated-user extractor to ensure requests are associated with a valid user.

JWT configuration includes:

```env
JWT_SECRET=...
JWT_EXPIRATION_HOURS=8
```

The expiration period can be changed through environment configuration.

---

# 12. Users and Roles

The system maintains application users and supports role-based access.

Users are associated with operational actions throughout the system.

User identity may be used for:

- Authentication
- Production-stage changes
- Audit information
- Notification context
- Operational history

---

# 13. Dashboard

The dashboard provides an operational overview of workshop activity.

Current dashboard areas include:

### Production Overview

Displays:

- Total orders
- Active orders
- Completed orders
- Cancelled orders

The KPI cards are interactive and navigate to the corresponding order lists.

### Production Stages

Displays the number of items currently associated with each production stage.

Selecting a stage takes the user to the corresponding production-stage view.

### Production Progress

Displays:

- Quantity raised
- Quantity received
- Outstanding quantity
- Overall receiving progress

### Recent Production Activity

Displays paginated production-stage activity in a table.

Each record includes:

- WSO number
- Product/item description
- Production stage
- User responsible
- Date/time

Activity records remain clickable and take the user directly to the associated WSO.

### Attention Required

Displays products that have remained in production stages beyond their expected duration.

The system calculates:

- Stage start time
- Expected duration
- Elapsed time
- Overdue duration

Attention-required items can be opened directly from the dashboard.

---

# 14. Search, Filtering and Pagination

The backend supports server-side search and filtering where appropriate.

WSO lists can be filtered using business-relevant fields such as:

- WSO number
- Status

Pagination is used for larger result sets.

Dashboard recent activity is also paginated so the dashboard can retain access to the complete activity history without allowing the activity feed to dominate the page.

---

# 15. Notification System

The Operations Platform includes a configurable notification subsystem.

The notification system is designed around **events**, **settings**, **recipients**, **logs**, and **jobs**.

```text
Business Event
      │
      ▼
Notification Event
      │
      ▼
Notification Settings
      │
      ▼
Notification Recipients
      │
      ▼
Notification Log
      │
      ▼
Notification Job
      │
      ▼
Background Worker
      │
      ▼
SMTP
      │
      ▼
Recipient Email
```

---

# 16. Notification Events

Notification events define business events that can produce notifications.

Current events include:

| Code | Event |
|---|---|
| `wso_created` | Workshop Order Created |
| `wso_completed` | Workshop Order Completed |
| `wso_cancelled` | Workshop Order Cancelled |
| `wso_reactivated` | Workshop Order Reactivated |
| `attachment_uploaded` | Attachment Uploaded |
| `attention_required` | Product Attention Required |
| `product_fully_received` | Product Fully Received |

Each event has:

- Unique code
- Display name
- Description

---

# 17. Notification Settings

Each notification event has configurable settings.

Settings currently control:

- Whether the event is enabled
- Whether email notifications are enabled
- Whether in-app notifications are enabled

The settings are associated with the notification event through `notification_event_id`.

The event's `code`, display name, and description are retrieved by joining the notification settings with the notification events table.

---

# 18. Notification Recipients

Recipients are configurable from the application.

A recipient is associated with a notification event and contains:

- Display name
- Email address
- Enabled state

This allows different business events to notify different people or groups.

The system does not depend on a permanent hard-coded recipient for normal operation.

---

# 19. Email Delivery

Email notifications are delivered through SMTP using the Lettre library.

The current configuration supports Microsoft 365 SMTP.

Example configuration:

```env
SMTP_HOST=smtp.office365.com
SMTP_PORT=587
SMTP_USERNAME=sender@example.com
SMTP_PASSWORD=...
SMTP_FROM=sender@example.com

SYSTEM_NOTIFICATION_NAME=Operations Platform
SYSTEM_NOTIFICATION_EMAIL=sender@example.com
```

The sender service constructs HTML emails and submits them through the configured SMTP server.

---

# 20. Notification Jobs and Background Worker

Email delivery is asynchronous.

Rather than sending an email directly as part of the main HTTP request, the system creates a notification job.

The background worker periodically checks for pending jobs.

Current processing interval:

```text
10 seconds
```

The workflow is:

```text
Business Event
      │
      ▼
Create Notification Log
      │
      ▼
Create Notification Job
      │
      ▼
Job = pending
      │
      ▼
Background Worker
      │
      ▼
SMTP Delivery
      │
 ┌────┴─────┐
 ▼          ▼
Success    Failure
 │          │
 ▼          ▼
sent       failed
```

This prevents email delivery from unnecessarily blocking the primary business transaction.

---

# 21. Notification Reliability

Notification jobs maintain their processing state.

A job can contain:

- Status
- Attempt count
- Error message
- Creation time
- Processing time

Successful delivery results in the job being marked as:

```text
sent
```

Failed delivery results in:

```text
failed
```

The associated notification log is also updated.

This provides traceability when troubleshooting email delivery.

Pending jobs remain in the database and are therefore not lost when the application is restarted.

---

# 22. Notification Logging

Notification logs provide a record of notification activity.

Logs track information such as:

- Notification event
- Recipient
- Channel
- Status
- Error message
- Created timestamp
- Sent timestamp

This provides an operational audit trail for notification delivery.

---

# 23. Attention Required Monitoring

The attention-required feature detects products that have remained in a production stage longer than the expected duration.

The system records individual notification occurrences using the:

```text
attention_required_notifications
```

table.

A unique occurrence is identified using:

```text
wso_item_id
production_stage_id
stage_started_at
```

This prevents repeated dashboard polling from continuously generating the same notification for the same stage occurrence.

A new stage occurrence can therefore produce a new notification when the product moves into another production stage and subsequently becomes overdue.

---

# 24. Database

The application uses PostgreSQL as its primary database.

The schema is maintained through SQL migration scripts.

Major business areas represented in the database include:

- Users
- Roles
- Workshop Orders
- WSO Items
- Production Stages
- Production Stage History
- Attachments
- Notification Events
- Notification Settings
- Notification Recipients
- Notification Logs
- Notification Jobs
- Attention Required Notification Tracking

Foreign keys and cascading rules are used to maintain relationships between business entities.

---

# 25. Repository Pattern

Database access is intentionally isolated from HTTP handling.

For example:

```text
Handler
   │
   ▼
Service
   │
   ▼
Repository
   │
   ▼
SQLx
   │
   ▼
PostgreSQL
```

Repositories are responsible for:

- Executing SQL
- Mapping database records
- Performing database updates
- Handling database-level queries

Services are responsible for combining these operations into meaningful business workflows.

---

# 26. Error Handling

The backend uses a shared application error abstraction.

This provides consistent handling of:

- Validation errors
- Authentication errors
- Authorization errors
- Database errors
- Application errors

Handlers convert service failures into appropriate HTTP responses rather than exposing raw internal implementation details.

---

# 27. Configuration

Application configuration is loaded from environment variables.

A typical development `.env` contains:

```env
DATABASE_URL=postgres://postgres:password@localhost/wso_tracker

JWT_SECRET=your-secret
JWT_EXPIRATION_HOURS=8

SMTP_HOST=smtp.office365.com
SMTP_PORT=587
SMTP_USERNAME=sender@example.com
SMTP_PASSWORD=your-password
SMTP_FROM=sender@example.com

SYSTEM_NOTIFICATION_NAME=Operations Platform
SYSTEM_NOTIFICATION_EMAIL=sender@example.com
```

### Security

The `.env` file should **never be committed to source control**.

Production secrets should be supplied through a secure environment or secret-management system.

---

# 28. Running the Backend

## Requirements

Install:

- Rust
- Cargo
- PostgreSQL

Ensure PostgreSQL is running and the required database has been created.

Create the `.env` file with the required configuration.

Then run:

```bash
cargo run
```

The API starts on:

```text
http://localhost:3000
```

---

# 29. Development Checks

Compile and validate the project with:

```bash
cargo check
```

Build the project with:

```bash
cargo build
```

Run the application with:

```bash
cargo run
```

Run automated Rust tests with:

```bash
cargo test
```

---

# 30. API Surface

The API is organized into functional route groups.

Current areas include:

```text
/auth
/users
/wso
/line-items
/categories
/dashboard
/settings
/notification-recipients
/production-stage
/uploads
```

The exact endpoint implementation should be treated as the source of truth as the API continues to evolve.

Core WSO operations include:

```text
POST   /wso
GET    /wso
GET    /wso/{id}
PUT    /wso/{id}
PATCH  /wso/{id}/cancel
```

Additional endpoints support:

- WSO line items
- Categories
- Dashboard statistics
- Production stages
- Authentication
- Users
- Settings
- Notification recipients
- Attachments
- Attention-required monitoring

---

# 31. Frontend Architecture

The React frontend communicates with the backend through a dedicated API layer.

The architecture is:

```text
React Page
     │
     ▼
React Hook
     │
     ▼
API Module
     │
     ▼
Axios
     │
     ▼
Rust/Axum API
```

The frontend is organized approximately as:

```text
src/
├── api/
├── components/
├── hooks/
├── layouts/
├── pages/
├── services/
├── types/
├── app.tsx
└── main.tsx
```

This keeps API communication, server state, reusable UI components, and pages separated.

---

# 32. Frontend Pages

The current application includes the primary operational workflows around:

- Dashboard
- WSO list
- WSO details
- WSO creation
- WSO editing
- Authentication
- Settings
- Notification configuration
- Production-stage workflows

The dashboard provides the operational starting point for users and links directly into the relevant WSO and production workflows.

---

# 33. Frontend State Management

TanStack Query manages server-side state.

For example:

```text
Dashboard
    │
    ▼
useDashboard()
    │
    ▼
getDashboard()
    │
    ▼
GET /dashboard
```

This separates server data fetching from UI components and provides caching, loading states, error states, and query invalidation.

---

# 34. Development Workflow

The preferred development workflow is:

```text
1. Understand the business workflow
        ↓
2. Design/update database schema
        ↓
3. Create SQL migration
        ↓
4. Update model
        ↓
5. Update repository
        ↓
6. Update service
        ↓
7. Update handler
        ↓
8. Update route
        ↓
9. Test backend behaviour
        ↓
10. Update frontend if required
        ↓
11. Document
        ↓
12. Commit
```

Changes should be implemented incrementally.

The backend should generally be completed and verified before introducing the corresponding frontend changes.

---

# 35. Development Principles

The system follows several principles.

### Business workflows over database tables

Instead of thinking:

```text
Create WSO
Create Line Item
Update Line Item
```

the system should be understood as:

```text
Create Workshop Order
        ↓
Create Header
        ↓
Add Multiple Products
        ↓
Track Production
        ↓
Notify Relevant Users
        ↓
Receive Products
        ↓
Complete Workshop Order
```

### Separation of concerns

Database queries belong in repositories.

Business workflows belong in services.

HTTP concerns belong in handlers.

UI concerns belong in React components.

### Incremental development

Large changes should be broken into small, verifiable changes.

A typical feature should progress from:

```text
Database
→ Model
→ Repository
→ Service
→ Handler
→ Route
→ Backend Test
→ Frontend
```

---

# 36. Current MVP Status

The current system has progressed beyond the initial CRUD prototype and represents a functioning end-to-end operational MVP.

The major operational flow is working across:

```text
Authentication
       ↓
Dashboard
       ↓
Workshop Orders
       ↓
WSO Details
       ↓
Production Tracking
       ↓
Receiving
       ↓
Notifications
       ↓
Email Delivery
       ↓
Notification Logging
```

The notification system has been tested end-to-end, including real email delivery for supported business events.

The application is therefore at the stage where further development can focus increasingly on:

- UX refinement
- Workflow improvements
- Operational controls
- Reliability
- Reporting
- User feedback

rather than establishing the fundamental system architecture.

---

# 37. Future Enhancements

The following features are candidates for future versions. They are **not necessarily part of the current MVP**.

## Attention Required Acknowledgement

Introduce an acknowledgement workflow for attention-required items.

Potential workflow:

```text
Item becomes overdue
        ↓
Attention notification
        ↓
User acknowledges
        ↓
Acknowledgement recorded
        ↓
Other users can see that it is being handled
```

Potential information:

- Acknowledged by
- Acknowledged at
- Acknowledgement note
- Current acknowledgement state

This would prevent multiple users from independently investigating the same issue.

---

## Notification Retry / Recovery

Improve failed notification handling with controlled retries.

Potential future behaviour:

```text
Pending
  ↓
Attempt
  ├── Success → Sent
  │
  └── Failure
       ↓
     Retry
       ↓
 Maximum attempts
       ↓
 Permanently Failed
```

The existing job architecture already provides a foundation for this.

---

## In-App Notifications

The notification system already contains an `in_app_enabled` setting.

A future version could implement an in-app notification center containing:

- Unread notifications
- Read/unread state
- Notification timestamps
- Links to affected WSOs
- Notification acknowledgement

---

## Notification Templates

Move notification HTML into configurable templates rather than constructing message bodies directly inside business services.

Potential template variables include:

```text
{{wso_number}}
{{req_number}}
{{product}}
{{stage}}
{{recipient}}
{{actor}}
```

This would allow notification wording to evolve without changing business logic.

---

## Improved Dashboard Activity

The current activity table can be expanded with:

- More advanced pagination
- Filtering
- Date ranges
- User filters
- Production-stage filters
- WSO search

---

## Reporting and Analytics

Potential future reporting features include:

- Production turnaround time
- Average time per production stage
- Delayed orders
- Receiving performance
- Workshop throughput
- Monthly production statistics
- Completed versus cancelled WSOs
- Stage bottleneck analysis

---

## Audit History

The existing production history can be expanded into a more comprehensive audit system covering:

- WSO creation
- WSO edits
- Quantity changes
- Stage changes
- Receiving
- Cancellation
- Reactivation
- Attachments
- Notification configuration changes

---

## Advanced Permissions

Future versions may introduce more granular permissions beyond simple roles.

Examples:

```text
WSO_CREATE
WSO_EDIT
WSO_CANCEL
WSO_REACTIVATE
WSO_RECEIVE
NOTIFICATION_CONFIGURE
USER_MANAGE
REPORT_VIEW
```

---

## Production Workflow Automation

Future versions could introduce automated workflow transitions based on production events.

For example:

```text
All quantities received
        ↓
Product completed
        ↓
All products completed
        ↓
WSO automatically completed
```

---

## ERP Integration

A future integration could connect the Operations Platform to the organization's ERP system.

Potential integration points include:

- Requisition numbers
- Product master data
- Stock information
- Order creation
- Receiving
- Production status
- Inventory synchronization

---

# 38. Long-Term Vision

The Operations Platform is intended to evolve from a WSO tracking tool into a broader operational workflow platform.

The long-term goal is not simply to maintain a database of workshop orders.

The system should answer operational questions such as:

> What needs to happen?

> What is currently happening?

> What is delayed?

> Who needs to act?

> Has someone acknowledged the problem?

> Who was notified?

> Was the notification delivered?

> What happened to this order?

> Where are production bottlenecks occurring?

The platform should progressively turn these questions into visible, actionable workflows rather than requiring users to reconstruct the answers manually from database records.

---

# 39. Project Status

**Current stage: Functional MVP / Operational Validation**

The core system is operational and the end-to-end workflow has been established.

The next development phase should prioritize incremental improvements based on actual user interaction with the system rather than introducing unnecessary architectural complexity.