use sqlx::{Postgres, Row, Transaction};

use crate::{
    database::DbPool,
    models::{
        change_production_item_stage::ChangeProductionItemStageRequest,
        wso_stage_history::WsoStageHistory,
    },
};

pub async fn create(
    pool: &DbPool,
    wso_item_id: i32,
    request: ChangeProductionItemStageRequest,
    changed_by: &str,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    create_tx(&mut tx, wso_item_id, request, changed_by).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    wso_item_id: i32,
    request: ChangeProductionItemStageRequest,
    changed_by: &str,
) -> Result<(), sqlx::Error> {
    //---------------------------------------------------------
    // Update the item's current stage
    //---------------------------------------------------------

    sqlx::query(
        r#"
        UPDATE wso_items
        SET
            current_stage_id = $1
        WHERE id = $2
        "#,
    )
    .bind(request.production_stage_id)
    .bind(wso_item_id)
    .execute(tx.as_mut())
    .await?;

    //---------------------------------------------------------
    // Record history
    //---------------------------------------------------------

    sqlx::query(
        r#"
        INSERT INTO wso_stage_history
        (
            wso_item_id,
            production_stage_id,
            notes,
            changed_by
        )
        VALUES
        (
            $1,$2,$3,$4
        )
        "#,
    )
    .bind(wso_item_id)
    .bind(request.production_stage_id)
    .bind(request.notes)
    .bind(changed_by)
    .execute(tx.as_mut())
    .await?;

    Ok(())
}

pub async fn find_by_wso_item(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<Vec<WsoStageHistory>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            ('stage-' || h.id) AS id,

            h.wso_item_id,

            'stage_change' AS event_type,

            h.production_stage_id,

            s.display_name AS stage_name,

            s.color AS stage_color,

            h.notes,

            NULL::INTEGER AS quantity_received,
            NULL::INTEGER AS total_raised,
            NULL::INTEGER AS balance,

            h.changed_by,
            h.changed_at

        FROM wso_stage_history h

        JOIN production_stages s
            ON s.id = h.production_stage_id

        WHERE h.wso_item_id = $1

        UNION ALL

        SELECT
            ('receipt-' || r.id) AS id,

            r.wso_item_id,

            'partial_received' AS event_type,

            NULL::INTEGER AS production_stage_id,

            'Partially Received' AS stage_name,

            '#f59e0b' AS stage_color,

            NULL::TEXT AS notes,

            r.quantity_received,
            r.total_raised,
            r.balance,

            r.received_by AS changed_by,
            r.received_at AS changed_at

        FROM wso_partial_receipt_events r

        WHERE r.wso_item_id = $1

        ORDER BY changed_at DESC
        "#,
    )
    .bind(wso_item_id)
    .bind(wso_item_id)
    .fetch_all(pool)
    .await?;

    let history = rows
        .into_iter()
        .map(|row| WsoStageHistory {
            id: row.get("id"),

            wso_item_id: row.get("wso_item_id"),

            event_type: row.get("event_type"),

            production_stage_id: row.get("production_stage_id"),

            stage_name: row.get("stage_name"),

            stage_color: row.get("stage_color"),

            notes: row.get("notes"),

            quantity_received: row.get("quantity_received"),

            total_raised: row.get("total_raised"),

            balance: row.get("balance"),

            changed_by: row.get("changed_by"),

            changed_at: row.get("changed_at"),
        })
        .collect();

    Ok(history)
}