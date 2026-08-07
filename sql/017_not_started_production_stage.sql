BEGIN;

------------------------------------------------------------
-- Add the Not Started production stage seed
------------------------------------------------------------

ALTER TABLE production_stages
ADD COLUMN IF NOT EXISTS expected_duration_hours INTEGER;

ALTER TABLE production_stages
ADD COLUMN IF NOT EXISTS attention_enabled BOOLEAN
NOT NULL DEFAULT TRUE;

INSERT INTO production_stages (
    code,
    display_name,
    display_order,
    color,
    active,
    expected_duration_hours,
    attention_enabled
)
VALUES (
    'NOT_STARTED',
    'Not Started',
    0,
    '#64748b',
    TRUE,
    24,
    TRUE
)
ON CONFLICT (code)
DO NOTHING;

COMMIT;
