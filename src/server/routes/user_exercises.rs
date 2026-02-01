use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;
use crate::{
    common::exercise_mod::exercise::Exercise, server::database_mod::database::get_exercises_stats,
};

pub async fn get_user_exercises(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
) -> Result<Json<Vec<Exercise>>, ApiError> {
    let exercises = get_exercises_stats(&pool, &user_authentication.username).await?;

    println!("Sending Exercise data to {}", user_authentication.username);

    Ok(Json(exercises))
}
