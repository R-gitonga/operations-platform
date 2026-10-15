CREATE TABLE notification_events (
    id SERIAL PRIMARY KEY,
    code VARCHAR(100) NOT NULL UNIQUE,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    module VARCHAR(20) NOT NULL DEFAULT 'wso',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE notification_settings (
    id SERIAL PRIMARY KEY,
    notification_event_id INTEGER NOT NULL UNIQUE REFERENCES notification_events(id) ON DELETE CASCADE,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    email_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    in_app_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE notification_recipients (
    id SERIAL PRIMARY KEY,
    notification_event_id INTEGER NOT NULL REFERENCES notification_events(id) ON DELETE CASCADE,
    display_name VARCHAR(150) NOT NULL,
    email VARCHAR(255) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX notification_recipients_event_idx ON notification_recipients(notification_event_id);
CREATE TABLE notification_logs (
    id SERIAL PRIMARY KEY,
    notification_event_id INTEGER NOT NULL REFERENCES notification_events(id),
    recipient_email VARCHAR(255) NOT NULL,
    channel VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL CHECK (status IN ('pending', 'sent', 'failed')),
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    sent_at TIMESTAMPTZ
);
CREATE INDEX notification_logs_event_idx ON notification_logs(notification_event_id);
CREATE INDEX notification_logs_created_idx ON notification_logs(created_at DESC);
CREATE TABLE notification_jobs (
    id SERIAL PRIMARY KEY,
    notification_log_id INTEGER NOT NULL REFERENCES notification_logs(id) ON DELETE CASCADE,
    sender_name VARCHAR(255) NOT NULL,
    sender_email VARCHAR(255) NOT NULL,
    recipient_email VARCHAR(255) NOT NULL,
    subject TEXT NOT NULL,
    html_body TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'sent', 'failed')),
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    processed_at TIMESTAMPTZ
);
CREATE INDEX notification_jobs_log_idx ON notification_jobs(notification_log_id);
CREATE INDEX notification_jobs_pending_idx ON notification_jobs(created_at) WHERE status = 'pending';
CREATE TABLE attention_required_notifications (
    id SERIAL PRIMARY KEY,
    wso_item_id INTEGER NOT NULL REFERENCES wso_items(id) ON DELETE CASCADE,
    production_stage_id INTEGER NOT NULL REFERENCES production_stages(id) ON DELETE CASCADE,
    stage_started_at TIMESTAMPTZ NOT NULL,
    notification_event_id INTEGER NOT NULL REFERENCES notification_events(id) ON DELETE RESTRICT,
    notified_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT attention_required_occurrence_unique UNIQUE (wso_item_id, production_stage_id, stage_started_at)
);
CREATE INDEX attention_required_stage_idx ON attention_required_notifications(production_stage_id);
