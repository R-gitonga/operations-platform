use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};

use serde_json::json;
use sqlx::query;
use sqlx::query_as;


use crate::{
    app_state::AppState,
    models::wso::{CreateWsoRequest, UpdateWsoRequest, WsoOrder},

};

pub async fn create_wso(
    State(state): State<AppState>,
    Json(payload): Json<CreateWsoRequest>,
) -> Json<serde_json::Value> {
    query(
        r#"
        INSERT INTO wso_orders
        (wso_number, req_number, description, remarks)
        VALUES ($1, $2, $3, $4)
        "#
    )

    .bind(&payload.wso_number)
    .bind(&payload.req_number)
    .bind(&payload.description)
    .bind(&payload.remarks)
    .execute(&state.pool)
    .await
    .unwrap();

    Json(json!({
        "message": "WSO Created Successfully"
    }))
}

pub async fn get_wsos(
    State(state): State<AppState>,
) -> Json<Vec<WsoOrder>> {

    let wsos = query_as::<_, WsoOrder>(
    r#"
        SELECT
            id,
            wso_number,
            req_number,
            description,
            remarks,
            status,
            created_at,
            updated_at
        FROM wso_orders
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();

    Json(wsos)
}

pub async fn get_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<WsoOrder>, StatusCode> {

    let wso = sqlx::query_as::<_, WsoOrder>(
        r#"
        SELECT
            id,
            wso_number,
            req_number,
            description,
            remarks,
            status,
            created_at,
            updated_at
        FROM wso_orders
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await;

    match wso {
        Ok(record) => Ok(Json(record)),

        Err(sqlx::Error::RowNotFound) => {
            Err(StatusCode::NOT_FOUND)
        }

        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateWsoRequest>,
) -> Result<Json<WsoOrder>, StatusCode> {

    let existing = sqlx::query_as::<_, WsoOrder>(
        r#"
        SELECT 
            id,
            wso_number,
            req_number,
            description,
            remarks,
            status,
            created_at,
            updated_at
        FROM wso_orders
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_one(&state.pool)
    .await;

    let mut wso = match existing {
        Ok(record) => record,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if let Some(val) = payload.wso_number {
        wso.wso_number = val;
    }

    if let Some(val) = payload.req_number {
        wso.req_number = Some(val);
    }
    
    if let Some(val) = payload.description {
        wso.description = Some(val);
    }

    if let Some(val) = payload.remarks {
        wso.remarks = Some(val);
    }

    if let Some(val) = payload.status {
        wso.status = val;
    }

    let updated = sqlx::query_as::<_, WsoOrder>(
        r#"
        UPDATE wso_orders
        SET 
            wso_number = $1,
            req_number = $2,
            description = $3,
            remarks = $4,
            status = $5,
            updated_at = NOW()
        WHERE id = $6
        RETURNING
            id,
            wso_number,
            req_number,
            description,
            remarks,
            status,
            created_at,
            updated_at
        "#
    )
    .bind(&wso.wso_number)
    .bind(&wso.req_number)
    .bind(&wso.description)
    .bind(&wso.remarks)
    .bind(&wso.status)
    .bind(id)
    .fetch_one(&state.pool)
    .await
    .map_err(|err| {
        eprintln!("update_wso sql error: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(updated))
}

pub async fn cancel_wso(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Json<serde_json::Value> {
    sqlx::query(
        r#"
        UPDATE wso_orders
        SET status = 'cancelled'
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .unwrap();

    Json(json!({
        "message": "WSO cancelled Successfully"
    }))
}