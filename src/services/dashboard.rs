use crate::{
    database::DbPool,
    models::dashboard::{
        DashboardSummary,
        OrderSummary,
        ProductionSummary,
    },
    repositories::dashboard,
};

pub async fn get_dashboard(
    pool: &DbPool,
    page: i64,
    page_size: i64,
) -> Result<DashboardSummary, sqlx::Error> {

    let (
        total_orders,
        active_orders,
        partial_orders,
        completed_orders,
        cancelled_orders,
    ) = dashboard::get_order_totals(pool).await?;

    let (
        total_qty_raised,
        total_qty_received,
    ) = dashboard::get_quantity_totals(pool).await?;

    let production_stages =
        dashboard::get_production_stage_summary(pool).await?;

    let recent_orders =
        dashboard::get_recent_orders(pool).await?;

    let recent_activity =
        dashboard::get_recent_activity(
            pool,
            page,
            page_size,
        )
        .await?;

    let outstanding_orders =
        dashboard::get_outstanding_orders(pool).await?;

    Ok(DashboardSummary {

        orders: OrderSummary {
            total: total_orders,
            active: active_orders,
            partial: partial_orders,
            completed: completed_orders,
            cancelled: cancelled_orders,
        },

        production: ProductionSummary {
            qty_raised: total_qty_raised,
            qty_received: total_qty_received,
            balance: total_qty_raised - total_qty_received,
        },

        production_stages,

        recent_orders,

        recent_activity,

        outstanding_orders,
    })
}