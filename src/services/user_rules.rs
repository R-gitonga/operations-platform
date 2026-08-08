use crate::{
    errors::app_error::AppError,
    models::user::User,
};

pub fn ensure_name_not_empty(
    name: &str,
) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(
            AppError::Validation(
                "User name cannot be empty.".into(),
            ),
        );
    }

    Ok(())
}

pub fn ensure_email_not_empty(
    email: &str,
) -> Result<(), AppError> {
    if email.trim().is_empty() {
        return Err(
            AppError::Validation(
                "Email cannot be empty.".into(),
            ),
        );
    }

    Ok(())
}

pub fn ensure_email_valid(
    email: &str,
) -> Result<(), AppError> {
    let email = email.trim();

    if !email.contains('@')
        || !email.contains('.')
    {
        return Err(
            AppError::Validation(
                "Please provide a valid email address.".into(),
            ),
        );
    }

    Ok(())
}

pub fn ensure_password_valid(
    password: &str,
) -> Result<(), AppError> {
    if password.is_empty() {
        return Err(
            AppError::Validation(
                "Password cannot be empty.".into(),
            ),
        );
    }

    if password.chars().count() < 8 {
        return Err(
            AppError::Validation(
                "Password must be at least 8 characters long.".into(),
            ),
        );
    }

    Ok(())
}

pub fn ensure_role_not_empty(
    role: &str,
) -> Result<(), AppError> {
    if role.trim().is_empty() {
        return Err(
            AppError::Validation(
                "User role cannot be empty.".into(),
            ),
        );
    }

    Ok(())
}

pub fn validate(
    user: &User,
) -> Result<(), AppError> {
    ensure_name_not_empty(&user.name)?;
    ensure_email_not_empty(&user.email)?;
    ensure_email_valid(&user.email)?;
    ensure_role_not_empty(&user.role)?;

    Ok(())
}