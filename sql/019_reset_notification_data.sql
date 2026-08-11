BEGIN;

-- ============================================================
-- 1. Remove queued/processed email jobs
-- ============================================================

DELETE FROM notification_jobs;


-- ============================================================
-- 2. Remove notification history
-- ============================================================

DELETE FROM notification_logs;


-- ============================================================
-- 3. Remove all notification recipients
-- ============================================================
-- This removes seeded recipients such as:
-- Retail Stores <retail-stores@image-first.biz>
-- as well as any manually-created test recipients.

DELETE FROM notification_recipients;


-- ============================================================
-- 4. Reset attention-required notification tracking
-- ============================================================

DELETE FROM attention_required_notifications;


-- ============================================================
-- 5. Reset notification settings to application defaults
-- ============================================================
-- Keep one settings row for every notification event.
-- The application expects these rows to exist.
--
-- Defaults:
-- enabled       = TRUE
-- email_enabled = TRUE
-- in_app_enabled = TRUE

UPDATE notification_settings
SET
    enabled = TRUE,
    email_enabled = TRUE,
    in_app_enabled = TRUE,
    updated_at = NOW();


COMMIT;