use crate::{
    database::DbPool,
    models::{
        change_wso_stage::ChangeWsoStageRequest,
        wso_stage_history::WsoStageHistory,
    },
    repositories::wso_stage_history,
};

pub async fn create(
    pool: &DbPool,
    wso_id: i32,
    request: ChangeWsoStageRequest,
) -> Result<(), sqlx::Error> {

    wso_stage_history::create(
        pool,
        wso_id,
        request,
    )
    .await
}

pub async fn list(
    pool: &DbPool,
    wso_id: i32,
) -> Result<Vec<WsoStageHistory>, sqlx::Error> {

    wso_stage_history::find_by_wso(
        pool,
        wso_id,
    )
    .await
}