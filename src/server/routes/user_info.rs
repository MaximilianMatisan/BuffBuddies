use axum::extract::State;

use crate::client::backend::profile_stat_manager::ProfileStatManager;
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
    let description = database::get_user_description(&pool, &user_authentication.username).await?;
    let profile_picture_handle =
        database::get_user_profile_picture(&pool, &user_authentication.username).await?;
    let weight = database::get_user_weight(&pool, &user_authentication.username).await?;
    let height = database::get_user_height(&pool, &user_authentication.username).await?;
    let gender = database::get_user_gender(&pool, &user_authentication.username).await?;
    let weekly_workout_goal =
        database::get_user_weekly_workout_goal(&pool, &user_authentication.username).await?;
    let weekly_workout_streak =
        database::get_user_weekly_workout_streak(&pool, &user_authentication.username).await?;
    let coin_balance =
        database::get_user_coin_balance(&pool, &user_authentication.username).await?;
    let favorite_mascot =
        database::get_user_favorite_mascot(&pool, &user_authentication.username).await?;

    let temp_exercises =
        database::get_exercises_stats(&pool, &user_authentication.username).await?; //TODO save in database to avoid multiple fetching of exercises
    let profile_stat_manager = ProfileStatManager::new(&temp_exercises);

    let user_information = UserInformation {
        username: user_authentication.username.clone(),
        description,
        profile_picture_handle,
        weight,
        height: height as u32,
        gender,
        weekly_workout_goal,
        weekly_workout_streak,
        coin_balance,
        favorite_mascot,
        profile_stat_manager,
    };

    println!(
        "Sending UserInformation data to {}",
        user_authentication.username
    );

    Ok(Json(user_information))
}
