use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::user::User,
    repositories::user,
    services::{
        password,
        user_rules,
    },
};

pub async fn list(
    pool: &DbPool,
) -> Result<Vec<User>, sqlx::Error> {
    user::find_all(pool).await
}

pub async fn get(
    pool: &DbPool,
    id: i32,
) -> Result<User, sqlx::Error> {
    user::find_by_id(pool, id).await
}

pub async fn get_by_email(
    pool: &DbPool,
    email: &str,
) -> Result<User, sqlx::Error> {
    user::find_by_email(pool, email).await
}

pub async fn create(
    pool: &DbPool,
    name: &str,
    email: &str,
    password: &str,
    role: &str,
) -> Result<User, AppError> {

    let name = name.trim().to_string();
    let email = email.trim().to_lowercase();
    let role = role.trim().to_string();

    user_rules::ensure_name_not_empty(&name)?;
    user_rules::ensure_email_not_empty(&email)?;
    user_rules::ensure_email_valid(&email)?;
    user_rules::ensure_password_valid(password)?;
    user_rules::ensure_role_not_empty(&role)?;

    let existing =
        user::find_by_email(
            pool,
            &email,
        )
        .await;

    match existing {
        Ok(_) => {
            return Err(
                AppError::Validation(
                    format!(
                        "A user with email '{}' already exists.",
                        email
                    ),
                ),
            );
        }

        Err(sqlx::Error::RowNotFound) => {}

        Err(error) => {
            return Err(error.into());
        }
    }

    let password_hash =
        password::hash_password(password)
            .map_err(AppError::Validation)?;

    let new_user = User {
        id: 0,
        name,
        email,
        password_hash,
        role,
        active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    Ok(
        user::create(
            pool,
            &new_user,
        )
        .await?
    )
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    name: &str,
    email: &str,
    role: &str,
    active: bool,
) -> Result<User, AppError> {

    let mut existing =
        user::find_by_id(
            pool,
            id,
        )
        .await?;

    let name = name.trim().to_string();
    let email = email.trim().to_lowercase();
    let role = role.trim().to_string();

    user_rules::ensure_name_not_empty(&name)?;
    user_rules::ensure_email_not_empty(&email)?;
    user_rules::ensure_email_valid(&email)?;
    user_rules::ensure_role_not_empty(&role)?;

    let email_owner =
        user::find_by_email(
            pool,
            &email,
        )
        .await;

    match email_owner {
        Ok(found) if found.id != id => {
            return Err(
                AppError::Validation(
                    format!(
                        "A user with email '{}' already exists.",
                        email
                    ),
                ),
            );
        }

        Ok(_) | Err(sqlx::Error::RowNotFound) => {}

        Err(error) => {
            return Err(error.into());
        }
    }

    existing.name = name;
    existing.email = email;
    existing.role = role;
    existing.active = active;

    Ok(
        user::update(
            pool,
            &existing,
        )
        .await?
    )
}

pub async fn deactivate(
    pool: &DbPool,
    id: i32,
) -> Result<User, AppError> {

    let mut existing =
        user::find_by_id(
            pool,
            id,
        )
        .await?;

    if !existing.active {
        return Err(
            AppError::Validation(
                format!(
                    "User '{}' is already inactive.",
                    existing.email
                ),
            ),
        );
    }

    existing.active = false;

    Ok(
        user::update(
            pool,
            &existing,
        )
        .await?
    )
}