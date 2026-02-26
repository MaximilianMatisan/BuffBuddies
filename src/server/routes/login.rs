use crate::common::login::{RequestValidRegisterAnswer, RequestValidUserAnswer};
use crate::server::database_mod::database::{add_user, check_user, get_all_usernames};
use crate::server::jwt::jwt_architecture::create_jwt;
use crate::server::server_main::ApiError;
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub async fn check_login(
    State(pool): State<SqlitePool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<RequestValidUserAnswer>, ApiError> {
    match check_user(&pool, &login_request.username, &login_request.password).await? {
        RequestValidUserAnswer::Valid(_) => {
            let jwt = create_jwt(login_request.username);
            Ok(Json(RequestValidUserAnswer::Valid(jwt)))
        }
        RequestValidUserAnswer::UserNotFound => Ok(Json(RequestValidUserAnswer::UserNotFound)),
        RequestValidUserAnswer::WrongPassword => Ok(Json(RequestValidUserAnswer::WrongPassword)),
    }
}

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<RequestValidRegisterAnswer>, ApiError> {
    let users = get_all_usernames(&pool).await?;
    if !users.contains(&login_request.username) || users.is_empty() {
        add_user(&pool, &login_request.username, &login_request.password).await?;
        let jwt = create_jwt(login_request.username);
        Ok(Json(RequestValidRegisterAnswer::Valid(jwt)))
    } else {
        Ok(Json(RequestValidRegisterAnswer::UserAlreadyExists))
    }
}
