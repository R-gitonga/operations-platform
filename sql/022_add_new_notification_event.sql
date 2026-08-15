INSERT INTO notification_events
(
    code,
    display_name,
    description
)
VALUES
(
    'partial_receiving_attention',
    'Partial Receiving Attention',
    'Raised when a product remains partially received beyond the configured attention threshold.'
)
ON CONFLICT (code) DO NOTHING;

INSERT INTO notification_settings
(
    notification_event_id
)
SELECT id
FROM notification_events
WHERE code = 'partial_receiving_attention'
ON CONFLICT (notification_event_id)
DO NOTHING;