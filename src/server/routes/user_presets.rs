use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

use crate::common::workout_preset::WorkoutPreset;
use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;

pub async fn get_user_presets(
    State(_pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
) -> Result<Json<Vec<WorkoutPreset>>, ApiError> {
    //let presets = get_user_presets(&pool, &user_authentication).await?;

    println!("{}: Fetching Presets Data!", user_authentication.username);

    //Ok(Json(presets))
    Ok(Json(Vec::new()))
}

pub async fn save_preset(
    user_authentication: UserAuthenticationRequestPath,
    State(_pool): State<SqlitePool>,
    Json(_preset): Json<WorkoutPreset>,
) -> Result<(), ApiError> {
    println!("{}: Preset received", user_authentication.username);
    Ok(())
}
