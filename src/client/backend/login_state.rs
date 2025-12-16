use crate::client::server_communicator::server_communicator::LoginRequest;

pub enum LoginStateError {
    UsernameEmpty,
    PasswordEmpty,
}

#[derive(Default, Debug)]
pub struct LoginState {
    pub username: String,
    pub password: String,
    pub logged_in: bool,
    pub error_text: String,
}

impl LoginState {
    pub fn try_login(&self) -> Result<LoginRequest, LoginStateError> {
        if self.username == "" {
            return Err(LoginStateError::UsernameEmpty)
        }
        if self.password == "" {
            return Err(LoginStateError::PasswordEmpty)
        }
        Ok((self.username.clone(), self.password.clone()).into())
    }
}