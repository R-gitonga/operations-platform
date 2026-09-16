-- ============================================================
-- PO Item Notes
-- ============================================================
--
-- Free-text, append-only commentary against a PO item that
-- isn't tied to a specific receipt or defect — e.g. "Supplier
-- confirmed dispatch by courier, ETA Thursday".
--
-- In WSO, this kind of note piggybacks on the stage-history
-- table (a note is attached to whichever stage change it
-- accompanied). PO items have no stage concept to attach to,
-- so notes get their own table from the start.
--
-- Together with po_receipts/po_receipt_lines and po_defects,
-- this is one of the three event sources merged (at read time,
-- not stored redundantly) into a PO item's "Procurement
-- Timeline" — the equivalent of WSO's Production Timeline.
-- ============================================================

CREATE TABLE IF NOT EXISTS po_item_notes (

    id            SERIAL PRIMARY KEY,

    po_item_id    INTEGER NOT NULL
                 REFERENCES po_items(id)
                 ON DELETE CASCADE,

    note          TEXT NOT NULL,

    created_by    TEXT NOT NULL,

    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS
idx_po_item_notes_po_item_id
ON po_item_notes(po_item_id);

CREATE INDEX IF NOT EXISTS
idx_po_item_notes_created_at
ON po_item_notes(created_at DESC);