-- ============================================================
-- PO Settings (singleton)
-- ============================================================
--
-- Same shape as partial_receiving_settings -- one row (id = 1),
-- read and updated in place. Holds every PO threshold the
-- notification sweeps need, rather than one table per knob.
-- ============================================================

CREATE TABLE IF NOT EXISTS po_settings (

    id SERIAL PRIMARY KEY,

    -- Days of grace after expected_delivery_date before an item
    -- counts as overdue. 0 means overdue the day after the date.
    overdue_grace_days INTEGER NOT NULL DEFAULT 0,

    -- Days before expected_delivery_date that "approaching
    -- deadline" starts firing.
    approaching_window_days INTEGER NOT NULL DEFAULT 2,

    -- Days since the last receipt on a partially-received item
    -- before it's considered stalled.
    stalled_after_days INTEGER NOT NULL DEFAULT 3,

    -- Days an open defect can sit unresolved before it's flagged.
    unresolved_defect_after_days INTEGER NOT NULL DEFAULT 3,

    -- How often the background worker re-runs the PO alert
    -- sweeps. Deadlines are date-based, so hourly is the default
    -- rather than the 10-second job-queue tick.
    alert_sweep_interval_minutes INTEGER NOT NULL DEFAULT 60,

    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()

);

INSERT INTO po_settings (id)
VALUES (1)
ON CONFLICT (id) DO NOTHING;