use crate::{
    app_state::AppState, authenticated_user::AuthenticatedUser, errors::app_error::AppError,
    services::paid_orders as service,
};
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn branch_management_requires_admin_and_valid_fields() {
        let user = serde_json::from_value(serde_json::json!({
            "id":1,"name":"Tester","email":"test@example.invalid","password_hash":"unused",
            "role":"user","active":true,"created_at":"2026-10-26T00:00:00Z","updated_at":"2026-10-26T00:00:00Z"
        })).unwrap();
        let mut user = AuthenticatedUser(user);
        let mut branch = BranchInput { code:"TEST".into(), name:"Test branch".into(), active:true };
        assert!(matches!(validate_branch(&user,&branch), Err(AppError::Forbidden)));
        user.0.role="admin".into();
        assert!(validate_branch(&user,&branch).is_ok());
        branch.code=" ".into();
        assert!(validate_branch(&user,&branch).is_err());
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/branches", get(branches).post(create_branch))
        .route("/branches/{id}", axum::routing::put(update_branch))
        .route("/paid-orders", get(list))
        .route(
            "/order-classification/{kind}/{id}",
            get(metadata).put(correct),
        )
}
async fn branches(
    State(s): State<AppState>,
    _u: AuthenticatedUser,
) -> Result<Json<Vec<Value>>, AppError> {
    Ok(Json(
        sqlx::query_scalar("SELECT to_jsonb(b) FROM branches b ORDER BY active DESC,name,id")
            .fetch_all(&s.pool)
            .await?,
    ))
}
#[derive(Deserialize)]
struct BranchInput {
    code: String,
    name: String,
    active: bool,
}
fn validate_branch(u: &AuthenticatedUser, p: &BranchInput) -> Result<(), AppError> {
    if u.0.role != "admin" {
        return Err(AppError::Forbidden);
    }
    if p.code.trim().is_empty()
        || p.code.trim().len() > 30
        || p.name.trim().is_empty()
        || p.name.trim().len() > 150
    {
        return Err(AppError::BadRequest(
            "Branch code (1–30 characters) and name (1–150 characters) are required.".into(),
        ));
    }
    Ok(())
}
fn branch_error(e: sqlx::Error) -> AppError {
    if e.as_database_error()
        .is_some_and(|d| d.is_unique_violation())
    {
        AppError::BadRequest("That branch code already exists.".into())
    } else {
        e.into()
    }
}
async fn create_branch(
    State(s): State<AppState>,
    u: AuthenticatedUser,
    Json(p): Json<BranchInput>,
) -> Result<Json<Value>, AppError> {
    validate_branch(&u, &p)?;
    Ok(Json(
        sqlx::query_scalar(
            "INSERT INTO branches(code,name,active) VALUES($1,$2,$3) RETURNING to_jsonb(branches)",
        )
        .bind(p.code.trim())
        .bind(p.name.trim())
        .bind(p.active)
        .fetch_one(&s.pool)
        .await
        .map_err(branch_error)?,
    ))
}
async fn update_branch(
    State(s): State<AppState>,
    u: AuthenticatedUser,
    Path(id): Path<i32>,
    Json(p): Json<BranchInput>,
) -> Result<Json<Value>, AppError> {
    validate_branch(&u, &p)?;
    // Codes are stable reporting identifiers; only name and availability change.
    Ok(Json(sqlx::query_scalar("UPDATE branches SET name=$2,active=$3,updated_at=now() WHERE id=$1 AND code=$4 RETURNING to_jsonb(branches)")
        .bind(id).bind(p.name.trim()).bind(p.active).bind(p.code.trim()).fetch_one(&s.pool).await.map_err(branch_error)?))
}
async fn list(
    State(s): State<AppState>,
    _u: AuthenticatedUser,
    Query(f): Query<service::Filters>,
) -> Result<Json<service::Page>, AppError> {
    Ok(Json(service::list(&s.pool, &f).await?))
}
async fn metadata(
    State(s): State<AppState>,
    _u: AuthenticatedUser,
    Path((kind, id)): Path<(String, i32)>,
) -> Result<Json<Value>, AppError> {
    Ok(Json(service::metadata(&s.pool, &kind, id).await?))
}
async fn correct(
    State(s): State<AppState>,
    u: AuthenticatedUser,
    Path((kind, id)): Path<(String, i32)>,
    Json(p): Json<service::Classification>,
) -> Result<Json<Value>, AppError> {
    Ok(Json(
        service::correct(&s.pool, &kind, id, &p, &u.0.name).await?,
    ))
}
