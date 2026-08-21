-- ============================================================
-- Branding Types
-- ============================================================
--
-- Configurable types of branding that can be applied to a
-- WSO product.
--
-- Examples:
--   Embroidery
--   Screenprint
--   Heatpress
--
-- These are intentionally stored as data rather than hardcoded
-- application values so they can be managed from Settings.
-- ============================================================

CREATE TABLE IF NOT EXISTS branding_types (

    id SERIAL PRIMARY KEY,

    code VARCHAR(100) NOT NULL UNIQUE,

    display_name VARCHAR(150) NOT NULL,

    description TEXT,

    display_order INTEGER NOT NULL DEFAULT 0,

    active BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

);

CREATE INDEX IF NOT EXISTS
idx_branding_types_active_order
ON branding_types(active, display_order);