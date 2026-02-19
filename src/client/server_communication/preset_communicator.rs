use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::common::workout_preset::WorkoutPreset;

pub async fn get_preset_data_from_server(
    jwt: String,
) -> Result<Vec<WorkoutPreset>, ServerRequestError> {
    let response = reqwest::Client::new()
        .get("http://127.0.0.1:3000/user/presets")
        .header("Authorization", format!("Token {jwt}"))
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    let response = response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    //println!("{}", response.status());
    let data = response
        .json::<Vec<WorkoutPreset>>()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    Ok(data)
}

pub async fn save_preset(jwt: String, preset: WorkoutPreset) -> Result<(), ServerRequestError> {
    let res = reqwest::Client::new()
        .post("http://127.0.0.1:3000/preset/save")
        .json(&preset)
        .header("Authorization", format!("Token {jwt}"))
        .send()
        .await;
    match res {
        Ok(_) => Ok(()),
        Err(_server_error) => Err(ServerRequestError::CouldNotSendData),
    }
}
