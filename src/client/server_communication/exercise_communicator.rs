use std::sync::Arc;
use crate::common::exercise_mod::exercise::Exercise;

#[derive(Debug, Clone)]
pub enum ServerRequestError {
    CouldNotRetrieveData,
    HTTPError
}
impl ServerRequestError {
    pub fn to_error_message(&self) -> String {
        let slice = match self {
            ServerRequestError::CouldNotRetrieveData => "Could not retrieve the data from the server!",
            ServerRequestError::HTTPError => "The http request failed!"
        };
        slice.to_string()
    }
}

pub async fn get_exercise_data_from_server(username: String) -> Result<Arc<Vec<Exercise>>, ServerRequestError> {
    let url = format!("http://127.0.0.1:3000/users/{username}/exercises");

    let response = reqwest::get(url).await.map_err(|_| ServerRequestError::CouldNotRetrieveData)?;
    let response = response.error_for_status().map_err(|_| ServerRequestError::HTTPError)?;

    let data = response.json::<Vec<Exercise>>().await.map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    Ok(Arc::new(data))
}
