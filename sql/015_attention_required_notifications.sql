-- ============================================================
-- Attention Required Notification Tracking
-- ============================================================
--
-- Records each product/stage occurrence for which the initial
-- attention-required notification has already been dispatched.
--
-- A new occurrence is created whenever the product enters a
-- different stage, because stage_started_at changes.
--
-- This prevents dashboard refreshes/polling from repeatedly
-- sending the same notification.
-- ============================================================

CREATE TABLE IF NOT EXISTS attention_required_notifications (

    id SERIAL PRIMARY KEY,

    wso_item_id INTEGER NOT NULL
        REFERENCES wso_items(id)
        ON DELETE CASCADE,

    production_stage_id INTEGER NOT NULL
        REFERENCES production_stages(id)
        ON DELETE CASCADE,

    stage_started_at TIMESTAMP WITH TIME ZONE NOT NULL,

    notification_event_id INTEGER NOT NULL
        REFERENCES notification_events(id)
        ON DELETE RESTRICT,

    notified_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_attention_required_notification_occurrence
        UNIQUE (
            wso_item_id,
            production_stage_id,
            stage_started_at
        )
);

CREATE INDEX IF NOT EXISTS
idx_attention_required_notifications_item
ON attention_required_notifications(wso_item_id);

CREATE INDEX IF NOT EXISTS
idx_attention_required_notifications_stage
ON attention_required_notifications(production_stage_id);

CREATE INDEX IF NOT EXISTS
idx_attention_required_notifications_notified
ON attention_required_notifications(notified_at DESC);