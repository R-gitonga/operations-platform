-- ============================================================
-- Notification Events: module grouping
-- ============================================================
--
-- Existing rows are all WSO events, so the default backfills
-- them correctly. New events (WSO or PO) should specify module
-- explicitly going forward; the default only exists to make this
-- migration safe against existing data.
-- ============================================================

ALTER TABLE notification_events
ADD COLUMN IF NOT EXISTS module VARCHAR(20) NOT NULL DEFAULT 'wso';