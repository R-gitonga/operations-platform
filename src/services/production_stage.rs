use crate::{
    database::DbPool,
    models::production_stage::ProductionStage,
    repositories::production_stage,
};

pub async fn list(
    pool: &DbPool,
) -> Result<Vec<ProductionStage>, sqlx::Error> {

    production_stage::find_all(pool).await
}

pub async fn get(
    pool: &DbPool,
    id: i32,
) -> Result<ProductionStage, sqlx::Error> {

    production_stage::find_by_id(pool, id).await
}