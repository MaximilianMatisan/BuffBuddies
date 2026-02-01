use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::common::mascot_mod::mascot_data_transfer::MascotDataServerClientTransfer;

pub async fn get_mascot_data_from_server(
    jwt: String,
) -> Result<MascotDataServerClientTransfer, ServerRequestError> {
    let response = reqwest::Client::new()
        .get("http://127.0.0.1:3000/mascot/get")
        .header("Authorization", format!("Token {jwt}"))
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    let response = response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    let data = response
        .json::<MascotDataServerClientTransfer>()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    Ok(data)
}
