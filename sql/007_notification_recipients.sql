CREATE TABLE IF NOT EXISTS notification_recipients (

    id SERIAL PRIMARY KEY,

    notification_event_id INTEGER NOT NULL
        REFERENCES notification_events(id)
        ON DELETE CASCADE,

    display_name VARCHAR(150) NOT NULL,

    email VARCHAR(255) NOT NULL,

    enabled BOOLEAN NOT NULL DEFAULT TRUE,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    updated_at TIMESTAMP NOT NULL DEFAULT NOW()

);

INSERT INTO notification_recipients
(notification_event_id, display_name, email)

SELECT
    id,
    'Retail Stores',
    'retail-stores@image-first.biz'
FROM notification_events;