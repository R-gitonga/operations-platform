use crate::{
    database::DbPool,
    models::branding_location::BrandingLocation,
};

use sqlx::{Postgres, Transaction};

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<BrandingLocation>, sqlx::Error> {
    sqlx::query_as::<_, BrandingLocation>(
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
        FROM branding_locations
        ORDER BY display_order, display_name
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_active(
    pool: &DbPool,
) -> Result<Vec<BrandingLocation>, sqlx::Error> {
    sqlx::query_as::<_, BrandingLocation>(
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
        FROM branding_locations
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
) -> Result<Option<BrandingLocation>, sqlx::Error> {
    sqlx::query_as::<_, BrandingLocation>(
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
        FROM branding_locations
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
) -> Result<BrandingLocation, sqlx::Error> {
    sqlx::query_as::<_, BrandingLocation>(
        r#"
        INSERT INTO branding_locations (
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
) -> Result<BrandingLocation, sqlx::Error> {
    sqlx::query_as::<_, BrandingLocation>(
        r#"
        UPDATE branding_locations
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
) -> Result<BrandingLocation, sqlx::Error> {
    sqlx::query_as::<_, BrandingLocation>(
        r#"
        UPDATE branding_locations
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
) -> Result<BrandingLocation, sqlx::Error> {
    sqlx::query_as::<_, BrandingLocation>(
        r#"
        UPDATE branding_locations
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
) -> Result<Option<BrandingLocation>, sqlx::Error> {
    sqlx::query_as::<_, BrandingLocation>(
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
        FROM branding_locations
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(tx.as_mut())
    .await
}