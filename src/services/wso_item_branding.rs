use crate::{
    database::DbPool,
    errors::app_error::AppError,
    models::{
        wso_item_branding::WsoItemBranding,
        wso_item_branding_detail::WsoItemBrandingDetail
    },
    repositories::{
        branding_location,
        branding_type,
        wso_item_branding,
    },
};

pub async fn find_by_wso_item(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<Vec<WsoItemBranding>, AppError> {
    Ok(
        wso_item_branding::find_by_wso_item(
            pool,
            wso_item_id,
        )
        .await?
    )
}

pub async fn find_details_by_wso_item(
    pool: &DbPool,
    wso_item_id: i32,
) -> Result<Vec<WsoItemBrandingDetail>, AppError> {
    Ok(
        wso_item_branding::find_details_by_wso_item(
            pool,
            wso_item_id,
        )
        .await?
    )
}

pub async fn create(
    pool: &DbPool,
    wso_item_id: i32,
    branding_type_id: i32,
    branding_location_id: i32,
    quantity: i32,
) -> Result<WsoItemBranding, AppError> {

    if quantity <= 0 {
        return Err(AppError::BadRequest(
            "Branding quantity must be greater than zero."
                .to_string(),
        ));
    }

    let branding_type =
        branding_type::find_by_id(
            pool,
            branding_type_id,
        )
        .await?
        .ok_or(AppError::NotFound)?;

    if !branding_type.active {
        return Err(AppError::BadRequest(
            "The selected branding type is inactive."
                .to_string(),
        ));
    }

    let branding_location =
        branding_location::find_by_id(
            pool,
            branding_location_id,
        )
        .await?
        .ok_or(AppError::NotFound)?;

    if !branding_location.active {
        return Err(AppError::BadRequest(
            "The selected branding location is inactive."
                .to_string(),
        ));
    }

    Ok(
        wso_item_branding::create(
            pool,
            wso_item_id,
            branding_type_id,
            branding_location_id,
            quantity,
        )
        .await?
    )
}

pub async fn delete(
    pool: &DbPool,
    id: i32,
) -> Result<(), AppError> {
    let existing =
        wso_item_branding::find_by_id(
            pool,
            id,
        )
        .await?
        .ok_or(AppError::NotFound)?;

    wso_item_branding::delete(
        pool,
        existing.id,
    )
    .await?;

    Ok(())
}