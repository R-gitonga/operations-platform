CREATE TABLE wso_line_items (
    id SERIAL PRIMARY KEY,

    wso_order_id INTEGER NOT NULL,

    size VARCHAR(20) NOT NULL,

    quantity INTEGER NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_wso_order
        FOREIGN KEY (wso_order_id)
        REFERENCES wso_orders(id)
        ON DELETE CASCADE
);