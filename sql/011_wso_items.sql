BEGIN;

------------------------------------------------------------
-- WSO Items
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS wso_items (

    id                  SERIAL PRIMARY KEY,

    wso_order_id        INTEGER NOT NULL,

    category_id         INTEGER,

    description         TEXT,

    design_code         TEXT,

    fabric_code         TEXT,

    branding_required   BOOLEAN NOT NULL DEFAULT FALSE,

    branding_completed  BOOLEAN NOT NULL DEFAULT FALSE,

    current_stage_id    INTEGER,

    created_at          TIMESTAMP NOT NULL DEFAULT NOW(),

    updated_at          TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_wso_items_order
        FOREIGN KEY (wso_order_id)
        REFERENCES wso_orders(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_wso_items_stage
        FOREIGN KEY (current_stage_id)
        REFERENCES production_stages(id)
);

------------------------------------------------------------
-- Line Items now belong to WSO Items
------------------------------------------------------------

ALTER TABLE wso_line_items

ADD COLUMN IF NOT EXISTS wso_item_id INTEGER;

ALTER TABLE wso_line_items

ADD CONSTRAINT fk_line_items_item

FOREIGN KEY (wso_item_id)

REFERENCES wso_items(id);

------------------------------------------------------------
-- Stage History now belongs to WSO Items
------------------------------------------------------------

ALTER TABLE wso_stage_history

ADD COLUMN IF NOT EXISTS wso_item_id INTEGER;

ALTER TABLE wso_stage_history

ADD CONSTRAINT fk_stage_history_item

FOREIGN KEY (wso_item_id)

REFERENCES wso_items(id);

------------------------------------------------------------
-- Create one default Item for every existing WSO
------------------------------------------------------------

INSERT INTO wso_items
(
    wso_order_id,
    category_id,
    description,
    design_code,
    fabric_code,
    current_stage_id
)
SELECT
    id,
    category_id,
    description,
    design_code,
    fabric_code,
    current_stage_id
FROM wso_orders
WHERE NOT EXISTS
(
    SELECT 1
    FROM wso_items i
    WHERE i.wso_order_id = wso_orders.id
);

------------------------------------------------------------
-- Link existing line items
------------------------------------------------------------

UPDATE wso_line_items li
SET wso_item_id = wi.id
FROM wso_items wi
WHERE
    li.wso_order_id = wi.wso_order_id
AND li.wso_item_id IS NULL;

------------------------------------------------------------
-- Link existing stage history
------------------------------------------------------------

UPDATE wso_stage_history sh
SET wso_item_id = wi.id
FROM wso_items wi
WHERE
    sh.wso_id = wi.wso_order_id
AND sh.wso_item_id IS NULL;

------------------------------------------------------------
-- Helpful indexes
------------------------------------------------------------

CREATE INDEX IF NOT EXISTS idx_wso_items_order
ON wso_items(wso_order_id);

CREATE INDEX IF NOT EXISTS idx_line_items_item
ON wso_line_items(wso_item_id);

CREATE INDEX IF NOT EXISTS idx_stage_history_item
ON wso_stage_history(wso_item_id);

COMMIT;