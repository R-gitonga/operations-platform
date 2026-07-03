use sqlx::query_as;

use crate::{
    database::DbPool,
    models::wso::WsoOrder,
};

pub async fn create_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    payload: &crate::models::create_complete_wso::CreateCompleteWsoRequest,
) -> Result<WsoOrder, sqlx::Error> {
    query_as::<_, WsoOrder>(
        r#"
        INSERT INTO wso_orders
            (category_id, date_signed, wso_number, req_number, description, design_code, fabric_code, remarks)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING
            id,
            category_id,
            date_signed,
            wso_number,
            req_number,
            description,
            design_code,
            fabric_code,
            remarks,
            status,
            created_at,
            updated_at
        "#,
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
    .await
}

pub async fn find_all(pool: &DbPool) -> Result<Vec<WsoOrder>, sqlx::Error> {
    query_as::<_, WsoOrder>(
        r#"
        SELECT
            id,
            category_id,
            date_signed,
            wso_number,
            req_number,
            description,
            design_code,
            fabric_code,
            remarks,
            status,
            created_at,
            updated_at
        FROM wso_orders
        ORDER BY id DESC
        "#,
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
        r#"
        SELECT
            id,
            category_id,
            date_signed,
            wso_number,
            req_number,
            description,
            design_code,
            fabric_code,
            remarks,
            status,
            created_at,
            updated_at
        FROM wso_orders
        WHERE ($1::TEXT IS NULL OR wso_number ILIKE $1)
          AND ($2::TEXT IS NULL OR status = $2)
        ORDER BY id DESC
        "#,
    )
    .bind(search_pattern)
    .bind(status)
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &DbPool, id: i32) -> Result<WsoOrder, sqlx::Error> {
    query_as::<_, WsoOrder>(
        r#"
        SELECT
            id,
            category_id,
            date_signed,
            wso_number,
            req_number,
            description,
            design_code,
            fabric_code,
            remarks,
            status,
            created_at,
            updated_at
        FROM wso_orders
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn update(pool: &DbPool, wso: &WsoOrder) -> Result<WsoOrder, sqlx::Error> {
    query_as::<_, WsoOrder>(
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
            status = $9,
            updated_at = NOW()
        WHERE id = $10
        RETURNING
            id,
            category_id,
            date_signed,
            wso_number,
            req_number,
            description,
            design_code,
            fabric_code,
            remarks,
            status,
            created_at,
            updated_at
        "#,
    )
    .bind(wso.category_id)
    .bind(wso.date_signed)
    .bind(&wso.wso_number)
    .bind(&wso.req_number)
    .bind(&wso.description)
    .bind(&wso.design_code)
    .bind(&wso.fabric_code)
    .bind(&wso.remarks)
    .bind(&wso.status)
    .bind(wso.id)
    .fetch_one(pool)
    .await
}

pub async fn cancel(pool: &DbPool, id: i32) -> Result<WsoOrder, sqlx::Error> {
    query_as::<_, WsoOrder>(
        r#"
        UPDATE wso_orders
        SET
            status = 'cancelled',
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            category_id,
            date_signed,
            wso_number,
            req_number,
            description,
            design_code,
            fabric_code,
            remarks,
            status,
            created_at,
            updated_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
