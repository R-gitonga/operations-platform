-- Durable PO occurrences. Deferred triggers observe the final transaction state,
-- including all lines of a receipt, and roll back with the business operation.
CREATE TABLE po_notification_occurrences (
    id BIGSERIAL PRIMARY KEY,
    event_code TEXT NOT NULL REFERENCES notification_events(code),
    purchase_order_id INTEGER NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
    occurrence_key TEXT NOT NULL UNIQUE,
    payload JSONB NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending','queued','suppressed')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX po_notification_pending_idx ON po_notification_occurrences(id) WHERE state='pending';
CREATE TABLE po_notification_states (
    state_key TEXT PRIMARY KEY,
    purchase_order_id INTEGER NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
    active BOOLEAN NOT NULL,
    episode BIGINT NOT NULL DEFAULT 0
);
ALTER TABLE notification_jobs DROP CONSTRAINT notification_jobs_status_check;
ALTER TABLE notification_jobs ADD CONSTRAINT notification_jobs_status_check
    CHECK (status IN ('pending','processing','retry','sent','failed','suppressed'));
ALTER TABLE notification_logs DROP CONSTRAINT notification_logs_status_check;
ALTER TABLE notification_logs ADD CONSTRAINT notification_logs_status_check
    CHECK (status IN ('pending','processing','retry','sent','failed','suppressed'));
ALTER TABLE notification_jobs ADD COLUMN occurrence_id BIGINT REFERENCES po_notification_occurrences(id) ON DELETE CASCADE;
ALTER TABLE notification_jobs ADD COLUMN available_at TIMESTAMPTZ NOT NULL DEFAULT now();
ALTER TABLE notification_jobs ADD COLUMN lease_until TIMESTAMPTZ;
CREATE UNIQUE INDEX notification_jobs_occurrence_recipient_idx
    ON notification_jobs(occurrence_id, lower(btrim(recipient_email))) WHERE occurrence_id IS NOT NULL;
CREATE INDEX notification_jobs_available_idx ON notification_jobs(available_at) WHERE status IN ('pending','retry','processing');

ALTER TABLE po_items ADD COLUMN notification_deadline_version BIGINT NOT NULL DEFAULT 0;
CREATE FUNCTION po_email_deadline_revision() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.expected_delivery_date IS DISTINCT FROM OLD.expected_delivery_date THEN
        NEW.notification_deadline_version:=OLD.notification_deadline_version+1;
    END IF;
    RETURN NEW;
END $$;
CREATE TRIGGER po_email_deadline_revision BEFORE UPDATE ON po_items
    FOR EACH ROW EXECUTE FUNCTION po_email_deadline_revision();

CREATE VIEW po_notification_line_totals AS
SELECT li.id, li.po_item_id, li.qty_ordered,
    COALESCE(r.delivered,0) AS delivered,
    COALESCE(r.delivered,0)-COALESCE(d.defective,0) AS accepted,
    r.last_received_at
FROM po_line_items li
LEFT JOIN LATERAL (
    SELECT sum(rl.qty_delivered) AS delivered, max(pr.received_at) AS last_received_at
    FROM po_receipt_lines rl JOIN po_receipts pr ON pr.id=rl.po_receipt_id
    WHERE rl.po_line_item_id=li.id
) r ON true
LEFT JOIN LATERAL (
    SELECT sum(qty_defective) AS defective FROM po_defects
    WHERE po_line_item_id=li.id AND resolution_type IS DISTINCT FROM 'accepted'
) d ON true;

CREATE FUNCTION po_email_snapshot(order_id INTEGER, item_id INTEGER DEFAULT NULL)
RETURNS JSONB LANGUAGE sql STABLE AS $$
    SELECT jsonb_build_object('purchase_order_id',p.id,'erp_reference',p.erp_reference,
        'accounts_reference',p.accounts_reference,'supplier',s.name,'created_by',p.created_by,
        'description',p.description,'item_id',item_id,'actor',nullif(current_setting('app.actor',true),''),
        'items',COALESCE((SELECT jsonb_agg(jsonb_build_object(
            'id',i.id,'description',i.description,'expected_delivery_date',i.expected_delivery_date,
            'branding_required',i.branding_required,
            'branding_type',(SELECT display_name FROM branding_types WHERE id=i.branding_type_id),
            'branding_location',(SELECT display_name FROM branding_locations WHERE id=i.branding_location_id),
            'lines',(SELECT jsonb_agg(jsonb_build_object('size',li.size,'ordered',t.qty_ordered,
                'delivered',t.delivered,'accepted',t.accepted,'outstanding',greatest(t.qty_ordered-t.accepted,0)))
                FROM po_notification_line_totals t JOIN po_line_items li ON li.id=t.id WHERE t.po_item_id=i.id)))
            FROM po_items i WHERE i.purchase_order_id=p.id AND (item_id IS NULL OR i.id=item_id)), '[]'::jsonb))
    FROM purchase_orders p JOIN suppliers s ON s.id=p.supplier_id WHERE p.id=order_id
$$;

CREATE FUNCTION po_email_emit(code TEXT, order_id INTEGER, key TEXT, data JSONB)
RETURNS void LANGUAGE sql AS $$
    INSERT INTO po_notification_occurrences(event_code,purchase_order_id,occurrence_key,payload)
    VALUES(code,order_id,key,data) ON CONFLICT(occurrence_key) DO NOTHING
$$;

CREATE FUNCTION po_email_transition(code TEXT, order_id INTEGER, key TEXT, qualifies BOOLEAN, data JSONB, baseline BOOLEAN DEFAULT false)
RETURNS void LANGUAGE plpgsql AS $$
DECLARE previous po_notification_states; next_episode BIGINT;
BEGIN
    INSERT INTO po_notification_states(state_key,purchase_order_id,active)
        VALUES(key,order_id,false) ON CONFLICT DO NOTHING;
    SELECT * INTO previous FROM po_notification_states WHERE state_key=key FOR UPDATE;
    IF qualifies AND NOT previous.active THEN
        next_episode := previous.episode+1;
        IF NOT baseline THEN
            PERFORM po_email_emit(code,order_id,key||':'||next_episode,data);
        END IF;
        UPDATE po_notification_states SET active=true,episode=next_episode WHERE state_key=key;
    ELSIF NOT qualifies THEN
        UPDATE po_notification_states SET active=false WHERE state_key=key;
    END IF;
END $$;

CREATE FUNCTION po_email_reconcile(order_id INTEGER, baseline BOOLEAN DEFAULT false)
RETURNS void LANGUAGE plpgsql AS $$
DECLARE p purchase_orders; item_state RECORD; complete BOOLEAN;
BEGIN
    SELECT * INTO p FROM purchase_orders WHERE id=order_id FOR NO KEY UPDATE;
    IF NOT FOUND THEN RETURN; END IF;
    -- Per-line qualification prevents an over-delivered size hiding a shortage.
    SELECT count(*)>0 AND bool_and(t.accepted>=t.qty_ordered) INTO complete
    FROM po_notification_line_totals t JOIN po_items i ON i.id=t.po_item_id WHERE i.purchase_order_id=order_id;
    PERFORM po_email_transition('po_completed',order_id,'completed:'||order_id,
        p.status!='cancelled' AND complete,po_email_snapshot(order_id),baseline);
    FOR item_state IN SELECT pi.id,pi.branding_required,count(t.id)>0 AND bool_and(t.accepted>=t.qty_ordered) AS ready
        FROM po_items pi LEFT JOIN po_notification_line_totals t ON t.po_item_id=pi.id
        WHERE pi.purchase_order_id=order_id GROUP BY pi.id LOOP
        PERFORM po_email_transition('po_branding_goods_ready',order_id,'branding:'||item_state.id,
            p.status!='cancelled' AND item_state.branding_required AND item_state.ready,po_email_snapshot(order_id,item_state.id),baseline);
    END LOOP;
END $$;

CREATE FUNCTION po_email_capture() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE order_id INTEGER; item_id INTEGER; code TEXT; data JSONB; row_data JSONB;
BEGIN
    row_data := CASE WHEN TG_OP='DELETE' THEN to_jsonb(OLD) ELSE to_jsonb(NEW) END;
    IF TG_TABLE_NAME='purchase_orders' THEN
        order_id := (row_data->>'id')::integer;
        IF TG_OP='INSERT' THEN code:='po_created';
        ELSIF OLD.status IS DISTINCT FROM NEW.status THEN
            code:=CASE WHEN NEW.status='cancelled' THEN 'po_cancelled' ELSE 'po_reactivated' END;
        END IF;
    ELSIF TG_TABLE_NAME='po_items' THEN order_id:=(row_data->>'purchase_order_id')::integer;
    ELSE
        IF TG_TABLE_NAME='po_line_items' THEN item_id:=(row_data->>'po_item_id')::integer;
        ELSE SELECT po_item_id INTO item_id FROM po_line_items WHERE id=(row_data->>'po_line_item_id')::integer;
        END IF;
        SELECT purchase_order_id INTO order_id FROM po_items WHERE id=item_id;
        IF TG_TABLE_NAME='po_defects' AND TG_OP='INSERT' THEN code:='po_defect_reported'; END IF;
    END IF;
    IF order_id IS NULL OR NOT EXISTS(SELECT 1 FROM purchase_orders WHERE id=order_id) THEN RETURN NULL; END IF;
    PERFORM 1 FROM purchase_orders WHERE id=order_id FOR NO KEY UPDATE;
    IF code IS NOT NULL THEN
        data:=po_email_snapshot(order_id,item_id);
        IF TG_TABLE_NAME='po_defects' THEN data:=data||jsonb_build_object('defect',row_data); END IF;
        data:=data||jsonb_build_object('actor',COALESCE(nullif(current_setting('app.actor',true),''),
            row_data->>'reported_by',CASE WHEN TG_OP='INSERT' THEN row_data->>'created_by' END));
        PERFORM po_email_emit(code,order_id,code||':'||(row_data->>'id')||':'||txid_current(),data);
    END IF;
    PERFORM po_email_reconcile(order_id);
    RETURN NULL;
END $$;

-- Baseline existing readiness without generating historical email.
DO $$ DECLARE p RECORD; BEGIN
    FOR p IN SELECT id FROM purchase_orders LOOP PERFORM po_email_reconcile(p.id,true); END LOOP;
END $$;
CREATE CONSTRAINT TRIGGER po_email_order AFTER INSERT OR UPDATE ON purchase_orders DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION po_email_capture();
CREATE CONSTRAINT TRIGGER po_email_item AFTER INSERT OR UPDATE OR DELETE ON po_items DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION po_email_capture();
CREATE CONSTRAINT TRIGGER po_email_line AFTER INSERT OR UPDATE OR DELETE ON po_line_items DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION po_email_capture();
CREATE CONSTRAINT TRIGGER po_email_receipt AFTER INSERT OR UPDATE OR DELETE ON po_receipt_lines DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION po_email_capture();
CREATE CONSTRAINT TRIGGER po_email_defect AFTER INSERT OR UPDATE OR DELETE ON po_defects DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION po_email_capture();

-- Timed episode keys are stable across threshold changes; dates and new receipts
-- intentionally start new episodes. Calendar deadlines use Africa/Nairobi.
CREATE VIEW po_email_timed_candidates AS
WITH items AS (
    SELECT i.*,p.status,count(t.id) AS lines,COALESCE(sum(t.delivered),0) AS delivered,
        bool_or(t.accepted<t.qty_ordered) AS outstanding,max(t.last_received_at) AS last_receipt
    FROM po_items i JOIN purchase_orders p ON p.id=i.purchase_order_id
    LEFT JOIN po_notification_line_totals t ON t.po_item_id=i.id GROUP BY i.id,p.status
)
SELECT 'po_delivery_approaching'::text AS code,i.purchase_order_id,i.id AS item_id,
    'approaching:'||i.id||':'||i.notification_deadline_version||':'||i.expected_delivery_date AS key,
    i.status!='cancelled' AND i.outstanding AND i.expected_delivery_date BETWEEN
        (now() AT TIME ZONE 'Africa/Nairobi')::date AND (now() AT TIME ZONE 'Africa/Nairobi')::date+s.approaching_window_days AS qualifies,
    jsonb_build_object('window_days',s.approaching_window_days) AS extra
FROM items i CROSS JOIN po_settings s WHERE i.expected_delivery_date IS NOT NULL
UNION ALL
SELECT 'po_delivery_overdue',i.purchase_order_id,i.id,'overdue:'||i.id||':'||i.notification_deadline_version||':'||i.expected_delivery_date,
    i.status!='cancelled' AND i.outstanding AND i.expected_delivery_date+s.overdue_grace_days<(now() AT TIME ZONE 'Africa/Nairobi')::date,
    jsonb_build_object('grace_days',s.overdue_grace_days,'days_overdue',(now() AT TIME ZONE 'Africa/Nairobi')::date-i.expected_delivery_date)
FROM items i CROSS JOIN po_settings s WHERE i.expected_delivery_date IS NOT NULL
UNION ALL
SELECT 'po_partial_stalled',i.purchase_order_id,i.id,'stalled:'||i.id||':'||extract(epoch from i.last_receipt),
    i.status!='cancelled' AND i.outstanding AND i.delivered>0 AND i.last_receipt<=now()-make_interval(secs=>s.stalled_after_days::double precision*86400),
    jsonb_build_object('threshold_days',s.stalled_after_days,'last_receipt_at',i.last_receipt)
FROM items i CROSS JOIN po_settings s WHERE i.last_receipt IS NOT NULL
UNION ALL
SELECT 'po_defect_unresolved',i.purchase_order_id,i.id,'defect:'||d.id,
    p.status!='cancelled' AND d.status='open' AND d.reported_at<=now()-make_interval(secs=>s.unresolved_defect_after_days::double precision*86400),
    jsonb_build_object('defect',to_jsonb(d),'threshold_days',s.unresolved_defect_after_days)
FROM po_defects d JOIN po_line_items l ON l.id=d.po_line_item_id JOIN po_items i ON i.id=l.po_item_id
JOIN purchase_orders p ON p.id=i.purchase_order_id CROSS JOIN po_settings s;

CREATE TABLE po_email_timed_history (occurrence_key TEXT PRIMARY KEY, recorded_at TIMESTAMPTZ NOT NULL DEFAULT now());
-- Existing qualifying episodes are acknowledged once at migration, never emailed.
INSERT INTO po_email_timed_history(occurrence_key) SELECT key FROM po_email_timed_candidates WHERE qualifies;

CREATE FUNCTION po_email_sweep() RETURNS void LANGUAGE plpgsql AS $$
DECLARE candidate RECORD;
BEGIN
    IF NOT pg_try_advisory_xact_lock(738101501) THEN RETURN; END IF;
    FOR candidate IN SELECT c.* FROM po_email_timed_candidates c
        JOIN notification_events e ON e.code=c.code JOIN notification_settings s ON s.notification_event_id=e.id
        WHERE c.qualifies AND s.enabled AND s.email_enabled
        AND EXISTS(SELECT 1 FROM notification_recipients r WHERE r.notification_event_id=e.id AND r.enabled)
        AND NOT EXISTS(SELECT 1 FROM po_email_timed_history h WHERE h.occurrence_key=c.key) LOOP
        PERFORM po_email_emit(candidate.code,candidate.purchase_order_id,candidate.key,
            po_email_snapshot(candidate.purchase_order_id,candidate.item_id)||candidate.extra);
        UPDATE po_notification_occurrences SET state='pending',
            payload=po_email_snapshot(candidate.purchase_order_id,candidate.item_id)||candidate.extra
            WHERE occurrence_key=candidate.key AND state='suppressed';
    END LOOP;
END $$;
