use sqlx::{FromRow, Row};

use crate::{database::DbPool, models::po_dashboard::{PoRecentActivity, PoRecentActivityPage}};

#[derive(Debug, FromRow)]
pub struct RecentOrderRow {
    pub id: i32,
    pub accounts_reference: String,
    pub supplier_name: String,
}

pub async fn find_recent_orders(
    pool: &DbPool,
    limit: i64,
) -> Result<Vec<RecentOrderRow>, sqlx::Error> {
    sqlx::query_as::<_, RecentOrderRow>(
        r#"
        SELECT
            purchase_orders.id,
            purchase_orders.accounts_reference,
            suppliers.name AS supplier_name
        FROM purchase_orders
        JOIN suppliers
            ON suppliers.id = purchase_orders.supplier_id
        ORDER BY purchase_orders.created_at DESC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

// Same three sources as the per-item Procurement Timeline
// (repositories::po_procurement_event), unfiltered by po_item_id
// and joined up to purchase_orders for a reference to display.
const PO_ACTIVITY_SOURCE: &str = r#"
(
    SELECT
        n.created_at AS changed_at,
        po.id AS purchase_order_id,
        po.accounts_reference,
        pi.id AS po_item_id,
        pi.description AS item_description,
        'note' AS event_type,
        NULL::TEXT AS size,
        n.created_by AS changed_by,
        n.note,
        NULL::INTEGER AS qty_delivered,
        NULL::INTEGER AS total_delivered_to_date,
        NULL::INTEGER AS delivered_balance,
        NULL::INTEGER AS qty_defective,
        NULL::TEXT AS reason,
        NULL::TEXT AS defect_status
    FROM po_item_notes n
    JOIN po_items pi ON pi.id = n.po_item_id
    JOIN purchase_orders po ON po.id = pi.purchase_order_id

    UNION ALL

    SELECT
        r.received_at AS changed_at,
        po.id AS purchase_order_id,
        po.accounts_reference,
        pi.id AS po_item_id,
        pi.description AS item_description,
        'receipt' AS event_type,
        li.size,
        r.received_by AS changed_by,
        NULL::TEXT AS note,
        rl.qty_delivered,
        rl.total_delivered_to_date,
        rl.delivered_balance,
        NULL::INTEGER AS qty_defective,
        NULL::TEXT AS reason,
        NULL::TEXT AS defect_status
    FROM po_receipt_lines rl
    JOIN po_receipts r ON r.id = rl.po_receipt_id
    JOIN po_line_items li ON li.id = rl.po_line_item_id
    JOIN po_items pi ON pi.id = li.po_item_id
    JOIN purchase_orders po ON po.id = pi.purchase_order_id

    UNION ALL

    SELECT
        d.reported_at AS changed_at,
        po.id AS purchase_order_id,
        po.accounts_reference,
        pi.id AS po_item_id,
        pi.description AS item_description,
        'defect' AS event_type,
        li.size,
        d.reported_by AS changed_by,
        NULL::TEXT AS note,
        NULL::INTEGER AS qty_delivered,
        NULL::INTEGER AS total_delivered_to_date,
        NULL::INTEGER AS delivered_balance,
        d.qty_defective,
        d.reason,
        d.status AS defect_status
    FROM po_defects d
    JOIN po_line_items li ON li.id = d.po_line_item_id
    JOIN po_items pi ON pi.id = li.po_item_id
    JOIN purchase_orders po ON po.id = pi.purchase_order_id
) activity
"#;

pub async fn get_recent_activity(
    pool: &DbPool,
    page: i64,
    page_size: i64,
) -> Result<PoRecentActivityPage, sqlx::Error> {

    let total: i64 = sqlx::query_scalar(
        &format!("SELECT COUNT(*) FROM {PO_ACTIVITY_SOURCE}"),
    )
    .fetch_one(pool)
    .await?;

    let offset = (page - 1) * page_size;

    let items = sqlx::query(
        &format!(
            r#"
            SELECT *
            FROM {PO_ACTIVITY_SOURCE}
            ORDER BY changed_at DESC
            LIMIT $1
            OFFSET $2
            "#
        ),
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| PoRecentActivity {
        changed_at: row.get("changed_at"),
        purchase_order_id: row.get("purchase_order_id"),
        accounts_reference: row.get("accounts_reference"),
        po_item_id: row.get("po_item_id"),
        item_description: row.get("item_description"),
        event_type: row.get("event_type"),
        size: row.get("size"),
        changed_by: row.get("changed_by"),
        note: row.get("note"),
        qty_delivered: row.get("qty_delivered"),
        total_delivered_to_date: row.get("total_delivered_to_date"),
        delivered_balance: row.get("delivered_balance"),
        qty_defective: row.get("qty_defective"),
        reason: row.get("reason"),
        defect_status: row.get("defect_status"),
    })
    .collect::<Vec<PoRecentActivity>>();

    let total_pages = if total == 0 {
        0
    } else {
        (total + page_size - 1) / page_size
    };

    Ok(PoRecentActivityPage {
        items,
        page,
        page_size,
        total,
        total_pages,
    })
}