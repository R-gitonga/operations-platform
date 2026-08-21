-- ============================================================
-- WSO Partial Receipt Events
-- ============================================================
--
-- One row per individual "receive" transaction against a
-- WSO line item. This is intentionally kept separate from
-- wso_stage_history: several existing queries treat the most
-- recent wso_stage_history row as "when did this item's
-- current production stage start" (used for stage-overdue
-- calculations), and a receiving event is not a stage change.
--
-- Read alongside wso_stage_history (via UNION) to build the
-- Production Timeline and the Recent Production Activity feed.
-- ============================================================

CREATE TABLE IF NOT EXISTS wso_partial_receipt_events (

    id SERIAL PRIMARY KEY,

    wso_item_id INTEGER NOT NULL
        REFERENCES wso_items(id),

    line_item_id INTEGER NOT NULL
        REFERENCES wso_line_items(id),

    quantity_received INTEGER NOT NULL,

    total_raised INTEGER NOT NULL,

    total_received INTEGER NOT NULL,

    balance INTEGER NOT NULL,

    received_by VARCHAR(255) NOT NULL,

    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

);

CREATE INDEX IF NOT EXISTS
idx_wso_partial_receipt_events_item
ON wso_partial_receipt_events(wso_item_id, received_at);