mod database;
mod models;
mod handlers;
mod app_state;
mod routes;

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
// use database::DbPool;
// use handlers::wso::{create_wso, get_wsos, get_wso, update_wso, cancel_wso};
// use axum::routing::post;
use routes::wso::routes as wso_routes;

//response returned from home route

// to read later:
// -structs
// -derive macros

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
    };

    //creating route
    let app = Router::new()
        .merge(wso_routes())
        .route("/", get(root))
        .with_state(state);
    //start listening
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}