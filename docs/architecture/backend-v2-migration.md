# Backend Migration Specification (v2)

## Purpose

This document describes the next evolution of the WSO Tracker backend.

The objective is **not** to redesign the application architecture.

The existing layered architecture must remain intact.

```
Handlers
    ↓
Services
    ↓
Repositories
    ↓
PostgreSQL
```

Only the domain model is changing to better reflect the real Excel workflow used by the business.

---

# Design Principles

The implementation must preserve:

- Repository pattern
- Service layer orchestration
- Transactional business operations
- AppState dependency injection
- Shared AppError handling
- Current routing structure
- Existing project organization

No SQL should appear inside handlers.

No business logic should appear inside repositories.

---

# New Domain Model

## Category

Represents one worksheet/category.

Examples:

- Shirts
- Trousers
- Fleeces
- Blazers
- Sportswear

Table:

categories

Fields:

- id
- name

---

## WSO

Represents one requisition.

Fields:

- id
- category_id (FK categories.id)
- date_signed
- wso_number
- req_number
- description
- design_code
- fabric_code
- remarks

---

## WSO Line Item

Represents one size entry.

Fields:

- id
- wso_order_id
- size
- qty_raised
- qty_received
- received_date
- status

Balance is NOT stored.

Balance must be computed as:

qty_raised - qty_received

---

# Relationships

Category

1

↓

Many

WSO

1

↓

Many

WSO Line Item

---

# Required Database Changes

Create categories table.

Seed default categories.

Modify wso_orders table:

Add:

- category_id
- date_signed
- design_code
- fabric_code

Remove:

Nothing yet.

Existing data should remain compatible.

Modify wso_line_items table:

Rename:

quantity

↓

qty_raised

Add:

qty_received
received_date
status

Do NOT add balance column.

---

# Repository Changes

Repositories must expose methods equivalent to current naming style.

Examples:

Category Repository

find_all()

find_by_id()

create()

WSO Repository

create()

update()

find_all()

find_by_id()

Line Item Repository

create()

update()

delete()

find_by_wso()

Business naming should remain consistent with current codebase.

---

# Service Changes

Transactional WSO creation must now include:

Create WSO

↓

Insert line items

↓

Commit transaction

The service layer must remain responsible for orchestration.

---

# API Changes

POST /wso

Request

Should now accept

category_id

date_signed

design_code

fabric_code

plus existing fields.

Line items should accept

qty_raised

qty_received

received_date

status

Response

Continue returning WsoDetail.

WsoDetail should expose computed balance for each line item.

---

# Business Rules

Balance

Never stored.

Always computed.

Received quantity

Cannot exceed quantity raised.

Status

Should be validated.

Accepted values:

Raised

Approved

Cutting

Stitching

Printing

Ready

Partially Received

Completed

Cancelled

No Fabric

---

# Non Goals

Do not redesign architecture.

Do not rewrite routing.

Do not introduce ORM abstractions.

Do not remove repositories.

Do not collapse service layer.

Do not move SQL into handlers.

---

# Deliverables

The implementation should update:

SQL migrations

Models

Repositories

Services

Handlers

Routes

Transaction logic

without changing overall architecture.