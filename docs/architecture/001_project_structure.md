# Project Structure

## What problem does this solve?
This document explains how the backend project is organized so developers can quickly understand where features belong, how the code is separated, and where to add new functionality.

A clear project structure reduces onboarding time, minimizes accidental coupling between unrelated code, and makes it easier to maintain and extend the service over time.

## Why was this approach chosen?
The structure is chosen to follow a simple domain-driven layout, with folders for:

- `src/app_state.rs` – application state and shared resources such as the database pool
- `src/routes/` – routing definitions and endpoint wiring
- `src/handlers/` – request handlers that translate HTTP requests into application logic
- `src/repositories/` – data access layer and SQL interactions
- `src/models/` – request and response models, database row mappings
- `src/errors/` – centralized error types and conversion logic
- `src/database/` – database connection abstractions

This organization keeps each concern in its own area and avoids mixing SQL, HTTP handling, and business logic in the same file.

## What alternatives were considered?
- **Layered feature modules**: grouping by feature rather than technical role (e.g. `wso/`, `line_item/`). This can work well for large systems, but this backend is still small enough that a role-based separation is easier to follow.
- **Monolithic single file**: keeping all logic in one place. This is easy at first but quickly becomes unmanageable as the application grows.
- **Service-only organization**: placing all business logic in service modules and leaving handlers very thin. That is a valid approach, but this project already has a clean separation between handlers, repositories, and models and does not require another indirection yet.

## Phase 2

`attachment uploads`
`image preview`
`pdf preview`
`Download attachments`
`drag and drop uploads`
`Using OCR to extract information from the pdf?`