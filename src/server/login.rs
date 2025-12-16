use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::server::server_main::ApiError;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "answer")]
pub enum RequestValidUserAnswer {
    UserNotFound,
    WrongPassword,
    Valid,
}

pub async fn check_login(Json(login_request): Json<LoginRequest>) -> Result<Json<RequestValidUserAnswer>, ApiError> {
    if login_request.username == "Felix" {
        if login_request.password == "password" {
            Ok(Json(RequestValidUserAnswer::Valid))
        } else {
            Ok(Json(RequestValidUserAnswer::WrongPassword))
        }
    } else {
        Ok(Json(RequestValidUserAnswer::UserNotFound))
    }
}