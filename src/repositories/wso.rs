use sqlx::query_as;

use crate::{
    database::DbPool, 
    models::{
        wso::WsoOrder,
        create_complete_wso::CreateCompleteWsoRequest,
    },
};

const WSO_SELECT: &str = r#"
SELECT
    id,
    date_signed,
    wso_number,
    req_number,
    attachment_name,
    attachment_path,
    status,
    created_at,
    updated_at
FROM wso_orders
"#;

pub async fn create_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    payload: &CreateCompleteWsoRequest,
) -> Result<WsoOrder, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO wso_orders
        (
            date_signed,
            wso_number,
            req_number,
            attachment_name,
            attachment_path,
            status
        )
        VALUES
        (
            $1,$2,$3,$4,$5,'active'
        )
        RETURNING id
        "#,
    )
    .bind(payload.date_signed)
    .bind(&payload.wso_number)
    .bind(&payload.req_number)
    .bind(Option::<String>::None)
    .bind(Option::<String>::None)
    .fetch_one(tx.as_mut())
    .await?;

    let id: i32 = sqlx::Row::get(&row, "id");

    query_as::<_, WsoOrder>(&format!("{} WHERE id = $1", WSO_SELECT))
        .bind(id)
        .fetch_one(tx.as_mut())
        .await
}

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<WsoOrder>, sqlx::Error> {
    query_as::<_, WsoOrder>(
        &format!("{} ORDER BY id DESC", WSO_SELECT),
    )
    .fetch_all(pool)
    .await
}

pub async fn find_all_filtered(
    pool: &DbPool,
    search: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<WsoOrder>, sqlx::Error> {

    let search_pattern =
        search.map(|v| format!("%{}%", v));

    query_as::<_, WsoOrder>(
        &format!(
            r#"
            {}

            WHERE

                ($1::TEXT IS NULL OR wso_number ILIKE $1)

            AND

                ($2::TEXT IS NULL OR status = $2)

            ORDER BY id DESC
            "#,
            WSO_SELECT
        ),
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
        &format!("{} WHERE id = $1", WSO_SELECT),
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
            date_signed = $1,
            wso_number = $2,
            req_number = $3,
            attachment_name = $4,
            attachment_path = $5,
            status = $6,
            updated_at = NOW()
        WHERE id = $7
        "#,
    )
    .bind(wso.date_signed)
    .bind(&wso.wso_number)
    .bind(&wso.req_number)
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
        "#,
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
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    find_by_id(pool, id).await
}