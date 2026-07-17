# Workshop Order Business Rules

This document defines the business rules governing Workshop Orders (WSOs).

The purpose of this document is to separate business policy from implementation so that future developers can understand why rules exist before changing code.

---

# Workshop Order Lifecycle

A Workshop Order progresses through the following lifecycle:

```
Active
   │
   ├────────────► Partial
   │                 │
   │                 ▼
   └────────────► Completed

Active
   │
   └────────────► Cancelled
                      │
                      ▼
                 Reactivated
                      │
                      ▼
                    Active
```

Status meanings:

| Status | Meaning |
|---------|---------|
| Active | Work has been created but no production has been received. |
| Partial | Some production has been received but the order is not yet complete. |
| Completed | All quantities have been received. |
| Cancelled | Production has been permanently stopped. |

---

# Status Calculation Rules

WSO status is automatically calculated from its line items.

The status is never manually edited by users.

Rules:

If

```
All line item balances == 0
```

then

```
Completed
```

Else if

```
Any line item qty_received > 0
```

then

```
Partial
```

Else

```
Active
```

Cancelled orders are never modified by the automatic status engine.

---

# Editing Rules

## Active

May be edited.

## Partial

May be edited.

## Completed

Cannot be edited.

Reason:

Completed orders represent finished production and become read-only historical records.

## Cancelled

Cannot be edited.

Reason:

Cancelled production must remain historically accurate.

---

# Receiving Rules

Receiving production is only allowed for:

- Active orders
- Partial orders

Receiving is NOT allowed for:

- Completed orders
- Cancelled orders

Additional validations:

- Quantity received must be greater than zero.
- Quantity received cannot exceed quantity raised.

---

# Cancelling Rules

A Workshop Order may be cancelled only if:

- it is Active
- it is Partial

Cannot cancel:

- Completed orders
- Already cancelled orders

Reason:

Completed production cannot be retroactively cancelled.

---

# Reactivation Rules

Only Cancelled orders may be reactivated.

Reactivation restores the order to Active status.

The automatic status engine will subsequently move it to Partial or Completed depending on production receipts.

---

# Upload Rules

Attachments may only be uploaded while a Workshop Order is:

- Active
- Partial

Completed and Cancelled orders are read-only.

---

# Line Item Rules

Each line item must satisfy:

```
qty_raised >= 0
```

```
qty_received >= 0
```

```
qty_received <= qty_raised
```

Balance is always calculated as

```
balance = qty_raised - qty_received
```

Balance is never entered manually.

---

# Dashboard Rules

Dashboard counts are calculated from Workshop Order status.

Displayed KPIs:

- Total Orders
- Active Orders
- Partial Orders
- Completed Orders
- Cancelled Orders
- Total Quantity Raised
- Total Quantity Received
- Outstanding Balance

Largest Outstanding Orders:

Include only

- Active
- Partial

Exclude

- Completed
- Cancelled

Only orders with outstanding quantity greater than zero appear.

---

# Design Principles

Business rules belong inside

```
services/wso_rules.rs
```

The frontend may hide buttons for convenience, but the backend is the source of truth.

All endpoints must validate business rules before modifying Workshop Orders.

Future business rules should be added to this document before implementation whenever possible.