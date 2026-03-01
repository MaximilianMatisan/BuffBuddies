mod common;

use buff_buddies::client::gui::bb_tab::login::LoginMessage;
use buff_buddies::client::gui::user_interface::Message;
use buff_buddies::client::server_communication::user_communicator::LoginRequest;
use common::setups::client_setup;

#[test]
fn change_name_login() {
    let mut app = client_setup();
    let _ = app.update(Message::Login(LoginMessage::UsernameEntered(
        "1234".to_string(),
    )));
    assert_eq!(app.login_state.username, "1234".to_string());
}

#[test]
fn change_password_login() {
    let mut app = client_setup();
    let _ = app.update(Message::Login(LoginMessage::PasswordEntered(
        "135".to_string(),
    )));
    assert_eq!(app.login_state.password, "135".to_string());
}

#[test]
fn test_correct_login() {
    let mut app = client_setup();
    let _ = app.update(Message::Login(LoginMessage::PasswordEntered(
        "1234".to_string(),
    )));
    let _ = app.update(Message::Login(LoginMessage::UsernameEntered(
        "1235".to_string(),
    )));
    assert_eq!(
        app.login_state.try_login(),
        Ok(LoginRequest {
            username: "1235".to_string(),
            password: "1234".to_string(),
        })
    )
}
