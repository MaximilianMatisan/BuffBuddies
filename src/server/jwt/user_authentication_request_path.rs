use crate::server::jwt::jwt_architecture::decode_jwt;
use axum::Json;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use serde_json::json;

pub enum JWTAuthenticationError {
    MissingAuthorizationHeader,
    CouldNotFindToken,
    WrongToken,
}
impl IntoResponse for JWTAuthenticationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            JWTAuthenticationError::MissingAuthorizationHeader => (
                StatusCode::BAD_REQUEST,
                "Couldn't find authentication header",
            ),
            JWTAuthenticationError::CouldNotFindToken => {
                (StatusCode::BAD_REQUEST, "Token missing in authentication")
            }
            JWTAuthenticationError::WrongToken => (
                StatusCode::UNAUTHORIZED,
                "Authentication token doesn't match",
            ),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
pub struct UserAuthenticationRequestPath {
    pub username: String,
}

impl<S: Sync> FromRequestParts<S> for UserAuthenticationRequestPath {
    type Rejection = JWTAuthenticationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|header_value| header_value.to_str().ok())
            .ok_or(JWTAuthenticationError::MissingAuthorizationHeader)?;

        let token = header
            .strip_prefix("Token ")
            .ok_or(JWTAuthenticationError::CouldNotFindToken)?;

        let username = decode_jwt(token).ok_or(JWTAuthenticationError::WrongToken)?;

        Ok(UserAuthenticationRequestPath { username })
    }
}
