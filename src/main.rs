mod app_state;
mod authenticated_user;
mod config;
mod database;
mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use axum::{routing::get, Json, Router};

use app_state::AppState;
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;

use routes::{
    auth::routes as auth_routes, branding::routes as branding_route,
    category::routes as category_routes, dashboard::routes as dashboard_routes,
    debug::routes as debug_route, line_item::routes as line_item_routes,
    notification_recipient::routes as notification_recipient_route,
    partial_receiving_attention::routes as partial_receiving_attention_route,
    po_dashboard::routes as po_dashboard_route, po_defect::routes as po_defect_route,
    po_line_item::routes as po_line_item_route, po_receipt::routes as po_receipt_route,
    production_stage::routes as production_Stage_route,
    purchase_order::routes as purchase_order_route, settings::routes as settings_routes,
    supplier::routes as supplier_route, users::routes as users_routes, wso::routes as wso_routes,
    wso_item_branding::routes as wso_item_branding_route,
    po_settings::routes as po_settings_route,
};

use tower_http::services::ServeDir;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

///GET /
async fn root() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: String::from("WSO Tracker API"),
    })
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = config::Config::from_env().expect("Failed to load application configuration");

    println!("Starting WSO Tracker API...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to Database");

    let state = AppState { pool, config };
    let worker_pool = state.pool.clone();
    let worker_config = state.config.clone();

    //creating route
    let app = Router::new()
        .merge(wso_routes())
        .merge(line_item_routes())
        .merge(category_routes())
        .merge(dashboard_routes())
        .merge(settings_routes())
        .merge(notification_recipient_route())
        .merge(debug_route())
        .merge(production_Stage_route())
        .merge(auth_routes())
        .merge(users_routes())
        .merge(partial_receiving_attention_route())
        .merge(branding_route())
        .merge(wso_item_branding_route())
        .merge(supplier_route())
        .merge(purchase_order_route())
        .merge(po_line_item_route())
        .merge(po_receipt_route())
        .merge(po_defect_route())
        .merge(po_dashboard_route())
        .nest_service(
            "/uploads",
            ServeDir::new("uploads"),
        )
        .route("/", get(root))
        .with_state(state);
    //start listening
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    tokio::spawn(async move {
        loop {
            if let Err(error) = crate::services::notification_worker::process_pending_jobs(
                &worker_pool,
                &worker_config,
            )
            .await
            {
                eprintln!("Notification Worker Error: {:?}", error,);
            }

            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    axum::serve(listener, app).await.unwrap();
}
