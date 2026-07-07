.iter()
.map()
.sim()
ownership
moving Vec to structs
composition

# Learning Note under sprint 3: WSO Details
    CreateWsoRequest is a Request MOdel
    WsoOrder is a Database Model
    WsoDetail is a Response Model
They all represent different views of the same business object

## New concepts introduced for WSO details
- `WsoDetail` is a response model / business aggregate, not a SQL table.
- The service layer is the correct place to compose aggregates from multiple repository results.
- `GET /wso/{id}` should return a rich, canonical WSO object with header data, child line items, and computed fields.
- Computed fields like `total_quantity` and `line_item_count` should be calculated once on the backend and returned as part of the response.
- Repositories remain focused on table-level queries; handlers and services compose those results into business-oriented API models.
