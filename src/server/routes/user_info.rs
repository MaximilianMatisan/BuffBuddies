use axum::extract::{Path, State};

use axum::Json;
use sqlx::SqlitePool;
use crate::client::backend::profile_stat_manager::ProfileStatManager;
use crate::common::user_mod::user::UserInformation;
use crate::server::database_mod::database;
use crate::server::server_main::ApiError;

pub async fn get_user_info(
    State(pool): State<SqlitePool>,
    Path(username): Path<String>
) ->  Result<Json<UserInformation>, ApiError> {
    let description = database::get_user_description(&pool, &username).await?;
    let profile_picture_handle = database::get_user_profile_picture(&pool, &username).await?;
    let weight = database::get_user_weight(&pool, &username).await?;
    let height = database::get_user_height(&pool, &username).await?;
    let gender = database::get_user_gender(&pool, &username).await?;
    let weekly_workout_goal = database::get_user_weekly_workout_goal(&pool, &username).await?;
    let weekly_workout_streak = database::get_user_weekly_workout_streak(&pool, &username).await?;
    let coin_balance = database::get_user_coin_balance(&pool,&username).await?;
    let favorite_mascot = database::get_user_favorite_mascot(&pool, &username).await?;

    let temp_exercises = database::get_exercises_stats(&pool, &username).await?; //TODO save in database to avoid multiple fetching of exercises
    let profile_stat_manager = ProfileStatManager::new(&temp_exercises);

    let user_information = UserInformation {
        username,
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

    Ok(Json(user_information))
}