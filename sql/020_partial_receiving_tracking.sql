-- ============================================================
-- Partial Receiving Attention Tracking
-- ============================================================
--
-- Tracks periods where a WSO product has been partially
-- received but still has an outstanding quantity.
--
-- This is deliberately separate from wso_line_items.
-- wso_line_items records what was raised/received.
-- This table records when an unresolved partial receipt
-- became an attention item.
-- ============================================================

CREATE TABLE IF NOT EXISTS partial_receiving_tracking (

    id SERIAL PRIMARY KEY,

    wso_item_id INTEGER NOT NULL
        REFERENCES wso_items(id)
        ON DELETE CASCADE,

    first_partial_received_at TIMESTAMPTZ NOT NULL,

    notification_sent_at TIMESTAMPTZ,

    resolved_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS
idx_partial_receiving_tracking_item
ON partial_receiving_tracking(wso_item_id);

CREATE INDEX IF NOT EXISTS
idx_partial_receiving_tracking_active
ON partial_receiving_tracking(wso_item_id)
WHERE resolved_at IS NULL;

CREATE INDEX IF NOT EXISTS
idx_partial_receiving_tracking_notification
ON partial_receiving_tracking(notification_sent_at);

CREATE UNIQUE INDEX IF NOT EXISTS
uq_partial_receiving_tracking_active_item
ON partial_receiving_tracking(wso_item_id)
WHERE resolved_at IS NULL;