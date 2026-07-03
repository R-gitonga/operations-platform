# Backend v2 Domain and Routes

## Summary

Phase 2 updates the backend domain model to match `sql/003_v2_schema_changes.sql`.

The existing architecture remains unchanged:

```text
Handlers
Services
Repositories
PostgreSQL
```

Handlers parse HTTP input and return JSON responses. Services orchestrate business rules and validation. Repositories own SQL queries.

## Domain Changes

WSO orders now include:

- `category_id`
- `date_signed`
- `design_code`
- `fabric_code`

WSO line items now include:

- `qty_raised`
- `qty_received`
- `received_date`
- `status`
- `balance`

`balance` is computed in SQL as `qty_raised - qty_received`. It is not stored in the database.

The old `quantity` field is no longer part of backend source models, repository queries, or responses.

## Category Routes

Categories now have HTTP routes:

- `GET /categories`
- `POST /categories`
- `GET /categories/{id}`
- `PUT /categories/{id}`
- `DELETE /categories/{id}`

Category delete will fail at the database level if a WSO references that category because `wso_orders.category_id` has a foreign key.

## WSO Routes

Existing WSO routes remain:

- `POST /wso`
- `GET /wso`
- `GET /wso?search={wso_number}`
- `GET /wso?status={status}`
- `GET /wso/{id}`
- `PUT /wso/{id}`
- `PATCH /wso/{id}/cancel`
- `GET /wso/summary`

`POST /wso` creates the WSO header and line items transactionally.

## Line Item Routes

Existing line item routes remain:

- `POST /wso/{id}/line-items`
- `GET /wso/{id}/line-items`
- `GET /line-items/{id}`
- `PUT /line-items/{id}`
- `DELETE /line-items/{id}`

## Validation Rules

Line item validation is handled in the service layer:

- `qty_raised` cannot be negative.
- `qty_received` cannot be negative.
- `qty_received` cannot exceed `qty_raised`.
- `status` must be one of:
  - `Raised`
  - `Approved`
  - `Cutting`
  - `Stitching`
  - `Printing`
  - `Ready`
  - `Partially Received`
  - `Completed`
  - `Cancelled`
  - `No Fabric`

Invalid requests return `400 Bad Request`.

## Summary Fixes

Dashboard summary now counts orders with `COUNT(DISTINCT wso_orders.id)` so orders with multiple line items are not overcounted.

Summary response fields are:

- `total_orders`
- `status_counts`
- `total_qty_raised`
- `total_qty_received`
- `total_balance`

## Verification

The backend was verified locally against PostgreSQL with:

- `cargo check`
- `cargo test`
- Category create, list, detail, update, and delete
- WSO create, list, search, status filter, detail, update, summary, and cancel
- Line item create, list, detail, update, and delete
- Invalid line item quantity validation returning `400 Bad Request`

The backend is ready for frontend integration against the v2 API contract.
