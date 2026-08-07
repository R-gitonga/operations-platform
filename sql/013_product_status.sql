BEGIN;

------------------------------------------------------------
-- Product Status
------------------------------------------------------------

ALTER TABLE wso_items

ADD COLUMN IF NOT EXISTS status TEXT NOT NULL DEFAULT 'Open';

CREATE INDEX IF NOT EXISTS idx_wso_items_status
ON wso_items(status);

COMMIT;