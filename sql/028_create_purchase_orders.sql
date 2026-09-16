-- ============================================================
-- Purchase Orders (PO)
-- ============================================================
--
-- Header record for an externally sourced purchase order.
--
-- Deliberately minimal: both external references, the
-- supplier, a general description, the final order attachment,
-- and a lifecycle status.
--
-- Fulfilment (received/outstanding), quality (defects) and
-- timeline (overdue) are all DERIVED from po_items /
-- po_line_items / po_receipts / po_defects — never stored here.
-- `status` only tracks the one manual action a user can take:
-- cancelling a PO. There is no "completed"/"closed" value; that
-- is a computed label, not a stored state.
-- ============================================================

CREATE TABLE IF NOT EXISTS purchase_orders (

    id                   SERIAL PRIMARY KEY,

    erp_reference        VARCHAR(50) NOT NULL UNIQUE,

    accounts_reference   VARCHAR(50) NOT NULL UNIQUE,

    supplier_id          INTEGER NOT NULL
                         REFERENCES suppliers(id)
                         ON DELETE RESTRICT,

    description          TEXT,

    attachment_path      TEXT,

    attachment_name      TEXT,

    status               VARCHAR(20) NOT NULL DEFAULT 'active',

    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS
idx_purchase_orders_supplier_id
ON purchase_orders(supplier_id);

CREATE INDEX IF NOT EXISTS
idx_purchase_orders_status
ON purchase_orders(status)