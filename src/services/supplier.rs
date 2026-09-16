use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        create_supplier_request::CreateSupplierRequest,
        supplier::Supplier,
        update_supplier_request::UpdateSupplierRequest,
    },
    repositories::supplier,
};

fn validate_name(name: &str) -> Result<(), AppError> {
    if name.trim().is_empty() {
        return Err(AppError::BadRequest(
            "Supplier name cannot be empty".to_string(),
        ));
    }

    Ok(())
}

pub async fn find_all(
    pool: &DbPool,
) -> Result<Vec<Supplier>, AppError> {
    Ok(supplier::find_all(pool).await?)
}

pub async fn find_active(
    pool: &DbPool,
) -> Result<Vec<Supplier>, AppError> {
    Ok(supplier::find_active(pool).await?)
}

pub async fn find_by_id(
    pool: &DbPool,
    id: i32,
) -> Result<Supplier, AppError> {
    Ok(
        supplier::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?
    )
}

pub async fn create(
    pool: &DbPool,
    payload: &CreateSupplierRequest,
) -> Result<Supplier, AppError> {
    let name = payload.name.trim();

    validate_name(name)?;

    let contact_name = payload
        .contact_name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let contact_info = payload
        .contact_info
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    Ok(
        supplier::create(pool, name, contact_name, contact_info)
            .await?
    )

}

pub async fn update(
    pool: &DbPool,
    id: i32,
    payload: &UpdateSupplierRequest,
) -> Result<Supplier, AppError> {
    let name = payload.name.trim();

    validate_name(name)?;

    supplier::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    let contact_name = payload
        .contact_name
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let contact_info = payload
        .contact_info
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let updated = supplier::update(pool, id, name, contact_name, contact_info)
        .await?;

    if payload.active {
        Ok(supplier::activate(pool, id).await?)
    } else {
        Ok(supplier::deactivate(pool, id).await?)
    }
}

pub async fn activate(
    pool: &DbPool,
    id: i32,
) -> Result<Supplier, AppError> {
    let existing = supplier::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    if existing.active {
        return Err(AppError::BadRequest(
            "Supplier is already active.".to_string(),
        ));
    }

    Ok(supplier::activate(pool, id).await?)
}

pub async fn deactivate(
    pool: &DbPool,
    id: i32,
) -> Result<Supplier, AppError> {
    let existing = supplier::find_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)?;

    if !existing.active {
        return Err(AppError::BadRequest(
            "Supplier is already inactive.".to_string(),
        ));
    }

    Ok(supplier::deactivate(pool, id).await?)
}