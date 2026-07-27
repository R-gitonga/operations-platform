CREATE TABLE notification_logs (

    id SERIAL PRIMARY KEY,

    notification_event_id INTEGER NOT NULL
        REFERENCES notification_events(id),

    recipient_email VARCHAR(255) NOT NULL,

    channel VARCHAR(50) NOT NULL,

    status VARCHAR(50) NOT NULL,

    error_message TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    sent_at TIMESTAMP
);

CREATE INDEX idx_notification_logs_event
ON notification_logs(notification_event_id);

CREATE INDEX idx_notification_logs_status
ON notification_logs(status);

CREATE INDEX idx_notification_logs_created
ON notification_logs(created_at DESC);