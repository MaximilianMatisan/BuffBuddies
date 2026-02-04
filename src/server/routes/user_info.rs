use axum::extract::State;

use crate::common::user_mod::user::UserInformation;
use crate::server::database_mod::database;
use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;
use axum::Json;
use sqlx::SqlitePool;

pub async fn get_user_info(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
) -> Result<Json<UserInformation>, ApiError> {

    let user_info = database::get_user_information(&pool, &user_authentication.username).await?;

    println!(
        "{}: Fetching UserInformation Data!",
        user_authentication.username
    );

    Ok(Json(user_info))
}
