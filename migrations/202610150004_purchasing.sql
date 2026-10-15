CREATE TABLE suppliers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    contact_name VARCHAR(150),
    contact_info VARCHAR(255),
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE purchase_orders (
    id SERIAL PRIMARY KEY,
    erp_reference VARCHAR(50) NOT NULL UNIQUE,
    accounts_reference VARCHAR(50) NOT NULL UNIQUE,
    supplier_id INTEGER NOT NULL REFERENCES suppliers(id) ON DELETE RESTRICT,
    description TEXT,
    attachment_path TEXT,
    attachment_name TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'cancelled')),
    created_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX purchase_orders_supplier_idx ON purchase_orders(supplier_id);
CREATE INDEX purchase_orders_status_idx ON purchase_orders(status);
CREATE INDEX purchase_orders_created_idx ON purchase_orders(created_at DESC);
CREATE TABLE po_items (
    id SERIAL PRIMARY KEY,
    purchase_order_id INTEGER NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    description TEXT,
    expected_delivery_date DATE,
    branding_required BOOLEAN NOT NULL DEFAULT FALSE,
    branding_type_id INTEGER REFERENCES branding_types(id) ON DELETE SET NULL,
    branding_location_id INTEGER REFERENCES branding_locations(id) ON DELETE SET NULL,
    created_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX po_items_order_idx ON po_items(purchase_order_id);
CREATE INDEX po_items_delivery_idx ON po_items(expected_delivery_date);
CREATE TABLE po_item_notes (
    id SERIAL PRIMARY KEY,
    po_item_id INTEGER NOT NULL REFERENCES po_items(id) ON DELETE CASCADE,
    note TEXT NOT NULL,
    created_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX po_item_notes_item_date_idx ON po_item_notes(po_item_id, created_at DESC);
CREATE TABLE po_line_items (
    id SERIAL PRIMARY KEY,
    po_item_id INTEGER NOT NULL REFERENCES po_items(id) ON DELETE CASCADE,
    size VARCHAR(50) NOT NULL,
    qty_ordered INTEGER NOT NULL CHECK (qty_ordered > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX po_line_items_item_idx ON po_line_items(po_item_id);
CREATE TABLE po_receipts (
    id SERIAL PRIMARY KEY,
    purchase_order_id INTEGER NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
    delivery_note_reference VARCHAR(255),
    notes TEXT,
    received_by VARCHAR(255) NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX po_receipts_order_date_idx ON po_receipts(purchase_order_id, received_at);
CREATE TABLE po_receipt_lines (
    id SERIAL PRIMARY KEY,
    po_receipt_id INTEGER NOT NULL REFERENCES po_receipts(id) ON DELETE CASCADE,
    po_line_item_id INTEGER NOT NULL REFERENCES po_line_items(id),
    qty_delivered INTEGER NOT NULL CHECK (qty_delivered > 0),
    total_delivered_to_date INTEGER NOT NULL CHECK (total_delivered_to_date >= qty_delivered),
    delivered_balance INTEGER NOT NULL,
    CONSTRAINT po_receipt_lines_receipt_line_unique UNIQUE (po_receipt_id, po_line_item_id)
);
CREATE INDEX po_receipt_lines_line_idx ON po_receipt_lines(po_line_item_id, po_receipt_id);
CREATE TABLE po_defects (
    id SERIAL PRIMARY KEY,
    po_line_item_id INTEGER NOT NULL REFERENCES po_line_items(id) ON DELETE CASCADE,
    qty_defective INTEGER NOT NULL CHECK (qty_defective > 0),
    reason TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'resolved')),
    resolution_type VARCHAR(30) CHECK (resolution_type IN ('replaced', 'returned_for_credit', 'written_off', 'accepted')),
    resolution_notes TEXT,
    reported_by VARCHAR(255) NOT NULL,
    reported_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    resolved_by VARCHAR(255),
    resolved_at TIMESTAMPTZ,
    CONSTRAINT po_defects_resolution_check CHECK (
        (status = 'open' AND resolution_type IS NULL AND resolved_at IS NULL AND resolved_by IS NULL)
        OR (status = 'resolved' AND resolution_type IS NOT NULL AND resolved_at IS NOT NULL AND resolved_by IS NOT NULL)
    )
);
CREATE INDEX po_defects_line_idx ON po_defects(po_line_item_id);
CREATE TABLE po_settings (
    id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    overdue_grace_days INTEGER NOT NULL DEFAULT 0 CHECK (overdue_grace_days >= 0),
    approaching_window_days INTEGER NOT NULL DEFAULT 2 CHECK (approaching_window_days >= 0),
    stalled_after_days INTEGER NOT NULL DEFAULT 3 CHECK (stalled_after_days >= 0),
    unresolved_defect_after_days INTEGER NOT NULL DEFAULT 3 CHECK (unresolved_defect_after_days >= 0),
    alert_sweep_interval_minutes INTEGER NOT NULL DEFAULT 60 CHECK (alert_sweep_interval_minutes >= 1),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
