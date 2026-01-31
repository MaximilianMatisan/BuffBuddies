use crate::server::database_mod::database;
use crate::server::database_mod::database::init_db;
use crate::server::routes::login::check_login;
use crate::server::routes::mascot_manager::save_mascot;
use crate::server::routes::user_exercises::get_user_exercises;
use crate::server::routes::workout::save_workout;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use database::init_pool;
use serde_json::json;
use sqlx::SqlitePool;
use tokio;
use crate::server::routes::user_info::get_user_info;

#[derive(Debug)]
#[allow(dead_code)] //TODO: construct variants `NotFound`, `InvalidInput`, and `InternalError`
pub enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
    DatabaseError,
    DatabaseRowNotFound,
}
impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ApiError::DatabaseRowNotFound,
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
            ApiError::DatabaseRowNotFound => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database entry doesn't exist!".to_string(),
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
        .route("/user/login", get(check_login))
        .route("/mascot/save", post(save_mascot))
        .route("/workout/save", post(save_workout))
        .route("/users/{username}/exercises", get(get_user_exercises))
        .route("/users/{username}/info/get", get(get_user_info))
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
