use sqlx::{query_as};

use crate::{
    database::DbPool,
    models::wso::{CreateWsoRequest, WsoOrder},
};

pub async fn create(
    pool: &DbPool,
    payload: &CreateWsoRequest,
) -> Result<WsoOrder, sqlx::Error> {
    query_as::<_, WsoOrder>(
        r#"
        INSERT INTO wso_orders
            (wso_number, req_number, description, remarks)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            wso_number,
            req_number,
            description,
            remarks,
            status,
            created_at,
            updated_at
        "#,
    )
    .bind(&payload.wso_number)
    .bind(&payload.req_number)
    .bind(&payload.description)
    .bind(&payload.remarks)
    .fetch_one(pool)
    .await
}

pub async fn find_all(pool: &DbPool) -> Result<Vec<WsoOrder>, sqlx::Error> {
    query_as::<_, WsoOrder>(
        r#"
        SELECT
            id,
            wso_number,
            req_number,
            description,
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

pub async fn find_by_id(pool: &DbPool, id: i32) -> Result<WsoOrder, sqlx::Error> {
    query_as::<_, WsoOrder>(
        r#"
        SELECT
            id,
            wso_number,
            req_number,
            description,
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
            wso_number = $1,
            req_number = $2,
            description = $3,
            remarks = $4,
            status = $5,
            updated_at = NOW()
        WHERE id = $6
        RETURNING
            id,
            wso_number,
            req_number,
            description,
            remarks,
            status,
            created_at,
            updated_at
        "#,
    )
    .bind(&wso.wso_number)
    .bind(&wso.req_number)
    .bind(&wso.description)
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
        SET status = 'cancelled'
        WHERE id = $1
        RETURNING
            id,
            wso_number,
            req_number,
            description,
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
