use crate::client::server_communication::user_communicator::LoginRequest;

#[derive(Debug, PartialEq)]
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
    /// checks if the current login state has a non-empty username and password
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_error_message_password_empty() {
        let login_state_error = LoginStateError::PasswordEmpty;
        assert_eq!(
            login_state_error.to_error_message(),
            "Password can't be empty!".to_string()
        );
    }
    #[test]
    fn to_error_message_username_empty() {
        let login_state_error = LoginStateError::UsernameEmpty;
        assert_eq!(
            login_state_error.to_error_message(),
            "Username can't be empty!".to_string()
        );
    }
    #[test]
    fn try_login_no_username() {
        let login_state = LoginState {
            username: "".to_string(),
            password: "123".to_string(),
            state: Default::default(),
            error_text: "".to_string(),
        };
        assert_eq!(login_state.try_login(), Err(LoginStateError::UsernameEmpty));
    }
    #[test]
    fn try_login_no_password() {
        let login_state = LoginState {
            username: "123".to_string(),
            password: "".to_string(),
            state: Default::default(),
            error_text: "".to_string(),
        };
        assert_eq!(login_state.try_login(), Err(LoginStateError::PasswordEmpty));
    }

    #[test]
    fn try_login_all_filled() {
        let login_state = LoginState {
            username: "123".to_string(),
            password: "1234".to_string(),
            state: Default::default(),
            error_text: "".to_string(),
        };
        let login_request = LoginRequest {
            username: "123".to_string(),
            password: "1234".to_string(),
        };
        assert_eq!(login_state.try_login(), Ok(login_request));
    }
}
