use crate::{
    database::DbPool,
    models::{
        wso_detail::WsoDetail,
        wso_summary::WsoSummary,
    },
    repositories::{
        line_item,
        wso,
    },
};

pub async fn get_wso_detail(pool: &DbPool, wso_id: i32) -> Result<WsoDetail, sqlx::Error> {
    let wso_order = wso::find_by_id(pool, wso_id).await?;
    let line_items = line_item::find_by_wso(pool, wso_id).await?;
    let total_quantity: i32 = line_items.iter().map(|item| item.quantity).sum();
    let line_item_count = line_items.len();

    Ok(WsoDetail {
        id: wso_order.id,
        wso_number: wso_order.wso_number,
        req_number: wso_order.req_number,
        description: wso_order.description,
        remarks: wso_order.remarks,
        status: wso_order.status,
        line_item_count,
        total_quantity,
        line_items,
    })
}

pub async fn get_wso_summary(pool: &DbPool) -> Result<WsoSummary, sqlx::Error> {
    let rows = sqlx::query!
        (
            r#"
            SELECT
                COUNT(*) AS total_orders,
                status,
                SUM(quantity) AS total_quantity
            FROM wso_orders
            LEFT JOIN wso_line_items ON wso_orders.id = wso_line_items.wso_order_id
            GROUP BY status
            "#
        )
        .fetch_all(pool)
        .await?;

    let mut status_counts = std::collections::HashMap::new();
    let mut total_quantity = 0i64;
    let mut total_orders = 0i64;

    for row in rows {
        let status = row.status;
        let count: i64 = row.total_orders.unwrap_or(0);
        status_counts.insert(status, count);
        total_quantity += row.total_quantity.unwrap_or(0);
        total_orders += count;
    }

    Ok(WsoSummary {
        total_orders,
        status_counts,
        total_quantity,
    })
}
