use crate::server::server_main::ApiError;
use axum::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutJson {
    username: String,
    workout: Vec<ExerciseJson>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExerciseJson {
    name: String,
    sets: Vec<crate::client::server_communication::server_communicator::SetJson>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetJson {
    weight: f64,
    reps: u32,
}

pub async fn save_workout(
    State(_state): State<SqlitePool>,
    Json(workout): Json<WorkoutJson>,
) -> Result<(), ApiError> {
    //TODO add to database
    println!("Workout received for: {}", workout.username);
    Ok(())
}
