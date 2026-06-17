CREATE TABLE wso_orders (
    id SERIAL PRIMARY KEY,

    wso_number VARCHAR(50) NOT NULL UNIQUE,

    req_number VARCHAR(50),

    description TEXT,

    remarks TEXT,

    status VARCHAR(20) NOT NULL DEFAULT 'active',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);