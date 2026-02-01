use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum RequestValidUserError {
    ServerError,
    UserNotFound,
    WrongPassword,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "answer", content = "token")]
pub enum RequestValidUserAnswer {
    UserNotFound,
    WrongPassword,
    Valid(String),
}
