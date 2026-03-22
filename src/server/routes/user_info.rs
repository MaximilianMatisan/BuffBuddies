use axum::extract::State;

use crate::common::user_mod::user::UserInformation;
use crate::server::database_mod::{database_user, database_user_goals};
use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;
use axum::Json;
use sqlx::SqlitePool;

pub async fn get_user_info(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
) -> Result<Json<UserInformation>, ApiError> {
    let user_info =
        database_user::get_user_information(&pool, &user_authentication.username).await?;

    println!(
        "{}: Fetching UserInformation Data!",
        user_authentication.username
    );

    Ok(Json(user_info))
}

pub async fn update_user_info(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
    Json(new_user_info): Json<UserInformation>,
) -> Result<(), ApiError> {
    database_user_goals::update_user_goals(
        &pool,
        &user_authentication.username,
        &new_user_info.user_goals,
    )
    .await?;

    database_user::update_user_info_settings(&pool, &user_authentication.username, new_user_info)
        .await?;

    println!(
        "{}: Updated UserInformation was saved in the database!",
        user_authentication.username
    );

    Ok(())
}
