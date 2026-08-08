BEGIN;

------------------------------------------------------------
-- Product fully received notification event
------------------------------------------------------------

INSERT INTO notification_events
(
    code,
    display_name,
    description
)
VALUES
(
    'product_fully_received',
    'Product Fully Received',
    'Raised when all line items for a product have been fully received.'
)
ON CONFLICT (code) DO NOTHING;

INSERT INTO notification_settings
(
    notification_event_id
)
SELECT id
FROM notification_events
WHERE code = 'product_fully_received'
ON CONFLICT (notification_event_id)
DO NOTHING;

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
WHERE code = 'product_fully_received'
  AND NOT EXISTS (
      SELECT 1
      FROM notification_recipients nr
      WHERE nr.notification_event_id = notification_events.id
        AND LOWER(nr.email) = LOWER('retail-stores@image-first.biz')
  );

COMMIT;
