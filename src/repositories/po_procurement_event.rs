use sqlx::Row;

use crate::{database::DbPool, models::po_procurement_event::PoProcurementEvent};

pub async fn find_by_po_item(
    pool: &DbPool,
    po_item_id: i32,
) -> Result<Vec<PoProcurementEvent>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            ('note-' || n.id) AS id,
            n.po_item_id,
            'note' AS event_type,
            NULL::TEXT AS size,
            n.note,
            NULL::INTEGER AS qty_delivered,
            NULL::INTEGER AS total_delivered_to_date,
            NULL::INTEGER AS delivered_balance,
            NULL::TEXT AS delivery_note_reference,
            NULL::INTEGER AS qty_defective,
            NULL::TEXT AS reason,
            NULL::TEXT AS defect_status,
            NULL::TEXT AS resolution_type,
            NULL::TEXT AS resolution_notes,
            n.created_by AS changed_by,
            n.created_at AS changed_at
        FROM po_item_notes n
        WHERE n.po_item_id = $1

        UNION ALL

        SELECT
            ('receipt-' || rl.id) AS id,
            li.po_item_id,
            'receipt' AS event_type,
            li.size,
            NULL::TEXT AS note,
            rl.qty_delivered,
            rl.total_delivered_to_date,
            rl.delivered_balance,
            r.delivery_note_reference,
            NULL::INTEGER AS qty_defective,
            NULL::TEXT AS reason,
            NULL::TEXT AS defect_status,
            NULL::TEXT AS resolution_type,
            NULL::TEXT AS resolution_notes,
            r.received_by AS changed_by,
            r.received_at AS changed_at
        FROM po_receipt_lines rl
        JOIN po_receipts r ON r.id = rl.po_receipt_id
        JOIN po_line_items li ON li.id = rl.po_line_item_id
        WHERE li.po_item_id = $2

        UNION ALL

        SELECT
            ('defect-' || d.id) AS id,
            li.po_item_id,
            'defect' AS event_type,
            li.size,
            NULL::TEXT AS note,
            NULL::INTEGER AS qty_delivered,
            NULL::INTEGER AS total_delivered_to_date,
            NULL::INTEGER AS delivered_balance,
            NULL::TEXT AS delivery_note_reference,
            d.qty_defective,
            d.reason,
            d.status AS defect_status,
            d.resolution_type,
            d.resolution_notes,
            d.reported_by AS changed_by,
            d.reported_at AS changed_at
        FROM po_defects d
        JOIN po_line_items li ON li.id = d.po_line_item_id
        WHERE li.po_item_id = $3

        ORDER BY changed_at DESC
        "#,
    )
    .bind(po_item_id)
    .bind(po_item_id)
    .bind(po_item_id)
    .fetch_all(pool)
    .await?;

    let events = rows
        .into_iter()
        .map(|row| PoProcurementEvent {
            id: row.get("id"),
            po_item_id: row.get("po_item_id"),
            event_type: row.get("event_type"),
            size: row.get("size"),
            note: row.get("note"),
            qty_delivered: row.get("qty_delivered"),
            total_delivered_to_date: row.get("total_delivered_to_date"),
            delivered_balance: row.get("delivered_balance"),
            delivery_note_reference: row.get("delivery_note_reference"),
            qty_defective: row.get("qty_defective"),
            reason: row.get("reason"),
            defect_status: row.get("defect_status"),
            resolution_type: row.get("resolution_type"),
            resolution_notes: row.get("resolution_notes"),
            changed_by: row.get("changed_by"),
            changed_at: row.get("changed_at"),
        })
        .collect();

    Ok(events)
}