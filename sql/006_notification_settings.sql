-- ============================================================
-- Notification Settings
-- ============================================================

CREATE TABLE IF NOT EXISTS notification_settings (

    id SERIAL PRIMARY KEY,

    notification_event_id INTEGER NOT NULL UNIQUE
        REFERENCES notification_events(id)
        ON DELETE CASCADE,

    enabled BOOLEAN NOT NULL DEFAULT TRUE,

    email_enabled BOOLEAN NOT NULL DEFAULT TRUE,

    in_app_enabled BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMP NOT NULL DEFAULT NOW()

);

-- ============================================================
-- Create default settings for every event
-- ============================================================

INSERT INTO notification_settings
(
    notification_event_id
)

SELECT id
FROM notification_events

ON CONFLICT (notification_event_id)
DO NOTHING;