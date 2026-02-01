use crate::common::user_mod::user::ForeignUser;
use crate::server::database_mod::database;
use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;
use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

pub async fn get_foreign_users(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
) -> Result<Json<Vec<ForeignUser>>, ApiError> {
    let mut non_friend_users =
        database::get_discovery_users(&pool, &user_authentication.username, 200).await?;
    let mut friends = database::get_all_friends(&pool, &user_authentication.username).await?;

    friends.append(&mut non_friend_users);

    println!(
        "Sending ForeignUser data to {}",
        user_authentication.username
    );

    Ok(Json(friends))
}
