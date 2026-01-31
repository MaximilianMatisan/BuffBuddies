use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::common::user_mod::user::UserInformation;

pub async fn get_user_information_from_server(username: String) -> Result<UserInformation, ServerRequestError> {
    let url = format!("http://127.0.0.1:3000/users/{username}/info/get");

    let response = reqwest::get(url).await.map_err(|_| ServerRequestError::CouldNotRetrieveData)?;
    let response = response.error_for_status().map_err(|_| ServerRequestError::HTTPError)?;

    let data = response.json::<UserInformation>().await.map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    Ok(data)
}
