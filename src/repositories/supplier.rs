use crate::{
    database::DbPool,
    models::supplier::Supplier,
};

use sqlx::{Postgres, Transaction};

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<Supplier>, sqlx::Error> {
    sqlx::query_as::<_, Supplier>(
        r#"
        SELECT
            id,
            name,
            contact_name,
            contact_info,
            active,
            created_at,
            updated_at
        FROM suppliers
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_active(
    pool: &DbPool,
) -> Result<Vec<Supplier>, sqlx::Error> {
    sqlx::query_as::<_, Supplier>(
        r#"
        SELECT
            id,
            name,
            contact_name,
            contact_info,
            active,
            created_at,
            updated_at
        FROM suppliers
        WHERE active = TRUE
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<Option<Supplier>, sqlx::Error> {
    sqlx::query_as::<_, Supplier>(
        r#"
        SELECT
            id,
            name,
            contact_name,
            contact_info,
            active,
            created_at,
            updated_at
        FROM suppliers
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
  pool: &DbPool,
  name: &str,
  contact_name: Option<&str>,  
  contact_info: Option<&str>,
) -> Result<Supplier, sqlx::Error> {
    sqlx::query_as::<_, Supplier>(
        r#"
        INSERT INTO suppliers (
            name,
            contact_name,
            contact_info
        )
        VALUES ($1, $2, $3)
        RETURNING
            id,
            name,
            contact_name,
            contact_info,
            active,
            created_at,
            updated_at
        "#,
    )
    .bind(name)
    .bind(contact_name)
    .bind(contact_info)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    name: &str,
    contact_name: Option<&str>,
    contact_info: Option<&str>,
) -> Result<Supplier, sqlx::Error> {
    sqlx::query_as::<_, Supplier>(
        r#"
        UPDATE suppliers
        SET
            name = $2,
            contact_name = $3,
            contact_info = $4,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            name,
            contact_name,
            contact_info,
            active,
            created_at,
            updated_at
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(contact_name)
    .bind(contact_info)
    .fetch_one(pool)
    .await
}

pub async fn activate(
    pool: &DbPool,
    id: i32,
) -> Result<Supplier, sqlx::Error> {
    sqlx::query_as::<_, Supplier>(
        r#"
        UPDATE suppliers
        SET
            active = TRUE,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            name,
            contact_name,
            contact_info,
            active,
            created_at,
            updated_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn deactivate(
    pool: &DbPool,
    id: i32,
) -> Result<Supplier, sqlx::Error> {
    sqlx::query_as::<_, Supplier>(
        r#"
        UPDATE suppliers
        SET
            active = FALSE,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            name,
            contact_name,
            contact_info,
            active,
            created_at,
            updated_at
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn find_by_id_tx(
    tx: &mut Transaction<'_, Postgres>,
    id: i32,
) -> Result<Option<Supplier>, sqlx::Error> {
    sqlx::query_as::<_, Supplier>(
        r#"
        SELECT
            id,
            name,
            contact_name,
            contact_info,
            active,
            created_at,
            updated_at
        FROM suppliers
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(tx.as_mut())
    .await
}