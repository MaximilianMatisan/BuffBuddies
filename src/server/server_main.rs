use crate::server::login::check_login;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
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
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

pub async fn server_main() {
    println!("Launching Server!");
    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("failed to start server")
}

fn create_app() -> Router {
    Router::new()
        .route("/server", get(health_check))
        .route("/user/login", get(check_login))
}

async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "Server is running",
    }))
}
