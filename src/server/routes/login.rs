use crate::common::login::{RequestValidRegisterAnswer, RequestValidUserAnswer};
use crate::server::database_mod::database::{
    RequestPasswordAnswer, add_user, get_all_usernames, get_password,
};
use crate::server::jwt::jwt_architecture::create_jwt;
use crate::server::server_main::ApiError;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn check_login(
    State(pool): State<SqlitePool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<RequestValidUserAnswer>, ApiError> {
    match get_password(&pool, &login_request.username).await? {
        RequestPasswordAnswer::Password(password_hash) => {
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&password_hash)?;
            match argon2.verify_password(login_request.password.as_bytes(), &parsed_hash) {
                Ok(_) => {
                    let jwt = create_jwt(login_request.username);
                    Ok(Json(RequestValidUserAnswer::Valid(jwt)))
                }
                Err(_) => Ok(Json(RequestValidUserAnswer::WrongPassword)),
            }
        }
        RequestPasswordAnswer::UserNotFound => Ok(Json(RequestValidUserAnswer::UserNotFound)),
    }
}

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(login_request): Json<LoginRequest>,
) -> Result<Json<RequestValidRegisterAnswer>, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(login_request.password.as_bytes(), &salt)?;
    let users = get_all_usernames(&pool).await?;
    if !users.contains(&login_request.username) || users.is_empty() {
        add_user(&pool, &login_request.username, &password_hash.to_string()).await?;
        let jwt = create_jwt(login_request.username);
        Ok(Json(RequestValidRegisterAnswer::Valid(jwt)))
    } else {
        Ok(Json(RequestValidRegisterAnswer::UserAlreadyExists))
    }
}
