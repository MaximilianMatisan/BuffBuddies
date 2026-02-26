use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::client::server_communication::server_communicator::LoginRequest;
use crate::common::login::{
    RequestValidRegisterAnswer, RequestValidRegisterError, RequestValidUserAnswer,
    RequestValidUserError,
};
use crate::common::user_mod::friend_request::FriendRequest;
use crate::common::user_mod::user::{ForeignUser, UserInformation};

/// Checks if the login data exists on serverside
/// Returns jwt if login was successful else RequestValidUserError
pub async fn valid_login(login_request: LoginRequest) -> Result<String, RequestValidUserError> {
    let res = reqwest::Client::new()
        .post("http://127.0.0.1:3000/user/login")
        .json(&login_request)
        .send()
        .await
        .map_err(|_| RequestValidUserError::ServerError)?;
    let res = res
        .error_for_status()
        .map_err(|_| RequestValidUserError::ServerError)?;

    match res.json::<RequestValidUserAnswer>().await {
        Ok(answer) => match answer {
            RequestValidUserAnswer::UserNotFound => Err(RequestValidUserError::UserNotFound),
            RequestValidUserAnswer::WrongPassword => Err(RequestValidUserError::WrongPassword),
            RequestValidUserAnswer::Valid(jwt) => Ok(jwt),
        },
        Err(_e) => Err(RequestValidUserError::ServerError),
    }
}

pub async fn valid_register(
    login_request: LoginRequest,
) -> Result<String, RequestValidRegisterError> {
    let res = reqwest::Client::new()
        .post("http://127.0.0.1:3000/user/register")
        .json(&login_request)
        .send()
        .await
        .map_err(|_| RequestValidRegisterError::ServerError)?;
    let res = res
        .error_for_status()
        .map_err(|_| RequestValidRegisterError::ServerError)?;

    match res.json::<RequestValidRegisterAnswer>().await {
        Ok(answer) => match answer {
            RequestValidRegisterAnswer::UserAlreadyExists => {
                Err(RequestValidRegisterError::UserAlreadyExists)
            }
            RequestValidRegisterAnswer::Valid(jwt) => Ok(jwt),
        },
        Err(_e) => Err(RequestValidRegisterError::ServerError),
    }
}

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

pub async fn update_user_info_on_server(
    jwt: String,
    new_user_info: UserInformation,
) -> Result<(), ServerRequestError> {
    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3000/user/info/update")
        .header("Authorization", format!("Token {jwt}"))
        .json(&new_user_info)
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?; //TODO create variant could not send data

    response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    Ok(())
}

pub async fn get_foreign_users_from_server(
    jwt: String,
) -> Result<Vec<ForeignUser>, ServerRequestError> {
    let response = reqwest::Client::new()
        .get("http://127.0.0.1:3000/user/foreign/get")
        .header("Authorization", format!("Token {jwt}"))
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    let response = response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    //println!("{}", response.text().await.unwrap());

    let data = response
        .json::<Vec<ForeignUser>>()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    Ok(data)
}
pub async fn add_foreign_user_as_friend_on_server(
    jwt: String,
    friend_request: FriendRequest,
) -> Result<(), ServerRequestError> {
    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3000/user/foreign/add_friend")
        .header("Authorization", format!("Token {jwt}"))
        .json(&friend_request)
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    Ok(())
}

pub async fn remove_foreign_user_as_friend_on_server(
    jwt: String,
    friend_request: FriendRequest,
) -> Result<(), ServerRequestError> {
    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3000/user/foreign/remove_friend")
        .header("Authorization", format!("Token {jwt}"))
        .json(&friend_request)
        .send()
        .await
        .map_err(|_| ServerRequestError::CouldNotRetrieveData)?;

    response
        .error_for_status()
        .map_err(|_| ServerRequestError::HTTPError)?;

    Ok(())
}
