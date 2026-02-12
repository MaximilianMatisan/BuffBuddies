use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::common::mascot_mod::mascot::Mascot;
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
pub async fn update_selected_mascot_on_server(
    jwt: String,
    new_mascot: Mascot,
) -> Result<(), ServerRequestError> {
    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3000/mascot/select")
        .header("Authorization", format!("Token {jwt}"))
        .json(&new_mascot)
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    //println!("{}", response.status());

    response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    Ok(())
}

pub async fn buy_mascot(jwt: String, mascot: Mascot) -> Result<Mascot, ServerRequestError> {
    let res = reqwest::Client::new()
        .post("http://127.0.0.1:3000/mascot/buy")
        .header("Authorization", format!("Token {jwt}"))
        .json(&mascot)
        .send()
        .await;
    match res {
        Ok(_) => Ok(mascot),
        Err(_server_error) => Err(ServerRequestError::CouldNotSendData),
    }
}
