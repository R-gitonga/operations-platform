use sqlx::Row;

use crate::{
    database::DbPool,
    models::{
        change_wso_stage::ChangeWsoStageRequest,
        wso_stage_history::WsoStageHistory,
    },
};

pub async fn create(
    pool: &DbPool,
    wso_id: i32,
    request: ChangeWsoStageRequest,
) -> Result<(), sqlx::Error> {

    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        UPDATE wso_orders

        SET

            current_stage_id = $1,

            updated_at = NOW()

        WHERE id = $2
        "#
    )
    .bind(request.production_stage_id)
    .bind(wso_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO wso_stage_history (

            wso_id,

            production_stage_id,

            notes,

            changed_by

        )
        VALUES ($1,$2,$3,$4)
        "#
    )
    .bind(wso_id)
    .bind(request.production_stage_id)
    .bind(request.notes)
    .bind(request.changed_by)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn find_by_wso(
    pool: &DbPool,
    wso_id: i32,
) -> Result<Vec<WsoStageHistory>, sqlx::Error> {

    let rows = sqlx::query(
        r#"
        SELECT

            h.id,

            h.wso_id,

            h.production_stage_id,

            s.display_name AS stage_name,

            h.notes,

            h.changed_by,

            h.changed_at

        FROM wso_stage_history h

        JOIN production_stages s

            ON s.id = h.production_stage_id

        WHERE h.wso_id = $1

        ORDER BY h.changed_at DESC
        "#
    )
    .bind(wso_id)
    .fetch_all(pool)
    .await?;

    let history = rows
        .into_iter()
        .map(|row| WsoStageHistory {

            id: row.get("id"),

            wso_id: row.get("wso_id"),

            production_stage_id: row.get("production_stage_id"),

            stage_name: row.get("stage_name"),

            notes: row.get("notes"),

            changed_by: row.get("changed_by"),

            changed_at: row.get("changed_at"),
        })
        .collect();

    Ok(history)
}