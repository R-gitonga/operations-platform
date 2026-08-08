mod database;
mod models;
mod handlers;
mod app_state;
mod authenticated_user;
mod errors;
mod routes;
mod repositories;
mod services;
mod config;

use axum::{
    routing::get,
    Json,
    Router,
};

use app_state::AppState;
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use std::env;

use routes::{
    wso::routes as wso_routes,
    line_item::routes as line_item_routes,
    category::routes as category_routes,
    dashboard::routes as dashboard_routes,
    settings::routes as settings_routes,
    notification_recipient::routes as notification_recipient_route,
    debug::routes as debug_route,
    production_stage::routes as production_Stage_route,
    auth::routes as auth_routes,
    users::routes as users_routes,
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

    let config = config::Config::from_env()
        .expect("Failed to load application configuration");

    println!("Starting WSO Tracker API...");

    let database_url =
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to Database");

    let state = AppState {
        pool,
        config,
    };
    let worker_pool = state.pool.clone();

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
        .nest_service(
            "/uploads",
            ServeDir::new("uploads"),
        )
        .route("/", get(root))
        .with_state(state);
    //start listening
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    tokio::spawn(async move {

        loop {

            if let Err(error) =
                crate::services::notification_worker::process_pending_jobs(
                    &worker_pool,
                )
                .await
            {
                eprintln!(
                    "Notification Worker Error: {:?}",
                    error,
                );
            }

            tokio::time::sleep(
                std::time::Duration::from_secs(10),
            )
            .await;
        }
});

    axum::serve(listener, app)
        .await
        .unwrap();
}
