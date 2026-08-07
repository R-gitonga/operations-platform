-- ============================================================
-- 014_reset_operational_data.sql
--
-- Clears all existing WSO/product operational data so the
-- notification system can be tested from a clean dataset.
--
-- Configuration is intentionally preserved.
-- ============================================================

BEGIN;

------------------------------------------------------------
-- WSO / Product operational data
------------------------------------------------------------

TRUNCATE TABLE
    wso_stage_history,
    wso_line_items,
    wso_items,
    wso_orders
RESTART IDENTITY
CASCADE;

------------------------------------------------------------
-- Notification test history
--
-- These records are operational history generated from
-- previous test activity, so clear them as well.
------------------------------------------------------------

TRUNCATE TABLE
    notification_jobs,
    notification_logs
RESTART IDENTITY
CASCADE;

COMMIT;