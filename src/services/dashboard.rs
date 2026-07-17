use crate::{
    database::DbPool,
    models::dashboard::{
        DashboardSummary,
        OutstandingOrder,
        RecentOrder,
    },
};

use sqlx::Row;

pub async fn get_dashboard(
    pool: &DbPool,
) -> Result<DashboardSummary, sqlx::Error> {
    // -------overall totals --------

    let totals = sqlx::query(
    r#"
    SELECT
        COUNT(*) AS total_orders,

        COUNT(*) FILTER (
            WHERE LOWER(status) = 'active'
        ) AS active_orders,

        COUNT(*) FILTER (
            WHERE LOWER(status) = 'partial'
        ) AS partial_orders,

        COUNT(*) FILTER (
            WHERE LOWER(status) = 'completed'
        ) AS completed_orders,

        COUNT(*) FILTER (
            WHERE LOWER(status) = 'cancelled'
        ) AS cancelled_orders

    FROM wso_orders
    "#,
)
.fetch_one(pool)
.await?;

let total_orders: i64 = totals.try_get("total_orders")?;
let active_orders: i64 = totals.try_get("active_orders")?;
let partial_orders: i64 = totals.try_get("partial_orders")?;
let completed_orders: i64 = totals.try_get("completed_orders")?;
let cancelled_orders: i64 = totals.try_get("cancelled_orders")?;

    // ---------- Quantity totals ----------

    let qty = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(qty_raised), 0) AS total_qty_raised,
            COALESCE(SUM(qty_received), 0) AS total_qty_received
        FROM wso_line_items
        "#,
    )
    .fetch_one(pool)
    .await?;

    let total_qty_raised: i64 = qty.try_get("total_qty_raised")?;
    let total_qty_received: i64 = qty.try_get("total_qty_received")?;

    let total_balance = total_qty_raised - total_qty_received;

    let recent_orders = sqlx::query(
        r#"
            SELECT
                id,
                wso_number,
                status
            FROM wso_orders
            ORDER BY created_at DESC
            LIMIT 5
        "#,
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| RecentOrder {
        id: row.get("id"),
        wso_number: row.get("wso_number"),
        status: row.get("status"),
    })
    .collect::<Vec<RecentOrder>>();

    let largest_outstanding = sqlx::query(
    r#"
    SELECT
        w.id,
        w.wso_number,
        COALESCE(SUM(li.qty_raised - li.qty_received), 0)
            AS outstanding_qty
    FROM wso_orders w
    JOIN wso_line_items li
        ON li.wso_order_id = w.id

    WHERE LOWER(w.status) IN ('active', 'partial')

    GROUP BY
        w.id,
        w.wso_number

    HAVING COALESCE(SUM(li.qty_raised - li.qty_received), 0) > 0

    ORDER BY outstanding_qty DESC

    LIMIT 5
    "#,
)
.fetch_all(pool)
.await?
.into_iter()
.map(|row| OutstandingOrder {
    id: row.get("id"),
    wso_number: row.get("wso_number"),
    outstanding_qty: row.get("outstanding_qty"),
})
.collect::<Vec<OutstandingOrder>>();

    Ok(DashboardSummary {
    total_orders,

    active_orders,
    partial_orders,
    completed_orders,
    cancelled_orders,

    total_qty_raised,
    total_qty_received,
    total_balance,

    recent_orders,
    largest_outstanding,
})
}