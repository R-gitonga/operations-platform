use sqlx::Row;

use crate::{
    database::DbPool,
    models::dashboard::{
        OutstandingOrder,
        ProductionStageSummary,
        RecentActivity,
        RecentActivityPage,
        RecentOrder,
    },
};

pub async fn get_order_totals(
    pool: &DbPool,
) -> Result<(i64, i64, i64, i64, i64), sqlx::Error> {

    let row = sqlx::query(
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

    Ok((
        row.try_get("total_orders")?,
        row.try_get("active_orders")?,
        row.try_get("partial_orders")?,
        row.try_get("completed_orders")?,
        row.try_get("cancelled_orders")?,
    ))
}

pub async fn get_quantity_totals(
    pool: &DbPool,
) -> Result<(i64, i64), sqlx::Error> {

    let row = sqlx::query(
        r#"
        SELECT
            COALESCE(SUM(qty_raised),0) AS total_qty_raised,
            COALESCE(SUM(qty_received),0) AS total_qty_received
        FROM wso_line_items
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok((
        row.try_get("total_qty_raised")?,
        row.try_get("total_qty_received")?,
    ))
}

pub async fn get_production_stage_summary(
    pool: &DbPool,
) -> Result<Vec<ProductionStageSummary>, sqlx::Error> {

    Ok(
        sqlx::query(
            r#"
            SELECT
                ps.id AS stage_id,
                ps.display_name As stage_name,
                ps.color AS stage_color,

                COUNT(wi.id) AS item_count
            FROM production_Stages ps

            LEFT JOIN wso_items wi
                ON wi.current_stage_id = ps.id

            GROUP BY
                ps.id,
                ps.display_name,
                ps.color,
                ps.display_order

            ORDER BY
                ps.display_order
            "#,
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| ProductionStageSummary {

            stage_id: row.get("stage_id"),

            stage_name: row.get("stage_name"),

            stage_color: row.get("stage_color"),

            item_count: row.get("item_count"),
        })
        .collect(),
    )
}

pub async fn get_recent_orders(
    pool: &DbPool,
) -> Result<Vec<RecentOrder>, sqlx::Error> {

    Ok(
        sqlx::query(
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
        .collect(),
    )
}

pub async fn get_outstanding_orders(
    pool: &DbPool,
) -> Result<Vec<OutstandingOrder>, sqlx::Error> {

    Ok(
        sqlx::query(
            r#"
            SELECT
                w.id,
                w.wso_number,
                COALESCE(
                    SUM(li.qty_raised - li.qty_received),
                    0
                ) AS outstanding_qty

            FROM wso_orders w

            JOIN wso_line_items li
                ON li.wso_order_id = w.id

            WHERE LOWER(w.status)
                IN ('active','partial')

            GROUP BY
                w.id,
                w.wso_number

            HAVING
                COALESCE(
                    SUM(li.qty_raised - li.qty_received),
                    0
                ) > 0

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
        .collect(),
    )
}

pub async fn get_recent_activity(
    pool: &DbPool,
    page: i64,
    page_size: i64,
) -> Result<RecentActivityPage, sqlx::Error> {

    const ACTIVITY_SOURCE: &str = r#"
        (
            SELECT
                h.changed_at,

                w.id AS wso_id,
                w.wso_number,

                wi.id AS wso_item_id,
                wi.description,

                'stage_change' AS event_type,

                ps.display_name AS stage_name,

                h.changed_by,
                h.notes,

                NULL::INTEGER AS quantity_received,
                NULL::INTEGER AS total_raised,
                NULL::INTEGER AS balance

            FROM wso_stage_history h

            JOIN wso_items wi
                ON wi.id = h.wso_item_id

            JOIN wso_orders w
                ON w.id = wi.wso_order_id

            JOIN production_stages ps
                ON ps.id = h.production_stage_id

            UNION ALL

            SELECT
                r.received_at AS changed_at,

                w.id AS wso_id,
                w.wso_number,

                wi.id AS wso_item_id,
                wi.description,

                'partial_received' AS event_type,

                'Partially Received' AS stage_name,

                r.received_by AS changed_by,
                NULL::TEXT AS notes,

                r.quantity_received,
                r.total_raised,
                r.balance

            FROM wso_partial_receipt_events r

            JOIN wso_items wi
                ON wi.id = r.wso_item_id

            JOIN wso_orders w
                ON w.id = wi.wso_order_id
        ) activity
    "#;

    let total: i64 = sqlx::query_scalar(
        &format!(
            "SELECT COUNT(*) FROM {}",
            ACTIVITY_SOURCE
        )
    )
    .fetch_one(pool)
    .await?;

    let offset = (page - 1) * page_size;

    let items = sqlx::query(
        &format!(
            r#"
            SELECT *
            FROM {}
            ORDER BY changed_at DESC
            LIMIT $1
            OFFSET $2
            "#,
            ACTIVITY_SOURCE
        )
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| RecentActivity {

        changed_at: row.get("changed_at"),

        wso_id: row.get("wso_id"),

        wso_number: row.get("wso_number"),

        wso_item_id: row.get("wso_item_id"),

        description: row.get("description"),

        event_type: row.get("event_type"),

        stage_name: row.get("stage_name"),

        changed_by: row.get("changed_by"),

        notes: row.get("notes"),

        quantity_received: row.get("quantity_received"),

        total_raised: row.get("total_raised"),

        balance: row.get("balance"),
    })
    .collect::<Vec<RecentActivity>>();

    let total_pages = if total == 0 {
        0
    } else {
        (total + page_size - 1) / page_size
    };

    Ok(RecentActivityPage {
        items,
        page,
        page_size,
        total,
        total_pages,
    })
}