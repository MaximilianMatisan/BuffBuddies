use axum::Json;
use axum::extract::{Path, State};
use sqlx::SqlitePool;

use crate::server::server_main::ApiError;
use crate::{
    common::exercise_mod::exercise::Exercise, server::database_mod::database::get_exercises_stats,
};

pub async fn get_user_exercises(
    State(pool): State<SqlitePool>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Exercise>>, ApiError> {
    let exercises = get_exercises_stats(&pool, &username).await?;
    Ok(Json(exercises))
}
