use crate::{
    database::DbPool,
    models::branding_type::BrandingType,
};

use sqlx::{Postgres, Transaction};

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<BrandingType>, sqlx::Error> {
    sqlx::query_as::<_, BrandingType>(
        r#"
        SELECT
            id,
            code,
            display_name,
            description,
            display_order,
            active,
            created_at,
            updated_at
        FROM branding_types
        ORDER BY display_order, display_name
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_active(
    pool: &DbPool,
) -> Result<Vec<BrandingType>, sqlx::Error> {
    sqlx::query_as::<_, BrandingType>(
        r#"
        SELECT
            id,
            code,
            display_name,
            description,
            display_order,
            active,
            created_at,
            updated_at
        FROM branding_types
        WHERE active = TRUE
        ORDER BY display_order, display_name
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<Option<BrandingType>, sqlx::Error> {
    sqlx::query_as::<_, BrandingType>(
        r#"
        SELECT
            id,
            code,
            display_name,
            description,
            display_order,
            active,
            created_at,
            updated_at
        FROM branding_types
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &DbPool,
    code: &str,
    display_name: &str,
    description: Option<&str>,
    display_order: i32,
) -> Result<BrandingType, sqlx::Error> {
    sqlx::query_as::<_, BrandingType>(
        r#"
        INSERT INTO branding_types (
            code,
            display_name,
            description,
            display_order
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            code,
            display_name,
            description,
            display_order,
            active,
            created_at,
            updated_at
        "#,
    )
    .bind(code)
    .bind(display_name)
    .bind(description)
    .bind(display_order)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    code: &str,
    display_name: &str,
    description: Option<&str>,
    display_order: i32,
) -> Result<BrandingType, sqlx::Error> {
    sqlx::query_as::<_, BrandingType>(
        r#"
        UPDATE branding_types
        SET
            code = $2,
            display_name = $3,
            description = $4,
            display_order = $5,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            code,
            display_name,
            description,
            display_order,
            active,
            created_at,
            updated_at
        "#,
    )
    .bind(id)
    .bind(code)
    .bind(display_name)
    .bind(description)
    .bind(display_order)
    .fetch_one(pool)
    .await
}

pub async fn activate(
    pool: &DbPool,
    id: i32,
) -> Result<BrandingType, sqlx::Error> {
    sqlx::query_as::<_, BrandingType>(
        r#"
        UPDATE branding_types
        SET
            active = TRUE,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            code,
            display_name,
            description,
            display_order,
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
) -> Result<BrandingType, sqlx::Error> {
    sqlx::query_as::<_, BrandingType>(
        r#"
        UPDATE branding_types
        SET
            active = FALSE,
            updated_at = NOW()
        WHERE id = $1
        RETURNING
            id,
            code,
            display_name,
            description,
            display_order,
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
) -> Result<Option<BrandingType>, sqlx::Error> {
    sqlx::query_as::<_, BrandingType>(
        r#"
        SELECT
            id,
            code,
            display_name,
            description,
            display_order,
            active,
            created_at,
            updated_at
        FROM branding_types
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(tx.as_mut())
    .await
}