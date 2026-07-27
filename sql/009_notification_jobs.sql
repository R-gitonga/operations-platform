CREATE TABLE IF NOT EXISTS notification_jobs (

    id SERIAL PRIMARY KEY,

    notification_log_id INTEGER NOT NULL
        REFERENCES notification_logs(id)
        ON DELETE CASCADE,

    recipient_email VARCHAR(255) NOT NULL,

    subject TEXT NOT NULL,

    html_body TEXT NOT NULL,

    status VARCHAR(20) NOT NULL DEFAULT 'pending',

    attempts INTEGER NOT NULL DEFAULT 0,

    error_message TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    processed_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS
idx_notification_jobs_status

ON notification_jobs(status);