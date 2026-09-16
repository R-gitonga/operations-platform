use sqlx::{query_as, FromRow};

use crate::database::DbPool;

#[derive(Debug, Clone, FromRow)]
pub struct PoStatusInputs {
    pub purchase_order_id: i32,
    pub raw_status: String,
    pub total_ordered: i64,
    pub total_delivered: i64,
    pub total_non_accepted_defective: i64,
}

const PO_STATUS_INPUTS_SELECT: &str = r#"
SELECT
    purchase_orders.id AS purchase_order_id,
    purchase_orders.status AS raw_status,
    COALESCE(SUM(po_line_items.qty_ordered), 0) AS total_ordered,
    COALESCE(SUM(delivered.total_delivered), 0) AS total_delivered,
    COALESCE(SUM(defective.total_defective), 0) AS total_non_accepted_defective
FROM purchase_orders
LEFT JOIN po_items
    ON po_items.purchase_order_id = purchase_orders.id
LEFT JOIN po_line_items
    ON po_line_items.po_item_id = po_items.id
LEFT JOIN (
    SELECT
        po_line_item_id,
        SUM(qty_delivered) AS total_delivered
    FROM po_receipt_lines
    GROUP BY po_line_item_id
) delivered
    ON delivered.po_line_item_id = po_line_items.id
LEFT JOIN (
    SELECT
        po_line_item_id,
        SUM(qty_defective) AS total_defective
    FROM po_defects
    WHERE resolution_type IS NULL OR resolution_type != 'accepted'
    GROUP BY po_line_item_id
) defective
    ON defective.po_line_item_id = po_line_items.id
"#;

pub async fn find_all_status_inputs(
    pool: &DbPool,
) -> Result<Vec<PoStatusInputs>, sqlx::Error> {
    query_as::<_, PoStatusInputs>(
        &format!(
            "{PO_STATUS_INPUTS_SELECT} GROUP BY purchase_orders.id, purchase_orders.status"
        ),
    )
    .fetch_all(pool)
    .await
}

pub async fn find_status_inputs_by_id(
    pool: &DbPool,
    purchase_order_id: i32,
) -> Result<PoStatusInputs, sqlx::Error> {
    query_as::<_, PoStatusInputs>(
        &format!(
            r#"
            {PO_STATUS_INPUTS_SELECT}
            WHERE purchase_orders.id = $1
            GROUP BY purchase_orders.id, purchase_orders.status
            "#
        ),
    )
    .bind(purchase_order_id)
    .fetch_one(pool)
    .await
}