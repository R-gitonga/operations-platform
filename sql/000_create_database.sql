CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE
);

INSERT INTO categories (name)
VALUES
    ('Shirts'),
    ('Trousers'),
    ('Fleeces'),
    ('Blazers'),
    ('Sportswear');

-- ==========================================
-- WSO Orders
-- ==========================================

CREATE TABLE wso_orders (
    id SERIAL PRIMARY KEY,

    category_id INTEGER,

    date_signed DATE,

    wso_number VARCHAR(50) NOT NULL UNIQUE,

    req_number VARCHAR(50),

    description TEXT,

    design_code VARCHAR(100),

    fabric_code VARCHAR(100),

    remarks TEXT,

    status VARCHAR(20) NOT NULL DEFAULT 'active',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_wso_category
        FOREIGN KEY (category_id)
        REFERENCES categories(id)
        ON DELETE SET NULL
);

-- ==========================================
-- WSO Line Items
-- ==========================================

CREATE TABLE wso_line_items (
    id SERIAL PRIMARY KEY,

    wso_order_id INTEGER NOT NULL,

    size VARCHAR(20) NOT NULL,

    qty_raised INTEGER NOT NULL,

    qty_received INTEGER NOT NULL DEFAULT 0,

    received_date DATE,

    status VARCHAR(30) NOT NULL DEFAULT 'Raised',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_wso_order
        FOREIGN KEY (wso_order_id)
        REFERENCES wso_orders(id)
        ON DELETE CASCADE
);

-- ==========================================
-- Helpful Indexes
-- ==========================================

CREATE INDEX idx_wso_number
ON wso_orders(wso_number);

CREATE INDEX idx_req_number
ON wso_orders(req_number);

CREATE INDEX idx_category
ON wso_orders(category_id);

CREATE INDEX idx_wso_status
ON wso_orders(status);

CREATE INDEX idx_line_item_wso
ON wso_line_items(wso_order_id);

CREATE INDEX idx_line_item_status
ON wso_line_items(status);