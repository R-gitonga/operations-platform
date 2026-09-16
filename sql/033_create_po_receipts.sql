-- ============================================================
-- PO Receipts
-- ============================================================
--
-- One row per delivery event against a purchase order. A single
-- delivery can touch multiple po_items/po_line_items at once
-- (e.g. one delivery note covering several products), so the
-- receipt is scoped to the purchase order, not to a single item.
--
-- Read alongside po_item_notes (via UNION) to build the
-- Procurement Timeline and the PO dashboard's Recent Activity
-- feed, the same way wso_partial_receipt_events is read
-- alongside wso_stage_history for WSO.
-- ============================================================

CREATE TABLE IF NOT EXISTS po_receipts (

    id SERIAL PRIMARY KEY,

    purchase_order_id INTEGER NOT NULL
        REFERENCES purchase_orders(id)
        ON DELETE CASCADE,

    delivery_note_reference VARCHAR(255),

    notes TEXT,

    received_by VARCHAR(255) NOT NULL,

    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

);

CREATE INDEX IF NOT EXISTS
idx_po_receipts_purchase_order
ON po_receipts(purchase_order_id, received_at);