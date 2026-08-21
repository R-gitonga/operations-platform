use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::{
        create_wso_item_branding_request::CreateWsoItemBrandingRequest,
        wso_item_branding_detail::WsoItemBrandingDetail,
    },
    services::wso_item_branding,
};

pub async fn get_branding_requirements(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(wso_item_id): Path<i32>,
) -> Result<Json<Vec<WsoItemBrandingDetail>>, AppError> {
    let branding =
        wso_item_branding::find_details_by_wso_item(
            &state.pool,
            wso_item_id,
        )
        .await?;

    Ok(Json(branding))
}

pub async fn create_branding_requirement(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(wso_item_id): Path<i32>,
    Json(payload): Json<CreateWsoItemBrandingRequest>,
) -> Result<Json<WsoItemBrandingDetail>, AppError> {
    let created =
        wso_item_branding::create(
            &state.pool,
            wso_item_id,
            payload.branding_type_id,
            payload.branding_location_id,
            payload.quantity,
        )
        .await?;

    let branding =
        wso_item_branding::find_details_by_wso_item(
            &state.pool,
            wso_item_id,
        )
        .await?;

    let detail =
        branding
            .into_iter()
            .find(|item| item.id == created.id)
            .ok_or(AppError::NotFound)?;

    Ok(Json(detail))
}

pub async fn delete_branding_requirement(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<(), AppError> {
    wso_item_branding::delete(
        &state.pool,
        id,
    )
    .await?;

    Ok(())
}