use sqlx::query_as;

use crate::{
    database::DbPool,
    models::wso::WsoOrder,
};

const WSO_SELECT: &str = r#"
SELECT

    w.id,

    w.category_id,

    w.date_signed,

    w.wso_number,

    w.req_number,

    w.description,

    w.design_code,

    w.fabric_code,

    w.remarks,

    w.attachment_name,

    w.attachment_path,

    w.status,

    w.current_stage_id,

    ps.display_name AS current_stage_name,

    ps.color AS current_stage_color,

    latest.changed_by AS current_stage_changed_by,

    latest.changed_at AS current_stage_changed_at,

    latest.notes AS current_stage_notes,

    w.created_at,

    w.updated_at

FROM wso_orders w

LEFT JOIN production_stages ps

    ON ps.id = w.current_stage_id

LEFT JOIN LATERAL (

    SELECT

        changed_by,

        changed_at,

        notes

    FROM wso_stage_history

    WHERE wso_id = w.id

    ORDER BY changed_at DESC

    LIMIT 1

) latest

ON TRUE
"#;

pub async fn create_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    payload: &crate::models::create_complete_wso::CreateCompleteWsoRequest,
) -> Result<WsoOrder, sqlx::Error> {

    let row = sqlx::query(
        r#"
        INSERT INTO wso_orders
        (
            category_id,
            date_signed,
            wso_number,
            req_number,
            description,
            design_code,
            fabric_code,
            remarks
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)

        RETURNING id
        "#
    )
    .bind(payload.category_id)
    .bind(payload.date_signed)
    .bind(&payload.wso_number)
    .bind(&payload.req_number)
    .bind(&payload.description)
    .bind(&payload.design_code)
    .bind(&payload.fabric_code)
    .bind(&payload.remarks)
    .fetch_one(tx.as_mut())
    .await?;

    let id: i32 = sqlx::Row::get(&row, "id");

    query_as::<_, WsoOrder>(
        &format!(
            "{} WHERE w.id = $1",
            WSO_SELECT
        )
    )
    .bind(id)
    .fetch_one(tx.as_mut())
    .await
}

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<WsoOrder>, sqlx::Error> {

    query_as::<_, WsoOrder>(
        &format!(
            "{} ORDER BY w.id DESC",
            WSO_SELECT
        )
    )
    .fetch_all(pool)
    .await
}

pub async fn find_all_filtered(
    pool: &DbPool,
    search: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<WsoOrder>, sqlx::Error> {

    let search_pattern = search.map(|value| format!("%{}%", value));

    query_as::<_, WsoOrder>(
        &format!(
            r#"
            {}

            WHERE

                ($1::TEXT IS NULL OR w.wso_number ILIKE $1)

                AND

                ($2::TEXT IS NULL OR w.status = $2)

            ORDER BY w.id DESC
            "#,
            WSO_SELECT
        )
    )
    .bind(search_pattern)
    .bind(status)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<WsoOrder, sqlx::Error> {

    query_as::<_, WsoOrder>(
        &format!(
            "{} WHERE w.id = $1",
            WSO_SELECT
        )
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    wso: &WsoOrder,
) -> Result<WsoOrder, sqlx::Error> {

    sqlx::query(
        r#"
        UPDATE wso_orders

        SET

            category_id = $1,

            date_signed = $2,

            wso_number = $3,

            req_number = $4,

            description = $5,

            design_code = $6,

            fabric_code = $7,

            remarks = $8,

            attachment_name = $9,

            attachment_path = $10,

            status = $11,

            updated_at = NOW()

        WHERE id = $12
        "#
    )
    .bind(wso.category_id)
    .bind(wso.date_signed)
    .bind(&wso.wso_number)
    .bind(&wso.req_number)
    .bind(&wso.description)
    .bind(&wso.design_code)
    .bind(&wso.fabric_code)
    .bind(&wso.remarks)
    .bind(&wso.attachment_name)
    .bind(&wso.attachment_path)
    .bind(&wso.status)
    .bind(wso.id)
    .execute(pool)
    .await?;

    find_by_id(pool, wso.id).await
}

pub async fn cancel(
    pool: &DbPool,
    id: i32,
) -> Result<WsoOrder, sqlx::Error> {

    sqlx::query(
        r#"
        UPDATE wso_orders

        SET

            status = 'cancelled',

            updated_at = NOW()

        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id).await
}

pub async fn reactivate(
    pool: &DbPool,
    id: i32,
) -> Result<WsoOrder, sqlx::Error> {

    sqlx::query(
        r#"
        UPDATE wso_orders

        SET

            status = 'active',

            updated_at = NOW()

        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id).await
}