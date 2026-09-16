-- ============================================================
-- PO Defects
-- ============================================================
--
-- Linked to po_line_items (not a specific po_receipt_line) --
-- a defect is a quality fact about the product/line, not about
-- exactly which delivery is at fault, per our discussion.
--
-- Deliberately a lean two-state lifecycle: 'open' -> 'resolved'.
-- The disposition (replaced / returned for credit / written off
-- / accepted after all) is captured once, at resolution time,
-- rather than modelled as a multi-step workflow. Resolving a
-- defect never creates a delivery -- a replacement is recorded
-- through the normal receiving flow, against the same line item.
-- ============================================================

CREATE TABLE IF NOT EXISTS po_defects (

    id SERIAL PRIMARY KEY,

    po_line_item_id INTEGER NOT NULL
        REFERENCES po_line_items(id)
        ON DELETE CASCADE,

    qty_defective INTEGER NOT NULL
        CHECK (qty_defective > 0),

    reason TEXT NOT NULL,

    status VARCHAR(20) NOT NULL DEFAULT 'open'
        CHECK (status IN ('open', 'resolved')),

    resolution_type VARCHAR(30)
        CHECK (resolution_type IN (
            'replaced', 'returned_for_credit', 'written_off', 'accepted'
        )),

    resolution_notes TEXT,

    reported_by VARCHAR(255) NOT NULL,

    reported_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    resolved_by VARCHAR(255),

    resolved_at TIMESTAMPTZ,

    CONSTRAINT chk_po_defects_resolution CHECK (
        (status = 'open'
            AND resolution_type IS NULL
            AND resolved_at IS NULL
            AND resolved_by IS NULL)
        OR
        (status = 'resolved'
            AND resolution_type IS NOT NULL
            AND resolved_at IS NOT NULL
            AND resolved_by IS NOT NULL)
    )

);

CREATE INDEX IF NOT EXISTS
idx_po_defects_line_item
ON po_defects(po_line_item_id);

CREATE INDEX IF NOT EXISTS
idx_po_defects_open
ON po_defects(po_line_item_id)
WHERE status = 'open';