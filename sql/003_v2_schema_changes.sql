-- V2 schema migration for WSO Tracker
-- Creates categories, adds v2 columns to existing tables, and renames line item quantity.

CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE
);

INSERT INTO categories (name)
VALUES
    ('Shirts'),
    ('Trousers'),
    ('Fleeces'),
    ('Blazers'),
    ('Sportswear')
ON CONFLICT (name) DO NOTHING;

ALTER TABLE wso_orders
    ADD COLUMN IF NOT EXISTS category_id INTEGER,
    ADD COLUMN IF NOT EXISTS date_signed DATE,
    ADD COLUMN IF NOT EXISTS design_code VARCHAR(100),
    ADD COLUMN IF NOT EXISTS fabric_code VARCHAR(100);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.table_constraints
        WHERE constraint_schema = current_schema()
          AND table_name = 'wso_orders'
          AND constraint_name = 'fk_wso_orders_category'
    ) THEN
        ALTER TABLE wso_orders
            ADD CONSTRAINT fk_wso_orders_category
            FOREIGN KEY (category_id)
            REFERENCES categories(id)
            ON DELETE SET NULL;
    END IF;
END $$;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_schema = current_schema()
          AND table_name = 'wso_line_items'
          AND column_name = 'quantity'
    )
    AND NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_schema = current_schema()
          AND table_name = 'wso_line_items'
          AND column_name = 'qty_raised'
    ) THEN
        ALTER TABLE wso_line_items
            RENAME COLUMN quantity TO qty_raised;
    END IF;
END $$;

ALTER TABLE wso_line_items
    ADD COLUMN IF NOT EXISTS qty_received INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS received_date DATE,
    ADD COLUMN IF NOT EXISTS status VARCHAR(30) NOT NULL DEFAULT 'Raised';
