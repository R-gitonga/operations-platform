use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use serde::Deserialize;

// use std::path::Path;
use tokio::fs;

use crate::{
    app_state::AppState,
    errors::app_error::AppError,
    models::{
        create_complete_wso::CreateCompleteWsoRequest,
        wso::{UpdateWsoRequest, WsoOrder},
        wso_detail::WsoDetail,
        wso_summary::WsoSummary,
    },
    repositories::wso,
    services::{
        wso as wso_service,
        wso_create as wso_create_service,
    },
};

pub async fn create_wso(
    State(state): State<AppState>,
    Json(payload): Json<CreateCompleteWsoRequest>,
) -> Result<Json<WsoDetail>, AppError> {
    let created = wso_create_service::create_complete_wso(&state.pool, &payload).await?;
    Ok(Json(created))
}

#[derive(Debug, Deserialize)]
pub struct ListWsoQuery {
    pub search: Option<String>,
    pub status: Option<String>,
}

pub async fn get_wsos(
    State(state): State<AppState>,
    Query(query): Query<ListWsoQuery>,
) -> Result<Json<Vec<WsoOrder>>, AppError> {
    let wsos = if query.search.is_none() && query.status.is_none() {
        wso::find_all(&state.pool).await?
    } else {
        wso::find_all_filtered(
            &state.pool,
            query.search.as_deref(),
            query.status.as_deref(),
        )
        .await?
    };

    Ok(Json(wsos))
}

pub async fn get_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<WsoDetail>, AppError> {
    let wso_detail = wso_service::get_wso_detail(&state.pool, id).await?;
    Ok(Json(wso_detail))
}

pub async fn update_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateWsoRequest>,
) -> Result<Json<WsoOrder>, AppError> {
    let mut wso_record = wso::find_by_id(&state.pool, id).await?;

    if let Some(val) = payload.wso_number {
        wso_record.wso_number = val;
    }
    if let Some(val) = payload.category_id {
        wso_record.category_id = Some(val);
    }
    if let Some(val) = payload.date_signed {
        wso_record.date_signed = Some(val);
    }
    if let Some(val) = payload.req_number {
        wso_record.req_number = Some(val);
    }
    if let Some(val) = payload.description {
        wso_record.description = Some(val);
    }
    if let Some(val) = payload.design_code {
        wso_record.design_code = Some(val);
    }
    if let Some(val) = payload.fabric_code {
        wso_record.fabric_code = Some(val);
    }
    if let Some(val) = payload.remarks {
        wso_record.remarks = Some(val);
    }
    if let Some(val) = payload.status {
        wso_record.status = val;
    }

    let updated = wso::update(&state.pool, &wso_record).await?;
    Ok(Json(updated))
}

pub async fn cancel_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<WsoOrder>, AppError> {
    let cancelled = wso_service::cancel(&state.pool, id).await?;
    Ok(Json(cancelled))
}

pub async fn upload_attachment(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    mut multipart: Multipart,
) -> Result<Json<WsoOrder>, AppError> {

    let mut wso = wso::find_by_id(&state.pool, id).await?;

    while let Some(field) = multipart.next_field().await? {
        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "attachment".to_string());

        let data = field.bytes().await?;

        fs::create_dir_all("uploads").await?;

        let saved_path = format!("uploads/{}_{}", id, file_name);

        fs::write(&saved_path, data).await?;

        wso.attachment_name = Some(file_name);
        wso.attachment_path = Some(saved_path);
    }

    let updated = wso::update(&state.pool, &wso).await?;

    Ok(Json(updated))
}

pub async fn get_wso_summary(
    State(state): State<AppState>,
) -> Result<Json<WsoSummary>, AppError> {
    let summary = wso_service::get_wso_summary(&state.pool).await?;
    Ok(Json(summary))
}
