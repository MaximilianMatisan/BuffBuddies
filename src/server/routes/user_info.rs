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

pub async fn update_user_info(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
    Json(new_user_info): Json<UserInformation>,
) -> Result<(), ApiError> {
    //TODO create a database function that combines these fields in one update query
    database::update_user_favorite_mascot(
        &pool,
        &user_authentication.username,
        &new_user_info.favorite_mascot,
    )
    .await?;

    database::update_user_gender(
        &pool,
        &user_authentication.username,
        &new_user_info.gender.to_string(),
    )
    .await?;

    database::update_user_height(&pool, &user_authentication.username, new_user_info.height)
        .await?;

    database::update_user_weight(&pool, &user_authentication.username, new_user_info.weight)
        .await?;

    database::update_user_description(
        &pool,
        &user_authentication.username,
        &new_user_info.description,
    )
    .await?;

    //TODO database::update_user_goals(&pool, &user_authenticator.name, new_user_info.user_goals)

    println!(
        "{}: Updated UserInformation was saved in the database!",
        user_authentication.username
    );

    Ok(())
}
