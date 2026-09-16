-- ============================================================
-- PO Items
-- ============================================================
--
-- A single product on a Purchase Order (e.g. "White T-Shirt").
-- Sits between purchase_orders (the header) and po_line_items
-- (the sized quantities) — mirrors the wso_orders -> wso_items
-- -> wso_line_items shape.
--
-- Deliberately has NO stage/status column. Unlike a WSO item,
-- a PO item does not move through a manufacturing pipeline —
-- its operational state (fulfilment / quality / timeline) is
-- entirely derived from its line items' receipts and defects.
--
-- expected_delivery_date lives here (per item), not on the PO
-- header, since different products on the same PO may arrive
-- on different schedules.
--
-- Branding here is a record of INTENT only (required + type +
-- location) — this module is supplier-focused. The actual
-- branding work, when required, is raised separately as a WSO
-- in the factory (heat press / embroidery) or sent to an
-- outside screen-print source; this module does not track its
-- completion.
-- ============================================================

CREATE TABLE IF NOT EXISTS po_items (

    id                     SERIAL PRIMARY KEY,

    purchase_order_id      INTEGER NOT NULL
                           REFERENCES purchase_orders(id)
                           ON DELETE CASCADE,

    category_id            INTEGER
                           REFERENCES categories(id)
                           ON DELETE SET NULL,

    description             TEXT,

    expected_delivery_date  DATE,

    branding_required       BOOLEAN NOT NULL DEFAULT FALSE,

    branding_type_id        INTEGER
                            REFERENCES branding_types(id)
                            ON DELETE SET NULL,

    branding_location_id    INTEGER
                            REFERENCES branding_locations(id)
                            ON DELETE SET NULL,

    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at               TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS
idx_po_items_purchase_order_id
ON po_items(purchase_order_id);

CREATE INDEX IF NOT EXISTS
idx_po_items_expected_delivery_date
ON po_items(expected_delivery_date);