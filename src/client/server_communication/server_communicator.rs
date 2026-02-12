use crate::client::backend::exercise_create::ExerciseCreate;
use crate::common::exercise_mod::set::{Reps, StrengthSet};
use crate::common::exercise_mod::weight::Kg;
use crate::common::login::{RequestValidUserAnswer, RequestValidUserError};
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

/// Checks if the login data exists on serverside
/// Returns jwt if login was successful else RequestValidUserError
pub async fn valid_login(login_request: LoginRequest) -> Result<String, RequestValidUserError> {
    let res = reqwest::Client::new()
        .post("http://127.0.0.1:3000/user/login")
        .json(&login_request)
        .send()
        .await
        .map_err(|_| RequestValidUserError::ServerError)?;
    let res = res
        .error_for_status()
        .map_err(|_| RequestValidUserError::ServerError)?;

    match res.json::<RequestValidUserAnswer>().await {
        Ok(answer) => match answer {
            RequestValidUserAnswer::UserNotFound => Err(RequestValidUserError::UserNotFound),
            RequestValidUserAnswer::WrongPassword => Err(RequestValidUserError::WrongPassword),
            RequestValidUserAnswer::Valid(jwt) => Ok(jwt),
        },
        Err(_e) => Err(RequestValidUserError::ServerError),
    }
}

#[derive(Clone, Debug)]
pub enum SaveWorkoutError {
    ServerError,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutJson {
    username: String,
    workout: Vec<ExerciseJson>,
}

impl WorkoutJson {
    pub fn new(username: String, workout: Vec<ExerciseCreate>) -> Self {
        WorkoutJson {
            username,
            workout: {
                let mut workout_json = Vec::new();
                for exercise in workout {
                    workout_json.push(exercise.into());
                }
                workout_json
            },
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
    weight: Kg,
    reps: Reps,
}

impl From<StrengthSet> for SetJson {
    fn from(strength_set: StrengthSet) -> Self {
        SetJson {
            weight: strength_set.weight,
            reps: strength_set.reps,
        }
    }
}

pub async fn save_workout(
    username: String,
    workout: Vec<ExerciseCreate>,
) -> Result<(), SaveWorkoutError> {
    let workout_json: WorkoutJson = WorkoutJson::new(username, workout);
    let res = reqwest::Client::new()
        .post("http://127.0.0.1:3000/workout/save")
        .json(&workout_json)
        .send()
        .await;
    match res {
        Ok(_) => Ok(()),
        Err(_server_error) => Err(SaveWorkoutError::ServerError),
    }
}
