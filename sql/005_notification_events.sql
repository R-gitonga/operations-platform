-- ============================================================
-- Notification Events
-- ============================================================

CREATE TABLE IF NOT EXISTS notification_events (

    id SERIAL PRIMARY KEY,

    code VARCHAR(100) NOT NULL UNIQUE,

    display_name VARCHAR(255) NOT NULL,

    description TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT NOW()

);

-- ============================================================
-- Seed default notification events
-- ============================================================

INSERT INTO notification_events
(code, display_name, description)

VALUES

(
'wso_created',
'Workshop Order Created',
'Raised whenever a new workshop order is created.'
),

(
'wso_completed',
'Workshop Order Completed',
'Raised when every line item has been fully received.'
),

(
'wso_cancelled',
'Workshop Order Cancelled',
'Raised when a workshop order is cancelled.'
),

(
'wso_reactivated',
'Workshop Order Reactivated',
'Raised when a cancelled workshop order is restored.'
),

(
'attachment_uploaded',
'Attachment Uploaded',
'Raised whenever a document is attached to a workshop order.'
)

ON CONFLICT (code) DO NOTHING;