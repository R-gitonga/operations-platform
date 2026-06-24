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
    .fetch_one(tx.as_mut())
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

pub async fn find_all_filtered(
    pool: &DbPool,
    search: Option<&str>,
    status: Option<&str>,
) -> Result<Vec<WsoOrder>, sqlx::Error> {
    let search_pattern = search.map(|value| format!("%{}%", value));
    let mut query = String::from(
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
        "#,
    );

    let mut conditions = Vec::new();
    if search.is_some() {
        conditions.push("wso_number ILIKE $1");
    }
    if status.is_some() {
        conditions.push("status = $2");
    }

    if !conditions.is_empty() {
        query.push_str("WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY id DESC");

    let mut builder = sqlx::query_as::<_, WsoOrder>(&query);
    if let Some(pattern) = search_pattern {
        builder = builder.bind(pattern);
    }
    if let Some(status_value) = status {
        builder = builder.bind(status_value);
    }

    builder.fetch_all(pool).await
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
