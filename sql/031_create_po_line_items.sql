-- ============================================================
-- PO Line Items
-- ============================================================
--
-- The sized quantity breakdown under a single po_item — e.g.
-- "Small: 50" and "Large: 50" under a "White T-Shirt" po_item.
-- Mirrors wso_line_items' role, but NOT its shape.
--
-- Unlike wso_line_items, this table stores ONLY qty_ordered.
-- There is deliberately no qty_received / balance column here.
-- Received, outstanding, accepted and defective quantities are
-- always derived by summing po_receipt_lines and po_defects
-- against a given line — an insert-only ledger, never a
-- mutated counter. This is the specific WSO pattern the
-- architecture review flagged as unsuitable for LPO receiving.
-- ============================================================

CREATE TABLE IF NOT EXISTS po_line_items (

    id             SERIAL PRIMARY KEY,

    po_item_id     INTEGER NOT NULL
                   REFERENCES po_items(id)
                   ON DELETE CASCADE,

    size           VARCHAR(50) NOT NULL,

    qty_ordered    INTEGER NOT NULL,

    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS
idx_po_line_items_po_item_id
ON po_line_items(po_item_id);