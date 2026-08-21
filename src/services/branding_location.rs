use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        branding_location::BrandingLocation,
        create_branding_location_request::CreateBrandingLocationRequest,
        update_branding_location_request::UpdateBrandingLocationRequest,
    },
    repositories::branding_location,
};

fn validate_code(code: &str) -> Result<(), AppError> {
    if code.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Branding location code cannot be empty".to_string(),
        ));
    }

    Ok(())
}

fn validate_display_name(display_name: &str) -> Result<(), AppError> {
    if display_name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Branding location display name cannot be empty".to_string(),
        ));
    }

    Ok(())
}

fn validate_display_order(display_order: i32) -> Result<(), AppError> {
    if display_order < 0 {
        return Err(AppError::BadRequest(
            "Branding location display order cannot be negative".to_string(),
        ));
    }

    Ok(())
}

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<BrandingLocation>, AppError> {
    Ok(branding_location::find_all(pool).await?)
}

pub async fn find_active(
    pool: &DbPool,
) -> Result<Vec<BrandingLocation>, AppError> {
    Ok(branding_location::find_active(pool).await?)
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<BrandingLocation, AppError> {
    Ok(
        branding_location::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?
    )
}

pub async fn create(
    pool: &DbPool,
    payload: &CreateBrandingLocationRequest,
) -> Result<BrandingLocation, AppError> {
    let code = payload.code.trim();
    let display_name = payload.display_name.trim();

    validate_code(code)?;
    validate_display_name(display_name)?;
    validate_display_order(payload.display_order)?;

    Ok(
        branding_location::create(
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
    payload: &UpdateBrandingLocationRequest,
) -> Result<BrandingLocation, AppError> {
    let code = payload.code.trim();
    let display_name = payload.display_name.trim();

    validate_code(code)?;
    validate_display_name(display_name)?;
    validate_display_order(payload.display_order)?;

    branding_location::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(
        branding_location::update(
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
) -> Result<BrandingLocation, AppError> {
    let existing = branding_location::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    if existing.active {
        return Err(AppError::BadRequest(
            "Branding location is already active.".to_string(),
        ));
    }

    Ok(branding_location::activate(pool, id).await?)
}

pub async fn deactivate(
    pool: &DbPool,
    id: i32,
) -> Result<BrandingLocation, AppError> {
    let existing = branding_location::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    if !existing.active {
        return Err(AppError::BadRequest(
            "Branding location is already inactive.".to_string(),
        ));
    }

    Ok(branding_location::deactivate(pool, id).await?)
}