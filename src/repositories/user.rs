use sqlx::query_as;

use crate::{
    database::DbPool,
    models::user::User,
};

const USER_SELECT: &str = r#"
SELECT
    id,
    name,
    email,
    password_hash,
    role,
    active,
    created_at,
    updated_at
FROM users
"#;

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<User>, sqlx::Error> {
    query_as::<_, User>(
        &format!("{} ORDER BY id DESC", USER_SELECT),
    )
    .fetch_all(pool)
    .await
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<User, sqlx::Error> {
    query_as::<_, User>(
        &format!("{} WHERE id = $1", USER_SELECT),
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn find_by_email(
    pool: &DbPool,
    email: &str,
) -> Result<User, sqlx::Error> {
    query_as::<_, User>(
        &format!(
            "{} WHERE LOWER(email) = LOWER($1)",
            USER_SELECT
        ),
    )
    .bind(email)
    .fetch_one(pool)
    .await
}

pub async fn create(
    pool: &DbPool,
    user: &User,
) -> Result<User, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO users
        (
            name,
            email,
            password_hash,
            role,
            active
        )
        VALUES
        (
            $1,$2,$3,$4,$5
        )
        RETURNING id
        "#,
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.role)
    .bind(user.active)
    .fetch_one(pool)
    .await?;

    let id: i32 = sqlx::Row::get(&row, "id");

    find_by_id(pool, id).await
}

pub async fn update(
    pool: &DbPool,
    user: &User,
) -> Result<User, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET
            name = $1,
            email = $2,
            password_hash = $3,
            role = $4,
            active = $5,
            updated_at = NOW()
        WHERE id = $6
        "#,
    )
    .bind(&user.name)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(&user.role)
    .bind(user.active)
    .bind(user.id)
    .execute(pool)
    .await?;

    find_by_id(pool, user.id).await
}