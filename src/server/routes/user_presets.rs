use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::common::workout_preset::WorkoutPreset;
use crate::server::database_mod::database::{add_preset, add_preset_to_user, get_presets_for_user};
use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;

pub async fn get_user_presets(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
) -> Result<Json<Vec<WorkoutPreset>>, ApiError> {
    let presets = get_presets_for_user(&pool, &user_authentication.username).await?;
    println!("{}: Fetching Presets Data!", user_authentication.username);
    Ok(Json(presets))
}

pub async fn save_preset(
    user_authentication: UserAuthenticationRequestPath,
    State(pool): State<SqlitePool>,
    Json(preset): Json<WorkoutPreset>,
) -> Result<(), ApiError> {
    println!("{}: Preset received", user_authentication.username);
    let preset_id = add_preset(&pool, &preset, 10).await?;
    add_preset_to_user(&pool, &user_authentication.username, preset_id).await?;
    Ok(())
}
