# Repository Pattern

## What problem does this solve?
The repository pattern abstracts database access behind a simple interface so the rest of the application does not depend on raw SQL or direct database calls.

It helps keep the data layer separate from request handling, makes unit testing easier, and centralizes query logic in one location.

## Why was this approach chosen?
This backend separates SQL logic into `src/repositories/`, while handlers remain focused on HTTP request/response flow.

That approach was chosen because:

- it keeps SQL in one place, reducing duplication and making queries easier to update
- it avoids spreading database access logic across multiple handler files
- it makes the codebase easier to reason about and maintain

## What alternatives were considered?
- **Direct SQL in handlers**: easier to implement quickly, but harder to test and maintain.
- **Service layer only**: putting query logic into services instead of repositories. This can work, but the current app structure already benefits from a dedicated repository layer for data access.
- **ORM-based repository**: using a full ORM to encapsulate persistence. This project uses `sqlx` and raw SQL for explicit control and lower dependency overhead.
