use crate::server::database_mod::database;
use crate::server::database_mod::database::init_db;
use crate::server::routes::foreign_users::{add_friend, get_foreign_users, remove_friend};
use crate::server::routes::login::check_login;
use crate::server::routes::mascot_manager::{buy_mascot, get_mascot_data, select_mascot};
use crate::server::routes::user_exercises::get_user_exercises;
use crate::server::routes::user_info::{get_user_info, update_user_info};
use crate::server::routes::user_presets::{get_user_presets, save_preset};
use crate::server::routes::workout::save_workout;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use database::init_pool;
use serde_json::json;
use sqlx::SqlitePool;
use tokio;

#[derive(Debug)]
#[allow(dead_code)] //TODO: construct variants `NotFound`, `InvalidInput`, and `InternalError`
pub enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
    DatabaseError,
}
impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ApiError::NotFound,
            _ => ApiError::DatabaseError,
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not found".to_string()),
            ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            ApiError::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub async fn server_main() {
    let pool = create_database().await.expect("DB init failed");

    //test_database(&pool).await.expect("test_db_failed");

    println!("Launching Server!");
    let app = create_app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("failed to start server")
}

fn create_app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/server", get(health_check))
        .route("/user/login", post(check_login))
        .route("/mascot/buy", post(buy_mascot))
        .route("/mascot/get", get(get_mascot_data))
        .route("/mascot/select", post(select_mascot))
        .route("/workout/save", post(save_workout))
        .route("/preset/save", post(save_preset))
        .route("/user/exercises", get(get_user_exercises))
        .route("/user/presets", get(get_user_presets))
        .route("/user/info/get", get(get_user_info))
        .route("/user/info/update", post(update_user_info))
        .route("/user/foreign/get", get(get_foreign_users))
        .route("/user/foreign/add_friend", post(add_friend))
        .route("/user/foreign/remove_friend", post(remove_friend))
        .with_state(pool)
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}

pub async fn create_database() -> Result<SqlitePool, sqlx::Error> {
    let pool = init_pool().await?;
    init_db(&pool).await?;
    Ok(pool)
}
