-- ============================================================
-- WSO Item Branding Requirements
-- ============================================================
--
-- Associates a WSO product with one or more branding
-- requirements.
--
-- Branding types and locations are configurable reference data.
-- This table stores the actual requirement for a particular
-- WSO item.
--
-- Example:
--
-- WSO Item
--   |
--   +-- Embroidery + Pocket
--   |
--   +-- Heatpress + Sleeve
--   |
--   +-- Screenprint + Chest
--
-- Nothing about the available branding types or locations is
-- hardcoded here.
-- ============================================================

CREATE TABLE IF NOT EXISTS wso_item_branding (

    id SERIAL PRIMARY KEY,

    wso_item_id INTEGER NOT NULL
        REFERENCES wso_items(id)
        ON DELETE CASCADE,

    branding_type_id INTEGER NOT NULL
        REFERENCES branding_types(id),

    branding_location_id INTEGER NOT NULL
        REFERENCES branding_locations(id),

    quantity INTEGER NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_wso_item_branding_quantity
        CHECK (quantity > 0),

    CONSTRAINT uq_wso_item_branding_requirement
        UNIQUE (
            wso_item_id,
            branding_type_id,
            branding_location_id
        )
);

CREATE INDEX IF NOT EXISTS
idx_wso_item_branding_item
ON wso_item_branding(wso_item_id);

CREATE INDEX IF NOT EXISTS
idx_wso_item_branding_type
ON wso_item_branding(branding_type_id);

CREATE INDEX IF NOT EXISTS
idx_wso_item_branding_location
ON wso_item_branding(branding_location_id);