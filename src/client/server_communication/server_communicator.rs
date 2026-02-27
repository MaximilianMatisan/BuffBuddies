use crate::client::backend::exercise_create::{ExerciseCreate, StrengthSetCreate, WorkoutCreate};
use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::common::exercise_mod::general_exercise::Id;
use crate::common::exercise_mod::set::Reps;
use crate::common::exercise_mod::weight::Kg;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

impl From<(String, String)> for LoginRequest {
    fn from((username, password): (String, String)) -> Self {
        LoginRequest { username, password }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutJson {
    workout: Vec<ExerciseJson>,
    first_workout: bool,
}

impl WorkoutJson {
    pub fn new(workout: Vec<ExerciseCreate>, first_workout: bool) -> Self {
        WorkoutJson {
            workout: {
                let mut workout_json = Vec::new();
                for exercise in workout {
                    workout_json.push(exercise.into());
                }
                workout_json
            },
            first_workout,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExerciseJson {
    name: String,
    sets: Vec<SetJson>,
}

impl From<ExerciseCreate> for ExerciseJson {
    fn from(exercise_create: ExerciseCreate) -> Self {
        ExerciseJson {
            name: exercise_create.name,
            sets: {
                let mut sets = Vec::new();
                for set in exercise_create.sets {
                    sets.push(set.into());
                }
                sets
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetJson {
    pub(crate) weight: Kg,
    pub(crate) reps: Reps,
}

impl From<StrengthSetCreate> for SetJson {
    fn from(strength_set: StrengthSetCreate) -> Self {
        SetJson {
            weight: strength_set.weight,
            reps: strength_set.reps,
        }
    }
}

pub async fn save_workout(
    jwt: String,
    workout: WorkoutCreate,
    first_workout: bool,
) -> Result<Id, ServerRequestError> {
    let workout_json: WorkoutJson = WorkoutJson::new(workout, first_workout);
    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3000/workout/save")
        .json(&workout_json)
        .header("Authorization", format!("Token {jwt}"))
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    let response = response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    let data = response
        .json::<Id>()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    Ok(data)
}
