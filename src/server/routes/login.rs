use crate::server::jwt::jwt_architecture::create_jwt;
use crate::server::server_main::ApiError;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "answer", content = "token")]
pub enum RequestValidUserAnswer {
    UserNotFound,
    WrongPassword,
    Valid(String),
}

pub async fn check_login(
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<RequestValidUserAnswer>, ApiError> {
    if login_request.username == "Felix" {
        if login_request.password == "password" {
            let jwt = create_jwt(login_request.username);
            println!("Sent session code to client: {}", jwt.clone());
            Ok(Json(RequestValidUserAnswer::Valid(jwt)))
        } else {
            Ok(Json(RequestValidUserAnswer::WrongPassword))
        }
    } else {
        Ok(Json(RequestValidUserAnswer::UserNotFound))
    }
}
