
-- File: sql/041_fix_wso_stage_history_relationship.sql
--
-- WSO production history is now associated with
-- individual WSO items, not directly with WSO orders.
--
-- Remove the legacy wso_id column while ensuring that
-- all existing history records belong to an item.

BEGIN;

-- Safety check: refuse to proceed if legacy history
-- records have no associated WSO item.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM wso_stage_history
        WHERE wso_item_id IS NULL
    ) THEN
        RAISE EXCEPTION
            'Cannot migrate: stage history contains records without a WSO item';
    END IF;
END $$;

-- Make the current relationship mandatory.
ALTER TABLE wso_stage_history
    ALTER COLUMN wso_item_id SET NOT NULL;

-- Remove the obsolete order-level relationship.
-- PostgreSQL automatically removes the associated
-- foreign-key constraint and column-dependent index.
ALTER TABLE wso_stage_history
    DROP COLUMN wso_id;

COMMIT;
