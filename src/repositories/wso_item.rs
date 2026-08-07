use sqlx::{query_as, Postgres, Transaction};

use crate::{
    database::DbPool,
    models::{create_wso_item::CreateWsoItemRequest, wso_item::WsoItem},
};
const WSO_ITEM_SELECT: &str = r#"
SELECT
    wi.id,
    wi.wso_order_id,
    wi.category_id,
    wi.description,
    wi.design_code,
    wi.fabric_code,
    wi.branding_required,
    wi.branding_completed,
    wi.status,
    wi.current_stage_id,

    ps.display_name AS current_stage_name,
    ps.color AS current_stage_color,

    sh.changed_by AS current_stage_changed_by,
    sh.changed_at AS current_stage_changed_at,
    sh.notes AS current_stage_notes,

    wi.created_by,
    wi.created_at,
    wi.updated_at

FROM wso_items wi

LEFT JOIN production_stages ps
    ON ps.id = wi.current_stage_id

LEFT JOIN LATERAL (
    SELECT
        changed_by,
        changed_at,
        notes
    FROM wso_stage_history
    WHERE
        wso_item_id = wi.id
    ORDER BY changed_at DESC
    LIMIT 1
) sh ON TRUE
"#;

pub async fn find_by_wso(pool: &DbPool, wso_id: i32) -> Result<Vec<WsoItem>, sqlx::Error> {
    query_as::<_, WsoItem>(&format!(
        "{} WHERE wi.wso_order_id = $1 ORDER BY wi.id",
        WSO_ITEM_SELECT
    ))
    .bind(wso_id)
    .fetch_all(pool)
    .await
}

pub async fn find_first_by_wso(pool: &DbPool, wso_id: i32) -> Result<WsoItem, sqlx::Error> {
    query_as::<_, WsoItem>(&format!(
        "{} WHERE wi.wso_order_id = $1 ORDER BY wi.id LIMIT 1",
        WSO_ITEM_SELECT
    ))
    .bind(wso_id)
    .fetch_one(pool)
    .await
}

pub async fn find_by_id(pool: &DbPool, id: i32) -> Result<WsoItem, sqlx::Error> {
    query_as::<_, WsoItem>(&format!("{} WHERE wi.id = $1", WSO_ITEM_SELECT))
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_tx(
    tx: &mut Transaction<'_, Postgres>,
    wso_order_id: i32,
    payload: &CreateWsoItemRequest,
) -> Result<WsoItem, sqlx::Error> {
    create_with_initial_stage_tx(tx, wso_order_id, payload, None, None, None).await
}

pub async fn create_with_initial_stage_tx(
    tx: &mut Transaction<'_, Postgres>,
    wso_order_id: i32,
    payload: &CreateWsoItemRequest,
    current_stage_id: Option<i32>,
    changed_by: Option<&str>,
    notes: Option<&str>,
) -> Result<WsoItem, sqlx::Error> {
    let created_item = query_as::<_, WsoItem>(
        r#"
        INSERT INTO wso_items
        (
            wso_order_id,
            category_id,
            description,
            design_code,
            fabric_code,
            branding_required,
            branding_completed,
            current_stage_id
        )
        VALUES
        (
            $1,$2,$3,$4,$5,$6,FALSE,$7
        )

        RETURNING
            id,
            wso_order_id,
            category_id,
            description,
            design_code,
            fabric_code,
            branding_required,
            branding_completed,
            status,
            current_stage_id,
            NULL AS current_stage_name,
            NULL AS current_stage_color,
            NULL AS current_stage_changed_by,
            NULL AS current_stage_changed_at,
            NULL AS current_stage_notes,
            created_by,
            created_at,
            updated_at
        "#,
    )
    .bind(wso_order_id)
    .bind(payload.category_id)
    .bind(&payload.description)
    .bind(&payload.design_code)
    .bind(&payload.fabric_code)
    .bind(payload.branding_required)
    .bind(current_stage_id)
    .fetch_one(tx.as_mut())
    .await?;

    if let Some(stage_id) = current_stage_id {
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
        .bind(created_item.id)
        .bind(stage_id)
        .bind(notes)
        .bind(changed_by)
        .execute(tx.as_mut())
        .await?;
    }

    Ok(created_item)
}

pub async fn update(pool: &DbPool, item: &WsoItem) -> Result<WsoItem, sqlx::Error> {
    query_as::<_, WsoItem>(
        r#"
        UPDATE wso_items
        SET
            category_id = $1,
            description = $2,
            design_code = $3,
            fabric_code = $4,
            branding_required = $5,
            branding_completed = $6,
            status = $7,
            current_stage_id = $8,
            updated_at = NOW()
        WHERE id = $9

        RETURNING
            id,
            wso_order_id,
            category_id,
            description,
            design_code,
            fabric_code,
            branding_required,
            branding_completed,
            status,
            current_stage_id,
            NULL AS current_stage_name,
            NULL AS current_stage_color,
            NULL AS current_stage_changed_by,
            NULL AS current_stage_changed_at,
            NULL AS current_stage_notes,

            created_by,
            created_at,
            updated_at
        "#,
    )
    .bind(item.category_id)
    .bind(&item.description)
    .bind(&item.design_code)
    .bind(&item.fabric_code)
    .bind(item.branding_required)
    .bind(item.branding_completed)
    .bind(&item.status)
    .bind(item.current_stage_id)
    .bind(item.id)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &DbPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM wso_items WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
