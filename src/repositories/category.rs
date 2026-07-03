use sqlx::query_as;

use crate::{
    database::DbPool,
    models::category::Category,
};

pub async fn find_all(pool: &DbPool) -> Result<Vec<Category>, sqlx::Error> {
    query_as::<_, Category>(
        r#"
        SELECT id, name
        FROM categories
        ORDER BY name ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(pool: &DbPool, id: i32) -> Result<Category, sqlx::Error> {
    query_as::<_, Category>(
        r#"
        SELECT id, name
        FROM categories
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn create(pool: &DbPool, name: &str) -> Result<Category, sqlx::Error> {
    query_as::<_, Category>(
        r#"
        INSERT INTO categories (name)
        VALUES ($1)
        RETURNING id, name
        "#,
    )
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn update(pool: &DbPool, id: i32, name: &str) -> Result<Category, sqlx::Error> {
    query_as::<_, Category>(
        r#"
        UPDATE categories
        SET name = $1
        WHERE id = $2
        RETURNING id, name
        "#,
    )
    .bind(name)
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn delete(pool: &DbPool, id: i32) -> Result<Category, sqlx::Error> {
    query_as::<_, Category>(
        r#"
        DELETE FROM categories
        WHERE id = $1
        RETURNING id, name
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}
