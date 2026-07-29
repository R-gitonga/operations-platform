use crate::{
    database::DbPool,
    models::{
        change_production_item_stage::ChangeProductionItemStageRequest,
        wso_stage_history::WsoStageHistory,
    },
    repositories::wso_stage_history,
};

pub async fn create(
    pool: &DbPool,
    wso_item_id: i32,
    request: ChangeProductionItemStageRequest,
) -> Result<(), sqlx::Error> {

    wso_stage_history::create(
        pool,
        wso_item_id,
        request,
    )
    .await
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