CREATE TABLE wso_orders (
    id SERIAL PRIMARY KEY,
    date_signed DATE,
    wso_number VARCHAR(50) NOT NULL UNIQUE,
    req_number VARCHAR(50),
    attachment_path TEXT,
    attachment_name TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'partial', 'completed', 'cancelled')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX wso_orders_status_idx ON wso_orders(status);
CREATE INDEX wso_orders_created_idx ON wso_orders(created_at DESC);

CREATE TABLE wso_items (
    id SERIAL PRIMARY KEY,
    wso_order_id INTEGER NOT NULL REFERENCES wso_orders(id) ON DELETE CASCADE,
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    description TEXT,
    design_code TEXT,
    fabric_code TEXT,
    branding_required BOOLEAN NOT NULL DEFAULT FALSE,
    branding_completed BOOLEAN NOT NULL DEFAULT FALSE,
    current_stage_id INTEGER REFERENCES production_stages(id),
    status TEXT NOT NULL DEFAULT 'Open',
    created_by TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT wso_items_id_order_unique UNIQUE (id, wso_order_id)
);
CREATE INDEX wso_items_order_idx ON wso_items(wso_order_id);
CREATE INDEX wso_items_category_idx ON wso_items(category_id);
CREATE INDEX wso_items_stage_idx ON wso_items(current_stage_id);

CREATE TABLE wso_line_items (
    id SERIAL PRIMARY KEY,
    -- Retained for the current API and order-level dashboard aggregates.
    wso_order_id INTEGER NOT NULL,
    wso_item_id INTEGER NOT NULL,
    size VARCHAR(20) NOT NULL,
    qty_raised INTEGER NOT NULL CHECK (qty_raised >= 0),
    qty_received INTEGER NOT NULL DEFAULT 0,
    received_date DATE,
    status VARCHAR(30) NOT NULL DEFAULT 'Raised',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT wso_line_items_quantities_check CHECK (qty_received BETWEEN 0 AND qty_raised),
    CONSTRAINT wso_line_items_status_check CHECK (status IN (
        'Raised', 'Approved', 'Cutting', 'Stitching', 'Printing', 'Ready',
        'Partially Received', 'Completed', 'Cancelled', 'No Fabric'
    )),
    CONSTRAINT wso_line_items_parent_fk FOREIGN KEY (wso_item_id, wso_order_id)
        REFERENCES wso_items(id, wso_order_id),
    CONSTRAINT wso_line_items_id_item_unique UNIQUE (id, wso_item_id)
);
CREATE INDEX wso_line_items_item_idx ON wso_line_items(wso_item_id);
CREATE INDEX wso_line_items_order_idx ON wso_line_items(wso_order_id);

CREATE TABLE wso_stage_history (
    id SERIAL PRIMARY KEY,
    wso_item_id INTEGER NOT NULL REFERENCES wso_items(id),
    production_stage_id INTEGER NOT NULL REFERENCES production_stages(id),
    notes TEXT,
    changed_by TEXT NOT NULL,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX wso_stage_history_item_date_idx ON wso_stage_history(wso_item_id, changed_at DESC);
CREATE INDEX wso_stage_history_stage_idx ON wso_stage_history(production_stage_id);
CREATE INDEX wso_stage_history_date_idx ON wso_stage_history(changed_at DESC);

CREATE TABLE wso_item_branding (
    id SERIAL PRIMARY KEY,
    wso_item_id INTEGER NOT NULL REFERENCES wso_items(id) ON DELETE CASCADE,
    branding_type_id INTEGER NOT NULL REFERENCES branding_types(id),
    branding_location_id INTEGER NOT NULL REFERENCES branding_locations(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT wso_item_branding_requirement_unique
        UNIQUE (wso_item_id, branding_type_id, branding_location_id)
);
CREATE INDEX wso_item_branding_type_idx ON wso_item_branding(branding_type_id);
CREATE INDEX wso_item_branding_location_idx ON wso_item_branding(branding_location_id);

CREATE TABLE wso_partial_receipt_events (
    id SERIAL PRIMARY KEY,
    wso_item_id INTEGER NOT NULL,
    line_item_id INTEGER NOT NULL,
    quantity_received INTEGER NOT NULL CHECK (quantity_received > 0),
    total_raised INTEGER NOT NULL CHECK (total_raised >= 0),
    total_received INTEGER NOT NULL,
    balance INTEGER NOT NULL,
    received_by VARCHAR(255) NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT wso_partial_receipt_parent_fk FOREIGN KEY (line_item_id, wso_item_id)
        REFERENCES wso_line_items(id, wso_item_id),
    CONSTRAINT wso_partial_receipt_quantities_check CHECK (
        total_received BETWEEN quantity_received AND total_raised
        AND balance = total_raised - total_received
    )
);
CREATE INDEX wso_partial_receipt_item_date_idx ON wso_partial_receipt_events(wso_item_id, received_at);
CREATE INDEX wso_partial_receipt_line_idx ON wso_partial_receipt_events(line_item_id);

CREATE TABLE partial_receiving_tracking (
    id SERIAL PRIMARY KEY,
    wso_item_id INTEGER NOT NULL REFERENCES wso_items(id) ON DELETE CASCADE,
    first_partial_received_at TIMESTAMPTZ NOT NULL,
    notification_sent_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX partial_receiving_tracking_item_idx ON partial_receiving_tracking(wso_item_id);
CREATE UNIQUE INDEX partial_receiving_tracking_active_unique
    ON partial_receiving_tracking(wso_item_id) WHERE resolved_at IS NULL;

CREATE TABLE partial_receiving_settings (
    id INTEGER PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    attention_after_days INTEGER NOT NULL DEFAULT 3 CHECK (attention_after_days >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
