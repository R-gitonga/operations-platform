use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use serde::Deserialize;
use tokio::fs;

use crate::{
    app_state::AppState,
    authenticated_user::AuthenticatedUser,
    errors::app_error::AppError,
    models::{
        create_purchase_order::CreatePurchaseOrderRequest,
        purchase_order::{PurchaseOrder, UpdatePurchaseOrderRequest},
        purchase_order_detail::PurchaseOrderDetail,
    },
    services::purchase_order,
};

pub async fn create_purchase_order(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(payload): Json<CreatePurchaseOrderRequest>,
) -> Result<Json<PurchaseOrderDetail>, AppError> {
    let created =
        purchase_order::create_complete(
            &state.pool,
            &payload,
            &user.name,
        )
        .await?;

    Ok(Json(created))
}

#[derive(Debug, Deserialize)]
pub struct ListPurchaseOrderQuery {
    pub search: Option<String>,
    pub status: Option<String>,
}

pub async fn get_purchase_orders(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Query(query): Query<ListPurchaseOrderQuery>,
) -> Result<Json<Vec<PurchaseOrder>>, AppError> {
    let orders = if query.search.is_none() && query.status.is_none() {
        purchase_order::find_all(&state.pool).await?
    } else {
        purchase_order::find_all_filtered(
            &state.pool,
            query.search.as_deref(),
            query.status.as_deref(),
        )
        .await?
    };

    Ok(Json(orders))
}

pub async fn get_purchase_order(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<PurchaseOrderDetail>, AppError> {
    let detail =
        purchase_order::get_detail(&state.pool, id).await?;

    Ok(Json(detail))
}

pub async fn update_purchase_order(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePurchaseOrderRequest>,
) -> Result<Json<PurchaseOrder>, AppError> {
    let updated =
        purchase_order::update(&state.pool, id, &payload).await?;

    Ok(Json(updated))
}

pub async fn cancel_purchase_order(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<PurchaseOrder>, AppError> {
    let cancelled =
        purchase_order::cancel(&state.pool, id).await?;

    Ok(Json(cancelled))
}

pub async fn reactivate_purchase_order(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
) -> Result<Json<PurchaseOrder>, AppError> {
    let order =
        purchase_order::reactivate(&state.pool, id).await?;

    Ok(Json(order))
}

pub async fn upload_attachment(
    State(state): State<AppState>,
    _user: AuthenticatedUser,
    Path(id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<PurchaseOrder>, AppError> {
    let mut saved: Option<(String, String)> = None;

    while let Some(field) = multipart.next_field().await? {
        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "attachment".to_string());

        let data = field.bytes().await?;

        fs::create_dir_all("uploads").await?;

        let saved_path =
            format!("uploads/po_{}_{}", id, file_name);

        fs::write(&saved_path, data).await?;

        saved = Some((file_name, saved_path));
    }

    let (attachment_name, attachment_path) =
        saved.ok_or(AppError::BadRequest(
            "No file was uploaded.".to_string(),
        ))?;

    let updated =
        purchase_order::update_attachment(
            &state.pool,
            id,
            &attachment_name,
            &attachment_path,
        )
        .await?;

    Ok(Json(updated))
}