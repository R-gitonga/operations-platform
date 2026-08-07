use crate::{
    database::DbPool,
    models::{
        attention_required_item::AttentionRequiredItem, production_stage::ProductionStage,
        production_stage_item::ProductionStageItem,
    },
};

use sqlx::{Postgres, Row, Transaction};

pub async fn find_all(pool: &DbPool) -> Result<Vec<ProductionStage>, sqlx::Error> {
    let stages = sqlx::query_as::<_, ProductionStage>(
        r#"
        SELECT

            id,

            code,

            display_name,

            display_order,

            color,

            active,

            expected_duration_hours,

            attention_enabled

        FROM production_stages

        WHERE active = TRUE

        ORDER BY display_order
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(stages)
}

pub async fn find_by_id(pool: &DbPool, id: i32) -> Result<ProductionStage, sqlx::Error> {
    let stage = sqlx::query_as::<_, ProductionStage>(
        r#"
        SELECT

            id,

            code,

            display_name,

            display_order,

            color,

            active,

            expected_duration_hours,

            attention_enabled

        FROM production_stages

        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(stage)
}

pub async fn create(
    pool: &DbPool,
    stage: &ProductionStage,
) -> Result<ProductionStage, sqlx::Error> {
    sqlx::query_as::<_, ProductionStage>(
        r#"
        INSERT INTO production_stages (

            code,
            display_name,
            display_order,
            color,
            active,
            expected_duration_hours,
            attention_enabled

        )

        VALUES (

            $1,$2,$3,$4,$5,$6,$7

        )

        RETURNING *
        "#,
    )
    .bind(&stage.code)
    .bind(&stage.display_name)
    .bind(&stage.display_order)
    .bind(&stage.color)
    .bind(&stage.active)
    .bind(&stage.expected_duration_hours)
    .bind(&stage.attention_enabled)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    stage: &ProductionStage,
) -> Result<ProductionStage, sqlx::Error> {
    sqlx::query_as::<_, ProductionStage>(
        r#"
        UPDATE production_stages

        SET

            code = $1,
            display_name = $2,
            display_order = $3,
            color = $4,
            active = $5,
            expected_duration_hours = $6,
            attention_enabled = $7

        WHERE id = $8

        RETURNING *
        "#,
    )
    .bind(&stage.code)
    .bind(&stage.display_name)
    .bind(stage.display_order)
    .bind(&stage.color)
    .bind(stage.active)
    .bind(stage.expected_duration_hours)
    .bind(stage.attention_enabled)
    .bind(stage.id)
    .fetch_one(pool)
    .await
}

pub async fn find_items_in_stage(
    pool: &DbPool,
    stage_id: i32,
) -> Result<Vec<ProductionStageItem>, sqlx::Error> {
    sqlx::query_as::<_, ProductionStageItem>(
        r#"
        SELECT

            wo.id AS wso_id,

            wo.wso_number,

            wi.id AS wso_item_id,

            wi.description,

            wi.design_code,

            wi.fabric_code,

            ps.display_name AS stage_name,

            ps.color AS stage_color,

            latest.changed_at AS current_stage_changed_at,

            latest.changed_by AS current_stage_changed_by

        FROM wso_items wi

        JOIN wso_orders wo
            ON wo.id = wi.wso_order_id

        JOIN production_stages ps
            ON ps.id = wi.current_stage_id

        LEFT JOIN LATERAL (

            SELECT

                h.changed_at,

                h.changed_by

            FROM wso_stage_history h

            WHERE h.wso_item_id = wi.id

            ORDER BY h.changed_at DESC

            LIMIT 1

        ) latest ON TRUE

        WHERE wi.current_stage_id = $1

        ORDER BY

            wo.wso_number,

            wi.description

        "#,
    )
    .bind(stage_id)
    .fetch_all(pool)
    .await
}

pub async fn count_items_in_stage(pool: &DbPool, stage_id: i32) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            COUNT(*) AS item_count
        FROM wso_items
        WHERE current_stage_id = $1
        "#,
    )
    .bind(stage_id)
    .fetch_one(pool)
    .await?;

    Ok(row.get("item_count"))
}

pub async fn find_by_code(
    pool: &DbPool,
    code: &str,
) -> Result<Option<ProductionStage>, sqlx::Error> {
    sqlx::query_as::<_, ProductionStage>(
        r#"
        SELECT
            id,
            code,
            display_name,
            display_order,
            color,
            active,
            expected_duration_hours,
            attention_enabled
        FROM production_stages
        WHERE LOWER(code) = LOWER($1)
        "#,
    )
    .bind(code)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_code_tx(
    tx: &mut Transaction<'_, Postgres>,
    code: &str,
) -> Result<Option<ProductionStage>, sqlx::Error> {
    sqlx::query_as::<_, ProductionStage>(
        r#"
        SELECT
            id,
            code,
            display_name,
            display_order,
            color,
            active,
            expected_duration_hours,
            attention_enabled
        FROM production_stages
        WHERE LOWER(code) = LOWER($1)
        "#,
    )
    .bind(code)
    .fetch_optional(tx.as_mut())
    .await
}

pub async fn find_attention_required_items(
    pool: &DbPool,
) -> Result<Vec<AttentionRequiredItem>, sqlx::Error> {
    sqlx::query_as::<_, AttentionRequiredItem>(
        r#"
        SELECT
            wo.id AS wso_id,
            wo.wso_number,
            wi.id AS wso_item_id,
            wi.description,
            wi.design_code,
            wi.fabric_code,
            ps.id AS current_stage_id,
            ps.display_name AS current_stage_name,
            ps.color AS current_stage_color,
            latest.changed_at AS stage_started_at,
            ps.expected_duration_hours,
            CAST(
                EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - latest.changed_at)) / 3600.0
                AS DOUBLE PRECISION
            ) AS elapsed_hours,
            CAST(
                (
                    EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - latest.changed_at)) / 3600.0
                ) - ps.expected_duration_hours
                AS DOUBLE PRECISION
            ) AS overdue_hours

        FROM wso_items wi

        JOIN wso_orders wo
            ON wo.id = wi.wso_order_id

        JOIN production_stages ps
            ON ps.id = wi.current_stage_id

        LEFT JOIN LATERAL (
            SELECT h.changed_at
            FROM wso_stage_history h
            WHERE h.wso_item_id = wi.id
            ORDER BY h.changed_at DESC
            LIMIT 1
        ) latest ON TRUE

        WHERE LOWER(wo.status) = 'active'
          AND wi.current_stage_id IS NOT NULL
          AND ps.active = TRUE
          AND ps.attention_enabled = TRUE
          AND ps.expected_duration_hours IS NOT NULL
          AND CAST(
                EXTRACT(EPOCH FROM (CURRENT_TIMESTAMP - latest.changed_at)) / 3600.0
                AS DOUBLE PRECISION
            ) > ps.expected_duration_hours

        ORDER BY overdue_hours DESC,
                 wo.wso_number,
                 wi.description
        "#,
    )
    .fetch_all(pool)
    .await
}
