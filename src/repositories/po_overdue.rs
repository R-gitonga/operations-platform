use sqlx::query_as;

use crate::{database::DbPool, models::po_dashboard::PoOverdueItem};

pub async fn find_overdue(
    pool: &DbPool,
) -> Result<Vec<PoOverdueItem>, sqlx::Error> {
    query_as::<_, PoOverdueItem>(
        r#"
        SELECT
            po.id AS purchase_order_id,
            po.accounts_reference,
            s.name AS supplier_name,
            pi.id AS po_item_id,
            pi.description,
            li.size,
            pi.expected_delivery_date,
            (CURRENT_DATE - pi.expected_delivery_date)::BIGINT AS days_overdue,
            (
                li.qty_ordered
                - (
                    COALESCE(delivered.total_delivered, 0)
                    - COALESCE(defective.total_defective, 0)
                )
            )::BIGINT AS outstanding
        FROM po_items pi
        JOIN purchase_orders po
            ON po.id = pi.purchase_order_id
        JOIN suppliers s
            ON s.id = po.supplier_id
        JOIN po_line_items li
            ON li.po_item_id = pi.id
        LEFT JOIN (
            SELECT po_line_item_id, SUM(qty_delivered) AS total_delivered
            FROM po_receipt_lines
            GROUP BY po_line_item_id
        ) delivered
            ON delivered.po_line_item_id = li.id
        LEFT JOIN (
            SELECT po_line_item_id, SUM(qty_defective) AS total_defective
            FROM po_defects
            WHERE resolution_type IS NULL OR resolution_type != 'accepted'
            GROUP BY po_line_item_id
        ) defective
            ON defective.po_line_item_id = li.id
        WHERE
            LOWER(po.status) != 'cancelled'
            AND pi.expected_delivery_date IS NOT NULL
            AND pi.expected_delivery_date < CURRENT_DATE
            AND (
                li.qty_ordered
                - (
                    COALESCE(delivered.total_delivered, 0)
                    - COALESCE(defective.total_defective, 0)
                )
            ) > 0
        ORDER BY days_overdue DESC
        "#,
    )
    .fetch_all(pool)
    .await
}