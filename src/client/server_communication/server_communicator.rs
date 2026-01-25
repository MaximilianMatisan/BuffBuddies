use crate::client::backend::exercise::exercise_create::ExerciseCreate;
use crate::client::backend::exercise::set::{Reps, StrengthSet};
use crate::client::backend::exercise::weight::Kg;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::mascot_trait;
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

#[derive(Debug, Clone)]
pub enum RequestValidUserError {
    ServerError,
    UserNotFound,
    WrongPassword,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "answer")]
pub enum RequestValidUserAnswer {
    UserNotFound,
    WrongPassword,
    Valid,
}

#[derive(Debug, Deserialize, Serialize)]
struct MascotJson {
    name: String,
}

impl From<Mascot> for MascotJson {
    fn from(mascot: Mascot) -> Self {
        MascotJson {
            name: mascot_trait::MascotTrait::get_name(&mascot).to_string(),
        }
    }
}

pub fn valid_login(login_request: LoginRequest) -> Result<(), RequestValidUserError> {
    let res = reqwest::blocking::Client::new()
        .get("http://127.0.0.1:3000/user/login")
        .json(&login_request)
        .send()
        .expect("checking user login went wrong");

    match res.json() {
        Ok(answer) => match answer {
            RequestValidUserAnswer::UserNotFound => Err(RequestValidUserError::UserNotFound),
            RequestValidUserAnswer::WrongPassword => Err(RequestValidUserError::WrongPassword),
            RequestValidUserAnswer::Valid => Ok(()),
        },
        Err(_e) => Err(RequestValidUserError::ServerError),
    }
}

#[derive(Clone, Debug)]
pub enum SaveMascotError {
    ServerError,
}

pub fn save_mascot(mascot: Mascot) -> Result<Mascot, SaveMascotError> {
    let mascot_json: MascotJson = mascot.into();
    let res = reqwest::blocking::Client::new()
        .post("http://127.0.0.1:3000/mascot/save")
        .json(&mascot_json)
        .send();
    match res {
        Ok(_) => Ok(mascot),
        Err(_server_error) => Err(SaveMascotError::ServerError),
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

pub fn save_workout(
    username: String,
    workout: Vec<ExerciseCreate>,
) -> Result<(), SaveWorkoutError> {
    let workout_json: WorkoutJson = WorkoutJson::new(username, workout);
    let res = reqwest::blocking::Client::new()
        .post("http://127.0.0.1:3000/workout/save")
        .json(&workout_json)
        .send();
    match res {
        Ok(_) => Ok(()),
        Err(_server_error) => Err(SaveWorkoutError::ServerError),
    }
}
