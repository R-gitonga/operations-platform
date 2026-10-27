-- Preserve branch and item reasons in future PO notification snapshots.
CREATE OR REPLACE FUNCTION po_email_snapshot(order_id INTEGER, item_id INTEGER DEFAULT NULL)
RETURNS JSONB LANGUAGE sql STABLE AS $$
    SELECT jsonb_build_object('purchase_order_id',p.id,'erp_reference',p.erp_reference,
        'accounts_reference',p.accounts_reference,'supplier',s.name,'created_by',p.created_by,
        'is_paid_order',p.is_paid_order,'branch_name',(SELECT name FROM branches WHERE id=p.branch_id),
        'description',p.description,'item_id',item_id,'actor',nullif(current_setting('app.actor',true),''),
        'items',COALESCE((SELECT jsonb_agg(jsonb_build_object(
            'id',i.id,'paid_order_reason',i.paid_order_reason,'description',i.description,'expected_delivery_date',i.expected_delivery_date,
            'branding_required',i.branding_required,
            'branding_type',(SELECT display_name FROM branding_types WHERE id=i.branding_type_id),
            'branding_location',(SELECT display_name FROM branding_locations WHERE id=i.branding_location_id),
            'lines',(SELECT jsonb_agg(jsonb_build_object('size',li.size,'ordered',t.qty_ordered,
                'delivered',t.delivered,'accepted',t.accepted,'outstanding',greatest(t.qty_ordered-t.accepted,0)))
                FROM po_notification_line_totals t JOIN po_line_items li ON li.id=t.id WHERE t.po_item_id=i.id)))
            FROM po_items i WHERE i.purchase_order_id=p.id AND (item_id IS NULL OR i.id=item_id)), '[]'::jsonb))
    FROM purchase_orders p JOIN suppliers s ON s.id=p.supplier_id WHERE p.id=order_id
$$;

