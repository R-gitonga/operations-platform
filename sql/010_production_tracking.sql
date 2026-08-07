BEGIN;

------------------------------------------------------------
-- Production Stages
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS production_stages (

    id              SERIAL PRIMARY KEY,

    code            TEXT NOT NULL UNIQUE,

    display_name    TEXT NOT NULL,

    display_order   INTEGER NOT NULL,

    color           TEXT NOT NULL DEFAULT '#2563eb',

    active          BOOLEAN NOT NULL DEFAULT TRUE,

    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),

    updated_at      TIMESTAMP NOT NULL DEFAULT NOW()
);

------------------------------------------------------------
-- Seed Default Stages
------------------------------------------------------------

INSERT INTO production_stages
(
    code,
    display_name,
    display_order,
    color
)
VALUES
('cad_room',           'CAD Room',             1, 'rgb(124, 58, 237)'),
('fabric_allocation',  'Fabric Allocation',    2, 'rgb(37, 99, 235)'),
('cutting',            'Cutting',              3, 'rgb(234, 88, 12)'),
('embroidery',         'Embroidery',           4, '#db2777'),
('printing',           'Printing',             5, '#0891b2'),
('production',         'Production Floor',     6, '#16a34a'),
('finishing',          'Finishing',            7, '#65a30d'),
('quality_control',    'Quality Control',      8, '#ca8a04'),
('packing',            'Packing',              9, '#0f766e'),
('dispatch',           'Ready For Dispatch',  10, '#1d4ed8')

ON CONFLICT (code)
DO NOTHING;

------------------------------------------------------------
-- Current Stage On WSO
------------------------------------------------------------

ALTER TABLE wso_orders

ADD COLUMN IF NOT EXISTS current_stage_id INTEGER;

ALTER TABLE wso_orders

ADD CONSTRAINT fk_wsos_current_stage

FOREIGN KEY (current_stage_id)

REFERENCES production_stages(id);

------------------------------------------------------------
-- Stage History
------------------------------------------------------------

CREATE TABLE IF NOT EXISTS wso_stage_history (

    id                  SERIAL PRIMARY KEY,

    wso_id              INTEGER NOT NULL,

    production_stage_id INTEGER NOT NULL,

    notes               TEXT,

    changed_by          TEXT NOT NULL,

    changed_at          TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_stage_history_wso

        FOREIGN KEY (wso_id)

        REFERENCES wsos(id)

        ON DELETE CASCADE,

    CONSTRAINT fk_stage_history_stage

        FOREIGN KEY (production_stage_id)

        REFERENCES production_stages(id)
);

------------------------------------------------------------
-- Helpful Indexes
------------------------------------------------------------

CREATE INDEX IF NOT EXISTS idx_wsos_current_stage

ON wsos(current_stage_id);

CREATE INDEX IF NOT EXISTS idx_stage_history_wso

ON wso_stage_history(wso_id);

CREATE INDEX IF NOT EXISTS idx_stage_history_date

ON wso_stage_history(changed_at DESC);

COMMIT;