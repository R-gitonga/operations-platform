-- ============================================================
-- Suppliers
-- ============================================================
--
-- External parties from whom Purchase Order (PO) products are
-- sourced.
--
-- Deliberately minimal: name, a single point of contact, and
-- an active flag. Everything else about a PO's supplier
-- relationship (which orders, which deliveries) lives on the
-- PO records themselves, not here.
--
-- `active` follows the same convention as branding_types /
-- branding_locations: inactive suppliers are hidden from
-- pickers on new records but historical POs referencing them
-- remain untouched.
-- ============================================================

CREATE TABLE IF NOT EXISTS suppliers (

    id              SERIAL PRIMARY KEY,

    name            VARCHAR(150) NOT NULL,

    contact_name    VARCHAR(150),

    contact_info    VARCHAR(255),

    active          BOOLEAN NOT NULL DEFAULT TRUE,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS
idx_suppliers_active
ON suppliers(active);