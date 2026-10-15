-- ============================================================
-- Schema drift fix
-- ============================================================
--
-- These columns are used by the application code (models and
-- repositories) but were never created by an earlier migration --
-- they were evidently added by hand to the original development
-- database. A database built purely from sql/ was missing them,
-- which surfaced as: column "created_by" does not exist.
--
-- Every statement is idempotent (IF NOT EXISTS), so this is safe
-- to run on a database that already has the columns.
-- ============================================================

-- WSO items: nullable, matches WsoItem.created_by: Option<String>
ALTER TABLE wso_items
ADD COLUMN IF NOT EXISTS created_by TEXT;

-- Purchase orders / PO items: NOT NULL in the models. A temporary
-- default lets the column be added to tables that already have
-- rows; it's dropped straight after so new rows must always supply
-- the real user.
ALTER TABLE purchase_orders
ADD COLUMN IF NOT EXISTS created_by TEXT NOT NULL DEFAULT 'System';

ALTER TABLE purchase_orders
ALTER COLUMN created_by DROP DEFAULT;

ALTER TABLE po_items
ADD COLUMN IF NOT EXISTS created_by TEXT NOT NULL DEFAULT 'System';

ALTER TABLE po_items
ALTER COLUMN created_by DROP DEFAULT;

-- Notification jobs: sender identity stored on each queued email.
ALTER TABLE notification_jobs
ADD COLUMN IF NOT EXISTS sender_name VARCHAR(255) NOT NULL DEFAULT '';

ALTER TABLE notification_jobs
ALTER COLUMN sender_name DROP DEFAULT;

ALTER TABLE notification_jobs
ADD COLUMN IF NOT EXISTS sender_email VARCHAR(255) NOT NULL DEFAULT '';

ALTER TABLE notification_jobs
ALTER COLUMN sender_email DROP DEFAULT;