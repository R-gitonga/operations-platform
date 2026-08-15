use chrono::{Duration, Utc};

use crate::{
    database::DbPool,
    errors::app_error::AppError,
    repositories::{
        line_item,
        partial_receiving_settings,
        partial_receiving_tracking,
    },
};

pub async fn sync_tracking(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<(), AppError> {
    let line_items =
        line_item::find_by_item(pool, wso_item_id).await?;

    if line_items.is_empty() {
        return Ok(());
    }

    let total_balance: i32 =
        line_items
            .iter()
            .map(|item| item.balance)
            .sum();

    let total_received: i32 =
        line_items
            .iter()
            .map(|item| item.qty_received)
            .sum();

    let is_partial =
        total_received > 0 && total_balance > 0;

    let is_fully_received =
        total_received > 0 && total_balance == 0;

    let tracking =
        partial_receiving_tracking::find_active_by_wso_item(
            pool,
            wso_item_id,
        )
        .await?;

    if is_partial {
        if tracking.is_none() {
            partial_receiving_tracking::create(
                pool,
                wso_item_id,
                Utc::now(),
            )
            .await?;
        }

        return Ok(());
    }

    if is_fully_received {
        if let Some(tracking) = tracking {
            partial_receiving_tracking::resolve(
                pool,
                tracking.id,
                Utc::now(),
            )
            .await?;
        }
    }

    Ok(())
}