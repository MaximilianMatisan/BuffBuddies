use crate::common::exercise_mod::general_exercise::Id;
use crate::server::database_mod::database::{
    add_workout_to_exercise_log, get_user_coin_balance, update_user_coin_balance,
};
use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;
use axum::Json;
use axum::extract::State;
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutJson {
    workout: Vec<ExerciseJson>,
    first_workout: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExerciseJson {
    pub(crate) name: String,
    pub(crate) sets: Vec<crate::client::server_communication::server_communicator::SetJson>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetJson {
    pub(crate) weight: f64,
    pub(crate) reps: u32,
}

pub async fn save_workout(
    user_authentication: UserAuthenticationRequestPath,
    State(pool): State<SqlitePool>,
    Json(workout): Json<WorkoutJson>,
) -> Result<Json<Id>, ApiError> {
    let workout_id = add_workout_to_exercise_log(
        &pool,
        &user_authentication.username,
        workout.workout,
        Local::now().date_naive(),
    )
    .await?;

    if workout.first_workout {
        let current_coins = get_user_coin_balance(&pool, &user_authentication.username).await?;
        update_user_coin_balance(&pool, &user_authentication.username, current_coins + 5).await?;
    }
    println!("{}: Workout received", user_authentication.username);

    Ok(Json(workout_id))
}
