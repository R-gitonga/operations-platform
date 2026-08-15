CREATE TABLE IF NOT EXISTS partial_receiving_settings (

    id INTEGER PRIMARY KEY DEFAULT 1,

    attention_after_days INTEGER NOT NULL DEFAULT 3,

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_partial_receiving_attention_days
        CHECK (attention_after_days >= 0)
);INSERT INTO partial_receiving_settings
(
    id,
    attention_after_days
)
VALUES
(
    1,
    3
)
ON CONFLICT (id) DO NOTHING;