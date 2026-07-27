use crate::{
    database::DbPool,
    models::production_stage::ProductionStage,
};

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<ProductionStage>, sqlx::Error> {

    let stages = sqlx::query_as::<_, ProductionStage>(
        r#"
        SELECT

            id,

            code,

            display_name,

            display_order,

            color,

            active

        FROM production_stages

        WHERE active = TRUE

        ORDER BY display_order
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(stages)
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<ProductionStage, sqlx::Error> {

    let stage = sqlx::query_as::<_, ProductionStage>(
        r#"
        SELECT

            id,

            code,

            display_name,

            display_order,

            color,

            active

        FROM production_stages

        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(stage)
}