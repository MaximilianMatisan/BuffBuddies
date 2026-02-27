use crate::client::server_communication::server_communicator::LoginRequest;

pub enum LoginStateError {
    UsernameEmpty,
    PasswordEmpty,
}
impl LoginStateError {
    pub fn to_error_message(&self) -> String {
        let slice = match self {
            LoginStateError::UsernameEmpty => "Username can't be empty!",
            LoginStateError::PasswordEmpty => "Password can't be empty!",
        };
        slice.to_string()
    }
}
#[derive(Eq, PartialEq, Default, Debug)]
pub enum LoginStates {
    #[default]
    NotLoggedIn,
    LoggedIn,
}

#[derive(Default, Debug)]
pub struct LoginState {
    pub username: String,
    pub password: String,
    pub state: LoginStates,
    pub error_text: String,
}

impl LoginState {
    pub fn try_login(&self) -> Result<LoginRequest, LoginStateError> {
        if self.username.is_empty() {
            return Err(LoginStateError::UsernameEmpty);
        }
        if self.password.is_empty() {
            return Err(LoginStateError::PasswordEmpty);
        }
        Ok((self.username.clone(), self.password.clone()).into())
    }
}
