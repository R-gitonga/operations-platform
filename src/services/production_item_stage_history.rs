use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        change_production_item_stage::ChangeProductionItemStageRequest,
        wso_stage_history::WsoStageHistory,
    },
    repositories::{
        wso,
        wso_item,
        wso_stage_history,
    },
    services::wso_rules,
};

pub async fn create(
    pool: &DbPool,
    wso_item_id: i32,
    request: ChangeProductionItemStageRequest,
) -> Result<(), AppError> {

    //---------------------------------------------------------
    // Load the production item
    //---------------------------------------------------------

    let production_item =
        wso_item::find_by_id(
            pool,
            wso_item_id,
        )
        .await?;

    //---------------------------------------------------------
    // Load the parent Workshop Order
    //---------------------------------------------------------

    let order =
        wso::find_by_id(
            pool,
            production_item.wso_order_id,
        )
        .await?;

    //---------------------------------------------------------
    // Completed / Cancelled WSOs cannot change stage
    //---------------------------------------------------------

    wso_rules::ensure_can_edit(&order)?;

    //---------------------------------------------------------
    // Perform the stage change
    //---------------------------------------------------------

    wso_stage_history::create(
        pool,
        wso_item_id,
        request,
    )
    .await?;

    Ok(())
}

pub async fn list(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<Vec<WsoStageHistory>, sqlx::Error> {

    wso_stage_history::find_by_wso_item(
        pool,
        wso_item_id,
    )
    .await
}