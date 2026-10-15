use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::user::User,
    services::password,
};

pub async fn authenticate(
    pool: &DbPool,
    email: &str,
    password: &str,
) -> Result<User, AppError> {

    let email = email
        .trim()
        .to_lowercase();

    if email.is_empty() {
        return Err(
            AppError::Validation(
                "Email cannot be empty.".into(),
            ),
        );
    }

    if password.is_empty() {
        return Err(
            AppError::Validation(
                "Password cannot be empty.".into(),
            ),
        );
    }

    let user =
        match sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1",
        )
        .bind(&email)
        .fetch_one(pool)
        .await
        {
            Ok(user) => user,

            Err(sqlx::Error::RowNotFound) => {
                return Err(
                    AppError::Validation(
                        "Invalid email or password.".into(),
                    ),
                );
            }

            Err(error) => {
                return Err(error.into());
            }
        };

    if !user.active {
        return Err(
            AppError::Validation(
                "This user account is inactive.".into(),
            ),
        );
    }

    let valid =
        password::verify_password(
            password,
            &user.password_hash,
        )
        .map_err(AppError::Validation)?;

    if !valid {
        return Err(
            AppError::Validation(
                "Invalid email or password.".into(),
            ),
        );
    }

    Ok(user)
}