# Error Handling

## What problem does this solve?
This document describes how errors are captured, converted, and returned consistently across the backend.

Consistent error handling ensures the API returns predictable HTTP responses, avoids panics, and keeps error conversion logic in one place.

## Why was this approach chosen?
The project centralizes application-level errors in `src/errors/app_error.rs`.

This approach was chosen because:

- it separates error mapping from business logic and request handlers
- it allows repository functions to return low-level errors while handlers rely on a common `AppError` type
- it provides a single place to translate errors into HTTP responses and JSON messages

## What alternatives were considered?
- **Handler-level error mapping**: each handler would map database errors to HTTP status codes individually. This is workable, but leads to repeated logic and inconsistent responses.
- **Panic on errors**: using `.unwrap()` or `.expect()` in handlers. That is unsafe in production and was avoided deliberately.
- **Custom error types per module**: building separate error enums for each domain. That can be useful for large systems, but for this application a single central error type provides enough consistency without unnecessary complexity.
