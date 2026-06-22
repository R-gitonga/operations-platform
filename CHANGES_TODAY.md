# Changes Made Today

## What was changed
- Refactored WSO handler logic so SQL is handled exclusively in `src/repositories/wso.rs` and HTTP request handling stays in `src/handlers/wso.rs`.
- Added shared application error handling in `src/errors/app_error.rs` and wired handlers to return `AppError` instead of mapping `StatusCode` manually.
- Updated repository function names for clarity: `find_all`, `find_by_id`, `create`, `update`, and `cancel`.
- Changed the create endpoint to return the created `WsoOrder` object instead of a simple success message.
- Changed the cancel endpoint to return the cancelled `WsoOrder` object instead of a message.
- Added architecture documentation files in `docs/architecture/`.
- Created `backend/README.md` with project overview, tech stack, structure, run/test instructions, and API summary.
- Implemented missing line-item handlers in `src/handlers/line_item.rs` for `get_line_items`, `get_line_item`, `update_line_item`, and `delete_line_item`.

## Why these changes were made
- To improve separation of concerns and keep SQL queries in the repository layer.
- To centralize error handling and make API responses consistent.
- To make repository APIs more descriptive and easier to understand.
- To make create/update/cancel endpoints return useful response objects.
- To document the project design and current architecture for future developers.
- To provide a living README that can be updated as the backend evolves.

# Sprint 2 - Workshop Order Composition
Business requirement
    A workshop order us composed of two parts:
    Workshop order
    |
    |- Header
    |   |-WSO number
    |   |-Request number
    |   |-Description
    |   |Remarks
    |   |Status
    |
    |
    |
    |-Line items
        |-Size 26 -> qty 10
        |-Size 28 -> qty 15
        |-Size 30 -> qty 20


# What API do we need for this milestone?
ALREADY IMPLEMENTED APIs
    POST -> /wso
    GET -> /wso
    GET -> /wso{id}
    PUT -> /wso{id}
    PATCH -> /wso{id}/cancel

# WSO LINE ITEMS
POST: /wso{id}/line-items -> Adds a line item(size and quantity for each wso)
GET: /wso{id}/line-items -> List all the line items
PUT: /line-items{id} -> Updates a line item record
DELETE: /line-items{id} -> Delete a line item record

# Defining Repository API
what endpoints to create for line items?
    - create()
    - find_by_wso()
    - find_by_id()
    - update()
    - delete()

# Handler API
    - create_line-item()
    - get_line_items()
    - update_line_item()
    - delete_line_item()

# Data flow
POST /wso/5/line-items => request {
    "size": "30",
    "quantity":20
} -> route -> Handler -> Repository -> INSERT INTO wso_line_items -> Return WsoLineItem -> Json Response