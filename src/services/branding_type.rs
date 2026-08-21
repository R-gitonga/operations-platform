use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        branding_type::BrandingType,
        create_branding_type_request::CreateBrandingTypeRequest,
        update_branding_type_request::UpdateBrandingTypeRequest,
    },
    repositories::branding_type,
};

fn validate_code(code: &str) -> Result<(), AppError> {
    if code.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Branding type code cannot be empty".to_string(),
        ));
    }

    Ok(())
}

fn validate_display_name(display_name: &str) -> Result<(), AppError> {
    if display_name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Branding type display name cannot be empty".to_string(),
        ));
    }

    Ok(())
}

fn validate_display_order(display_order: i32) -> Result<(), AppError> {
    if display_order < 0 {
        return Err(AppError::BadRequest(
            "Branding type display order cannot be negative".to_string(),
        ));
    }

    Ok(())
}

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<BrandingType>, AppError> {
    Ok(branding_type::find_all(pool).await?)
}

pub async fn find_active(
    pool: &DbPool,
) -> Result<Vec<BrandingType>, AppError> {
    Ok(branding_type::find_active(pool).await?)
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<BrandingType, AppError> {
    Ok(
        branding_type::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?
    )
}

pub async fn create(
    pool: &DbPool,
    payload: &CreateBrandingTypeRequest,
) -> Result<BrandingType, AppError> {
    let code = payload.code.trim();
    let display_name = payload.display_name.trim();

    validate_code(code)?;
    validate_display_name(display_name)?;
    validate_display_order(payload.display_order)?;

    Ok(
        branding_type::create(
            pool,
            code,
            display_name,
            payload.description.as_deref(),
            payload.display_order,
        )
        .await?
    )
}

pub async fn update(
    pool: &DbPool,
    id: i32,
    payload: &UpdateBrandingTypeRequest,
) -> Result<BrandingType, AppError> {
    let code = payload.code.trim();
    let display_name = payload.display_name.trim();

    validate_code(code)?;
    validate_display_name(display_name)?;
    validate_display_order(payload.display_order)?;

    branding_type::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(
        branding_type::update(
            pool,
            id,
            code,
            display_name,
            payload.description.as_deref(),
            payload.display_order,
        )
        .await?
    )
}

pub async fn activate(
    pool: &DbPool,
    id: i32,
) -> Result<BrandingType, AppError> {
    let existing = branding_type::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    if existing.active {
        return Err(AppError::BadRequest(
            "Branding type is already active.".to_string(),
        ));
    }

    Ok(branding_type::activate(pool, id).await?)
}

pub async fn deactivate(
    pool: &DbPool,
    id: i32,
) -> Result<BrandingType, AppError> {
    let existing = branding_type::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    if !existing.active {
        return Err(AppError::BadRequest(
            "Branding type is already inactive.".to_string(),
        ));
    }

    Ok(branding_type::deactivate(pool, id).await?)
}