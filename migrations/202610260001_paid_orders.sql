CREATE TABLE branches (
    id SERIAL PRIMARY KEY,
    code TEXT NOT NULL CHECK (code = btrim(code) AND length(code) BETWEEN 1 AND 30),
    name TEXT NOT NULL CHECK (name = btrim(name) AND length(name) BETWEEN 1 AND 150),
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE UNIQUE INDEX branches_code_unique ON branches(lower(code));
ALTER TABLE wso_orders ADD COLUMN is_paid_order BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN branch_id INTEGER REFERENCES branches(id) ON DELETE RESTRICT,
    ADD CONSTRAINT wso_paid_branch CHECK (is_paid_order = (branch_id IS NOT NULL));
ALTER TABLE purchase_orders ADD COLUMN is_paid_order BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN branch_id INTEGER REFERENCES branches(id) ON DELETE RESTRICT,
    ADD CONSTRAINT po_paid_branch CHECK (is_paid_order = (branch_id IS NOT NULL));
ALTER TABLE wso_items ADD COLUMN paid_order_reason TEXT CHECK (paid_order_reason IN ('mtm','stock_out'));
ALTER TABLE po_items ADD COLUMN paid_order_reason TEXT CHECK (paid_order_reason IN ('mtm','stock_out'));
CREATE INDEX wso_paid_branch_date ON wso_orders(branch_id,created_at,id) WHERE is_paid_order;
CREATE INDEX po_paid_branch_date ON purchase_orders(branch_id,created_at,id) WHERE is_paid_order;

CREATE TABLE paid_order_audit (
    id BIGSERIAL PRIMARY KEY,
    source_table TEXT NOT NULL,
    record_id INTEGER NOT NULL,
    old_metadata JSONB,
    new_metadata JSONB NOT NULL,
    actor TEXT,
    changed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX paid_order_audit_record ON paid_order_audit(source_table,record_id,changed_at);
CREATE FUNCTION paid_order_audit_change() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE old_data JSONB; new_data JSONB;
BEGIN
    IF TG_TABLE_NAME IN ('wso_orders','purchase_orders') THEN
        new_data := jsonb_build_object('is_paid_order',NEW.is_paid_order,'branch_id',NEW.branch_id);
        IF TG_OP='UPDATE' THEN old_data := jsonb_build_object('is_paid_order',OLD.is_paid_order,'branch_id',OLD.branch_id); END IF;
    ELSE
        new_data := jsonb_build_object('paid_order_reason',NEW.paid_order_reason);
        IF TG_OP='UPDATE' THEN old_data := jsonb_build_object('paid_order_reason',OLD.paid_order_reason); END IF;
    END IF;
    IF old_data IS DISTINCT FROM new_data AND
       (TG_OP='UPDATE' OR (to_jsonb(NEW)->>'is_paid_order')='true' OR to_jsonb(NEW)->>'paid_order_reason' IS NOT NULL) THEN
        INSERT INTO paid_order_audit(source_table,record_id,old_metadata,new_metadata,actor)
        VALUES(TG_TABLE_NAME,NEW.id,old_data,new_data,nullif(current_setting('app.actor',true),''));
    END IF;
    RETURN NEW;
END $$;

-- Lock the parent on item writes so classification corrections cannot race
-- additions. Deferred validation below checks the final transaction state.
CREATE FUNCTION paid_item_lock_parent() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF TG_TABLE_NAME='wso_items' THEN
        PERFORM 1 FROM wso_orders WHERE id=NEW.wso_order_id FOR UPDATE;
    ELSE
        PERFORM 1 FROM purchase_orders WHERE id=NEW.purchase_order_id FOR UPDATE;
    END IF;
    RETURN NEW;
END $$;
CREATE FUNCTION paid_order_validate_items() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE parent_id INTEGER; invalid BOOLEAN;
BEGIN
    IF TG_TABLE_NAME IN ('wso_orders','wso_items') THEN
        parent_id := CASE WHEN TG_TABLE_NAME='wso_orders' THEN NEW.id ELSE (to_jsonb(NEW)->>'wso_order_id')::integer END;
        SELECT EXISTS(SELECT 1 FROM wso_items i JOIN wso_orders o ON o.id=i.wso_order_id
          WHERE o.id=parent_id AND o.is_paid_order <> (i.paid_order_reason IS NOT NULL)) INTO invalid;
    ELSE
        parent_id := CASE WHEN TG_TABLE_NAME='purchase_orders' THEN NEW.id ELSE (to_jsonb(NEW)->>'purchase_order_id')::integer END;
        SELECT EXISTS(SELECT 1 FROM po_items i JOIN purchase_orders o ON o.id=i.purchase_order_id
          WHERE o.id=parent_id AND o.is_paid_order <> (i.paid_order_reason IS NOT NULL)) INTO invalid;
    END IF;
    IF invalid THEN RAISE EXCEPTION 'Every paid item requires a reason; bulk items must have no reason' USING ERRCODE='23514'; END IF;
    RETURN NULL;
END $$;
CREATE FUNCTION paid_order_validate_branch() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    IF NEW.branch_id IS NOT NULL AND (TG_OP='INSERT' OR NEW.branch_id IS DISTINCT FROM OLD.branch_id) THEN
        PERFORM 1 FROM branches WHERE id=NEW.branch_id AND active FOR SHARE;
        IF NOT FOUND THEN RAISE EXCEPTION 'Select an active branch' USING ERRCODE='23514'; END IF;
    END IF;
    RETURN NEW;
END $$;
DO $$ DECLARE t TEXT; BEGIN
    FOREACH t IN ARRAY ARRAY['wso_orders','purchase_orders','wso_items','po_items'] LOOP
        EXECUTE format('CREATE TRIGGER paid_audit AFTER INSERT OR UPDATE ON %I FOR EACH ROW EXECUTE FUNCTION paid_order_audit_change()',t);
        EXECUTE format('CREATE CONSTRAINT TRIGGER paid_items_consistent AFTER INSERT OR UPDATE ON %I DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION paid_order_validate_items()',t);
    END LOOP;
    FOREACH t IN ARRAY ARRAY['wso_orders','purchase_orders'] LOOP
        EXECUTE format('CREATE TRIGGER paid_branch_active BEFORE INSERT OR UPDATE ON %I FOR EACH ROW EXECUTE FUNCTION paid_order_validate_branch()',t);
    END LOOP;
    FOREACH t IN ARRAY ARRAY['wso_items','po_items'] LOOP
        EXECUTE format('CREATE TRIGGER paid_lock_parent BEFORE INSERT OR UPDATE ON %I FOR EACH ROW EXECUTE FUNCTION paid_item_lock_parent()',t);
    END LOOP;
END $$;

-- One row per item, suitable for same-item category/reason filters and later
-- reports. Receipt and defect aggregates are independent to prevent fan-out.
CREATE VIEW paid_order_items AS
SELECT 'wso'::text AS order_type, o.id AS order_id, i.id AS item_id,
    o.wso_number AS reference, coalesce(o.req_number,'') AS secondary_reference,
    o.branch_id, o.created_at, o.status, i.category_id, c.name AS category_name,
    i.description, i.paid_order_reason,
    coalesce(q.ordered,0)::bigint AS ordered, coalesce(q.fulfilled,0)::bigint AS fulfilled
FROM wso_orders o JOIN wso_items i ON i.wso_order_id=o.id
LEFT JOIN categories c ON c.id=i.category_id
LEFT JOIN LATERAL (SELECT sum(qty_raised) ordered, sum(qty_received) fulfilled FROM wso_line_items WHERE wso_item_id=i.id) q ON true
WHERE o.is_paid_order
UNION ALL
SELECT 'po',o.id,i.id,o.erp_reference,o.accounts_reference,o.branch_id,o.created_at,
    CASE WHEN o.status='cancelled' THEN 'cancelled'
         WHEN totals.ordered=0 OR totals.accepted<=0 THEN 'active'
         WHEN totals.complete THEN 'completed' ELSE 'partial' END,
    i.category_id,c.name,i.description,i.paid_order_reason,
    coalesce(q.ordered,0)::bigint,coalesce(q.accepted,0)::bigint
FROM purchase_orders o JOIN po_items i ON i.purchase_order_id=o.id
LEFT JOIN categories c ON c.id=i.category_id
LEFT JOIN LATERAL (
    SELECT sum(l.qty_ordered) ordered,
      sum(coalesce(r.qty,0)-coalesce(d.qty,0)) accepted
    FROM po_line_items l
    LEFT JOIN LATERAL (SELECT sum(qty_delivered) qty FROM po_receipt_lines WHERE po_line_item_id=l.id) r ON true
    LEFT JOIN LATERAL (SELECT sum(qty_defective) qty FROM po_defects WHERE po_line_item_id=l.id AND resolution_type IS DISTINCT FROM 'accepted') d ON true
    WHERE l.po_item_id=i.id
) q ON true
LEFT JOIN LATERAL (
    SELECT coalesce(sum(l.qty_ordered),0) ordered,
      coalesce(sum(coalesce(r.qty,0)-coalesce(d.qty,0)),0) accepted,
      coalesce(bool_and(coalesce(r.qty,0)-coalesce(d.qty,0)>=l.qty_ordered),false) complete
    FROM po_items pi JOIN po_line_items l ON l.po_item_id=pi.id
    LEFT JOIN LATERAL (SELECT sum(qty_delivered) qty FROM po_receipt_lines WHERE po_line_item_id=l.id) r ON true
    LEFT JOIN LATERAL (SELECT sum(qty_defective) qty FROM po_defects WHERE po_line_item_id=l.id AND resolution_type IS DISTINCT FROM 'accepted') d ON true
    WHERE pi.purchase_order_id=o.id
) totals ON true
WHERE o.is_paid_order;
