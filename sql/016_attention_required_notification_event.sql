-- ============================================================
-- Attention Required Notification Event
-- ============================================================

INSERT INTO notification_events
(
    code,
    display_name,
    description
)
VALUES
(
    'attention_required',
    'Product Attention Required',
    'Raised when a product remains in a production stage beyond its expected duration.'
)
ON CONFLICT (code) DO NOTHING;


-- ============================================================
-- Create default notification settings
-- ============================================================

INSERT INTO notification_settings
(
    notification_event_id
)
SELECT id
FROM notification_events
WHERE code = 'attention_required'

ON CONFLICT (notification_event_id)
DO NOTHING;


-- ============================================================
-- Add default Retail Stores recipient
-- ============================================================
--
-- This follows the pattern used by migration 007.
-- Additional recipients can still be configured through the
-- notification recipient management functionality.
-- ============================================================

INSERT INTO notification_recipients
(
    notification_event_id,
    display_name,
    email
)
SELECT
    id,
    'Retail Stores',
    'retail-stores@image-first.biz'
FROM notification_events
WHERE code = 'attention_required'
  AND NOT EXISTS (
      SELECT 1
      FROM notification_recipients nr
      WHERE nr.notification_event_id = notification_events.id
        AND LOWER(nr.email) = LOWER('retail-stores@image-first.biz')
  );