# WSO Tracker Backend

## Project Overview

This backend service provides CRUD operations for WSO orders and line items. It exposes an HTTP API for creating, retrieving, updating, and cancelling WSO orders.

## Tech Stack

- Rust
- Axum
- SQLx
- PostgreSQL
- dotenvy
- chrono

## Folder Structure

- `src/` - application source code
  - `app_state.rs` - shared application state and database pool
  - `database/` - database connection abstractions
  - `errors/` - shared application error types
  - `handlers/` - HTTP request handlers
  - `models/` - data models and request/response structs
  - `repositories/` - database access layer and SQL queries
  - `routes/` - route definitions and endpoint wiring
- `docs/architecture/` - architecture and design documentation
- `sql/` - database schema and migration scripts

## How to Run

1. Create a `.env` file with `DATABASE_URL` set to your PostgreSQL connection string.
2. Run the application:

```bash
cargo run
```

3. The API listens on `http://localhost:3000`.

## How to Test

This project currently uses `cargo test` for Rust tests.

```bash
cargo test
```

## API Overview

- `POST /wso` - create a new WSO order
- `GET /wso` - list all WSO orders
- `GET /wso/{id}` - get a single WSO order by ID
- `PUT /wso/{id}` - update an existing WSO order
- `PATCH /wso/{id}/cancel` - cancel a WSO order

> Note: This README will be updated throughout development as the API and project evolve.
# Development Road Map
# Sprint 1
    -WSO CRUD
    -LINE ITEM CRUD
    -REPOSITORY PATTERN
    -ROUTE SEPARATION
    -APPSTATE

# Sprint 2
    Authentication
    Users
    JWT
    Login
    Roles

# Sprint 3
    Email Notifications
    Audit Logs
    Search
    Filtering
    Pagination

# Sprint 4
    React Frontend
    Login Screen
    Dashboard
    WSO list
    Wso Details
    Create/Edit Forms

# My workflow
- Database design
- Sql Migration
- Model
- Repository
- Handler
- Route
- Postman test
- Documentation
- Git commit

# Business Features
    Authentication
    Users
    Roles
    Audit Logs
    Email notification
    Dashboard Statistics
    Search and Filters

# Instead of thinking in tables, I should think in business workflows
instead of:
    create WSO
    Create Line Item
    Update Line Item

I should:
    Create workshop order
    Header
    Multiple Line Items
    Notify Team
    Audit Log
    Return Complete Order


# REACT FRONTEND

`what does the user want to do?`
    login(later)
      ↓
    Dashboard
      ↓
    Recent Wso orders
      ↓
    Search
      ↓
    Open WSO
      ↓
    Update Quantities
      ↓
    Cance if necessary
      ↓
    Creat new WSO

`MVP Screens` -> Five pages
    src/
    pages/
        dashboard
        wsolist
        wsodetail
        createwso
        editwso
` React Architecture`
    frontend/
    src/
        api/
        components/
        pages/
        layouts/
        hooks/
        types/
        services/
        app.tsx
        main.tsx

Component Hierarchy
    App
    |__ MainLayout
            |__ Sidebar
            |__ Header
            |__ Page
    `Every page will share the same navigation`

`Styling`
We'll use a component library
React + TypeScript + Vite + Tailwind CSS + shadcn/ui

`why?`
    professional appearance
    accessible components
    responsive by default
    extremely common in modern projects

`API Layer`
React will be unaware of URLs
api/
wso.ts
lineItem.ts
dashboard.ts

    `Example`
    pages/
    Dashboard
    ↓
    api/dashboard.ts
    ↓
    GET /wso/summary
`Git Workflow`
    Create new branch in GIT `feature/frontendmvp`

`Roadmap to Follow
    - Create React App
    - Install Dependencies
    - Configure Routing
    - Configure Api Client
    - Create Layout
    - Create Navigation
    - Create Empty Pages

# Frontend Tech Stack
React + TypeScript + Vite + React Router  + Tailwind CSS + shadcn/ui + TanStack Query + Axios