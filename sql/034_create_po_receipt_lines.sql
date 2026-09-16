-- ============================================================
-- PO Receipt Lines
-- ============================================================
--
-- One row per po_line_item touched by a given po_receipt.
-- Insert-only: a line can appear in several receipts over time
-- as the supplier delivers in stages. This is deliberately NOT
-- a mutated quantity on po_line_items -- see po_line_items.sql.
--
-- qty_delivered is what physically arrived in this delivery,
-- before any inspection. Whether it was ultimately accepted is
-- tracked separately in po_defects (added in a later migration)
-- and is NOT decided here.
--
-- total_delivered_to_date / delivered_balance are snapshots of
-- the running totals as of this receipt, following the same
-- convention as wso_partial_receipt_events, so the Procurement
-- Timeline can render history without recomputing sums. Live
-- outstanding-including-defects figures will be computed
-- separately once po_defects exists.
-- ============================================================

CREATE TABLE IF NOT EXISTS po_receipt_lines (

    id SERIAL PRIMARY KEY,

    po_receipt_id INTEGER NOT NULL
        REFERENCES po_receipts(id)
        ON DELETE CASCADE,

    po_line_item_id INTEGER NOT NULL
        REFERENCES po_line_items(id),

    qty_delivered INTEGER NOT NULL,

    total_delivered_to_date INTEGER NOT NULL,

    delivered_balance INTEGER NOT NULL,

    CONSTRAINT chk_po_receipt_lines_qty_delivered
        CHECK (qty_delivered > 0)

);

CREATE INDEX IF NOT EXISTS
idx_po_receipt_lines_receipt
ON po_receipt_lines(po_receipt_id);

CREATE INDEX IF NOT EXISTS
idx_po_receipt_lines_line_item
ON po_receipt_lines(po_line_item_id, po_receipt_id);