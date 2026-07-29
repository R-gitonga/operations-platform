use sqlx::Row;

use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        wso::WsoOrder,
        wso_detail::WsoDetail,
        wso_item_detail::WsoItemDetail,
        wso_summary::WsoSummary,
    },
    repositories::{
        line_item,
        wso,
        wso_item,
    },
    services::wso_rules,
};

pub async fn get_wso_detail(
    pool: &DbPool,
    wso_id: i32,
) -> Result<WsoDetail, sqlx::Error> {

    let order = wso::find_by_id(pool, wso_id).await?;

    let items = wso_item::find_by_wso(pool, wso_id).await?;

    let mut detail_items = Vec::new();

    let mut total_qty_raised = 0;
    let mut total_qty_received = 0;
    let mut total_balance = 0;

    for item in items {

        let line_items =
            line_item::find_by_item(pool, item.id).await?;

        let qty_raised: i32 =
            line_items.iter().map(|x| x.qty_raised).sum();

        let qty_received: i32 =
            line_items.iter().map(|x| x.qty_received).sum();

        let balance: i32 =
            line_items.iter().map(|x| x.balance).sum();

        total_qty_raised += qty_raised;
        total_qty_received += qty_received;
        total_balance += balance;

        detail_items.push(
            WsoItemDetail {

                id: item.id,

                category_id: item.category_id,

                description: item.description,

                design_code: item.design_code,

                fabric_code: item.fabric_code,

                branding_required: item.branding_required,

                branding_completed: item.branding_completed,

                current_stage_id: item.current_stage_id,

                current_stage_name: item.current_stage_name,

                current_stage_color: item.current_stage_color,

                current_stage_changed_by: item.current_stage_changed_by,

                current_stage_changed_at: item.current_stage_changed_at,

                current_stage_notes: item.current_stage_notes,

                total_qty_raised: qty_raised,

                total_qty_received: qty_received,

                total_balance: balance,

                line_items,
            }
        );
    }

    Ok(
        WsoDetail {

            id: order.id,

            date_signed: order.date_signed,

            wso_number: order.wso_number,

            req_number: order.req_number,

            attachment_name: order.attachment_name,

            attachment_path: order.attachment_path,

            status: order.status,

            total_items: detail_items.len(),

            total_qty_raised,

            total_qty_received,

            total_balance,

            items: detail_items,
        }
    )
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
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut status_counts =
        std::collections::HashMap::new();

    let mut total_orders = 0;
    let mut total_qty_raised = 0;
    let mut total_qty_received = 0;

    for row in rows {

        let status: String = row.try_get("status")?;
        let count: i64 = row.try_get("order_count")?;
        let raised: i64 = row.try_get("total_qty_raised")?;
        let received: i64 = row.try_get("total_qty_received")?;

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
        total_balance: total_qty_raised - total_qty_received,
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

    let wso_items =
    wso_item::find_by_wso(pool, wso_id).await?;

    let mut all_line_items = Vec::new();

    for item in wso_items {

        let mut lines =
            line_item::find_by_item(pool, item.id).await?;

        all_line_items.append(&mut lines);
    }

    if all_line_items.is_empty() {

        order.status = "active".to_string();

        wso::update(pool, &order).await?;

        return Ok(());
    }

    let total_raised: i32 =
        all_line_items.iter().map(|x| x.qty_raised).sum();

    let total_received: i32 =
        all_line_items.iter().map(|x| x.qty_received).sum();

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