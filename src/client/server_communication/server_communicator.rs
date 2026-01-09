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
struct JsonMascot {
    name: String,
}

impl From<Mascot> for JsonMascot {
    fn from(mascot: Mascot) -> Self {
        JsonMascot {
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
    let mascot_json: JsonMascot = mascot.into();
    let res = reqwest::blocking::Client::new()
        .post("http://127.0.0.1:3000/mascot/save")
        .json(&mascot_json)
        .send();
    match res {
        Ok(_) => Ok(mascot),
        Err(_server_error) => Err(SaveMascotError::ServerError),
    }
}
