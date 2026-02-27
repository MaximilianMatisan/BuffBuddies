use crate::common::exercise_mod::exercise::Exercise;

//TODO move into server_communicator.rs
#[derive(Debug, Clone)]
pub enum ServerRequestError {
    /// Jsonwebtoken is missing or not correct
    NoJWTValidation,
    CouldNotRetrieveData,
    CouldNotSendData,
    /// HTTP request Error mostly used when error code is sent back
    HTTPError,
}
impl ServerRequestError {
    pub fn to_error_message(&self) -> String {
        let slice = match self {
            ServerRequestError::NoJWTValidation => {
                "Could not connect to server due to missing validation!"
            }
            ServerRequestError::CouldNotRetrieveData => {
                "Could not retrieve the data from the server!"
            }
            ServerRequestError::CouldNotSendData => "Could not send the data to the server!",
            ServerRequestError::HTTPError => "The http request failed!",
        };
        slice.to_string()
    }
}

pub async fn get_exercise_data_from_server(
    jwt: String,
) -> Result<Vec<Exercise>, ServerRequestError> {
    let response = reqwest::Client::new()
        .get("http://127.0.0.1:3000/user/exercises")
        .header("Authorization", format!("Token {jwt}"))
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    let response = response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    let data = response
        .json::<Vec<Exercise>>()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    Ok(data)
}
