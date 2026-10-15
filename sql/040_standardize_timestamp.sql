
-- File: migrations/040_standardize_timestamps.sql
-- Standardize event timestamps across Operations Platform.
--
-- Existing TIMESTAMP values are interpreted as UTC.
-- Appropriate for a fresh/reset database.
-- Run after migrations 001–039.

BEGIN;

-- Notification events
ALTER TABLE notification_events
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
        USING created_at AT TIME ZONE 'UTC';

-- Notification jobs
ALTER TABLE notification_jobs
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN processed_at TYPE TIMESTAMPTZ
        USING processed_at AT TIME ZONE 'UTC';

-- Notification logs
ALTER TABLE notification_logs
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN sent_at TYPE TIMESTAMPTZ
        USING sent_at AT TIME ZONE 'UTC';

-- Notification recipients
ALTER TABLE notification_recipients
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ
        USING updated_at AT TIME ZONE 'UTC';

-- Notification settings
ALTER TABLE notification_settings
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ
        USING updated_at AT TIME ZONE 'UTC';

-- Production stages
ALTER TABLE production_stages
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ
        USING updated_at AT TIME ZONE 'UTC';

-- WSO items
ALTER TABLE wso_items
    ALTER COLUMN created_at TYPE TIMESTAMPTZ
        USING created_at AT TIME ZONE 'UTC',
    ALTER COLUMN updated_at TYPE TIMESTAMPTZ
        USING updated_at AT TIME ZONE 'UTC';

-- WSO stage history
ALTER TABLE wso_stage_history
    ALTER COLUMN changed_at TYPE TIMESTAMPTZ
        USING changed_at AT TIME ZONE 'UTC';

COMMIT;
