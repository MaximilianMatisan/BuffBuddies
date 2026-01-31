use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::common::user_mod::user::UserInformation;

pub async fn get_user_information_from_server(
    jwt: String,
) -> Result<UserInformation, ServerRequestError> {
    let response = reqwest::Client::new()
        .get("http://127.0.0.1:3000/user/info/get")
        .header("Authorization", format!("Token {jwt}"))
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    let response = response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    //println!("{}", response.text().await.unwrap());

    let data = response
        .json::<UserInformation>()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    Ok(data)
}
