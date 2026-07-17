use sqlx::Row;

use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        wso::WsoOrder,
        wso_detail::WsoDetail,
        wso_summary::WsoSummary,
    },
    repositories::{
        line_item,
        wso,
    },
    services::wso_rules,
};

pub async fn get_wso_detail(
    pool: &DbPool,
    wso_id: i32,
) -> Result<WsoDetail, sqlx::Error> {

    let wso_order =
        wso::find_by_id(pool, wso_id).await?;

    let line_items =
        line_item::find_by_wso(pool, wso_id).await?;

    let total_qty_raised =
        line_items.iter().map(|x| x.qty_raised).sum();

    let total_qty_received =
        line_items.iter().map(|x| x.qty_received).sum();

    let total_balance =
        line_items.iter().map(|x| x.balance).sum();

    Ok(WsoDetail {
        id: wso_order.id,
        category_id: wso_order.category_id,
        date_signed: wso_order.date_signed,
        wso_number: wso_order.wso_number,
        req_number: wso_order.req_number,
        description: wso_order.description,
        design_code: wso_order.design_code,
        fabric_code: wso_order.fabric_code,
        remarks: wso_order.remarks,
        attachment_name: wso_order.attachment_name,
        attachment_path: wso_order.attachment_path,
        status: wso_order.status,

        line_item_count: line_items.len(),
        total_qty_raised,
        total_qty_received,
        total_balance,

        line_items,
    })
}

pub async fn get_wso_summary(
    pool: &DbPool,
) -> Result<WsoSummary, sqlx::Error> {

    let rows = sqlx::query(
        r#"
        SELECT
            wso_orders.status,
            COUNT(DISTINCT wso_orders.id) AS order_count,
            COALESCE(SUM(wso_line_items.qty_raised),0) AS total_qty_raised,
            COALESCE(SUM(wso_line_items.qty_received),0) AS total_qty_received
        FROM wso_orders
        LEFT JOIN wso_line_items
            ON wso_orders.id = wso_line_items.wso_order_id
        GROUP BY wso_orders.status
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut status_counts =
        std::collections::HashMap::new();

    let mut total_orders = 0;
    let mut total_qty_raised = 0;
    let mut total_qty_received = 0;

    for row in rows {

        let status: String =
            row.try_get("status")?;

        let count: i64 =
            row.try_get("order_count")?;

        let raised: i64 =
            row.try_get("total_qty_raised")?;

        let received: i64 =
            row.try_get("total_qty_received")?;

        status_counts.insert(status, count);

        total_orders += count;
        total_qty_raised += raised;
        total_qty_received += received;
    }

    Ok(WsoSummary {
        total_orders,
        status_counts,
        total_qty_raised,
        total_qty_received,
        total_balance:
            total_qty_raised - total_qty_received,
    })
}

pub async fn cancel(
    pool: &DbPool,
    id: i32,
) -> Result<WsoOrder, AppError> {

    let order =
        wso::find_by_id(pool, id).await?;

    wso_rules::ensure_can_cancel(&order)?;

    Ok(
        wso::cancel(pool, id).await?
    )
}

pub async fn reactivate(
    pool: &DbPool,
    id: i32,
) -> Result<WsoOrder, AppError> {

    let order =
        wso::find_by_id(pool, id).await?;

    wso_rules::ensure_can_reactivate(&order)?;

    Ok(
        wso::reactivate(pool, id).await?
    )
}

pub async fn refresh_wso_status(
    pool: &DbPool,
    wso_id: i32,
) -> Result<(), AppError> {

    let mut order =
        wso::find_by_id(pool, wso_id).await?;

    if order.status.eq_ignore_ascii_case("cancelled") {
        return Ok(());
    }

    let items =
        line_item::find_by_wso(pool, wso_id).await?;

    if items.is_empty() {
        order.status = "active".to_string();

        wso::update(pool, &order).await?;

        return Ok(());
    }

    let total_raised: i32 =
        items.iter().map(|x| x.qty_raised).sum();

    let total_received: i32 =
        items.iter().map(|x| x.qty_received).sum();

    if total_received == 0 {

        order.status = "active".to_string();

    } else if total_received < total_raised {

        order.status = "partial".to_string();

    } else {

        order.status = "completed".to_string();
    }

    wso::update(pool, &order).await?;

    Ok(())
}