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

# Sprint 3: WSO Aggregate
    returns a business object rather than a database row.
# Wso Detail:
    -Which fields belong in the response?
    -Should we include computed fields such as total_quantity?
    -Should the endpoint return raw database values or a business-oriented representation?

Every item in WSO should return the size and qty for every item in the wso
should include a total_qty as well so we dont have to calculate it for every page in frontend
BACKEND -> CALCULATE ONCE -> RETURN TOTAL QTY used throughtout the frontend.
Also include line_item_count
# Model Design
    This model will not exist in PostgreSQL. 
    We wont have a table called wso_detail
    It will be purely an API model
    Database Models -> Business Models -> API Models

# Repository
    we wont create a repository either because no table exists
    instead we'll use what we already have.
    Handler -> WSO Repository -> Line Item Repositry -> Construct WsoDetail

# Handler Flow
    GET /wso/{id}/details -> find WSO -> find line items -> calculate total_quantity -> calculate line_item_count -> construct WsoDetail -> return JSON
    
    To make the endpoint even better, instead of:
    GET /wso/{id}/details, we'll use
    GET /wso/{id},
    and remove the old "header-only" response.

    Why?
        From a business perspective, there is no such thing as a WSO without its line itens.
        They are one aggregate. A react page that opens a WSO almost always wants the entire object.
        We can keep the list endpoint (GET /wso) lightweight for tables and searches,
        while making the single-resource endpoint (GET /wso/{id}) rich and complete
        This design closely follows how real business applications are typically consumed and gives us a strong foundation for adding future computed values such as:
            total_quantity
            line_item_count
            is_cancelled
            last_updated_by
            has_pending_changes
        without requiring additional API calls or frontend calculations.

# New Desingn Principle
    Repositories should never call repositories.
    Good
    Handler -> WSO Repository -> Line Item Repository -> Compose Response
    Bad
    WSO Repository - Line Item Repository -> WSO repository -> ...
# Future Proofing
    WsoDetail should become our canonical Workshop Order Response.
    meaning,
    GET /wso, returns;
        [
            {
                "id":1,
                "wso_number":"WSO-OO1",
                "status":"active"
            }
        ]
    light weight list, while 
    GET /wso/{id}, returns;
        {
            "id":1,
            "wso_number":"WSO-OO1",
            "status":"active",
            line_itme_count:3,
            "total_quantity":47,
            line_items":[...]
        }
    full business object.




## Recent progress since last update
- Created new Git feature branch `feature/line-items` to isolate line-item work from `main`.
- Committed all backend changes on `feature/line-items` with message: `feat(line-items): implement line-item handlers and init feature branch`.
- Continued improving business-readable naming for line-item handlers and repository call sites.
- Added `src/services/wso.rs` to build `WsoDetail` from `WsoOrder` and `WsoLineItem` data.
- Updated `src/handlers/wso.rs` so `GET /wso/{id}` now returns the rich business aggregate `WsoDetail`.
- Exposed the new service layer in `src/main.rs` with `mod services;`.
- Kept `WsoDetail` as a pure API response model, not a database table.

# Four questions to ask before every implementation
    Business problem? Example -> A wso should be created atomically
    API contract? Example -> POST /wso accepts header _ line items.
    Data Model? Example -> New request DTO containing nested line items
    Implemantation plan? Example -> Model -> Repository -> Handler -> Route -> Test -> Docs

# Future Proposed Roadmap.
Version 0.4
- Strucutred Logging
- Config Module
- Validation
- Better Error Types

version 0.5
- Transactional Create WSO
- Header + Multiple Line Items _ One API Call

Version 0.6
- Authentication
- Users
- Roles

Version 1.0
- Email Notification
- Audit Trail
- react Frontend
