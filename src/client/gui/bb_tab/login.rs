use crate::client::backend::login_state::LoginStates;
use crate::client::backend::pop_up_manager::PopUpType;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::{BACKGROUND_COLOR, ERROR_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::combo_box::create_text_input_style;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_button_text};
use crate::client::gui::user_interface::Message;
use crate::client::server_communication::request_data::request_login_data;
use crate::client::server_communication::user_communicator::{valid_login, valid_register};
use crate::common::login::{RequestValidRegisterError, RequestValidUserError};
use iced::widget::{Column, Space, container, text, text_input};
use iced::{Element, Task};
use iced_core::Length::Fill;
use iced_core::Theme;
use crate::client::gui::bb_tab::tab::Tab;

const MAX_USERNAME_LENGTH: usize = 15;
const MAX_PASSWORD_LENGTH: usize = 100;

#[derive(Clone, Debug)]
pub enum LoginMessage {
    TryRegister,
    TryLogin,
    RequestValidUser(Result<String, RequestValidUserError>),
    RequestValidRegister(Result<String, RequestValidRegisterError>),
    UsernameEntered(String),
    PasswordEntered(String),
}

impl LoginMessage {
    pub fn update(&self, app: &mut App) -> Task<Message> {
        match self {
            LoginMessage::TryRegister => match app.login_state.try_login() {
                Err(err) => {
                    app.login_state.error_text = err.to_error_message();
                    Task::none()
                }
                Ok(login_request) => {
                    Task::perform(valid_register(login_request), |result| -> Message {
                        Message::Login(LoginMessage::RequestValidRegister(result))
                    })
                }
            },
            LoginMessage::TryLogin => match app.login_state.try_login() {
                Err(err) => {
                    app.login_state.error_text = err.to_error_message();
                    Task::none()
                }
                Ok(login_request) => {
                    Task::perform(valid_login(login_request), |result| -> Message {
                        Message::Login(LoginMessage::RequestValidUser(result))
                    })
                }
            },
            LoginMessage::RequestValidUser(Ok(jwt)) => {
                app.jsonwebtoken = Some(jwt.clone());
                app.screen = Tab::Loading;
                Task::perform(
                    request_login_data(app.jsonwebtoken.clone()),
                    Message::RequestLoginData,
                )
            }
            LoginMessage::RequestValidRegister(Ok(jwt)) => {
                app.jsonwebtoken = Some(jwt.clone());
                app.screen = Tab::Loading;
                Task::perform(
                    request_login_data(app.jsonwebtoken.clone()),
                    Message::RequestLoginData,
                )
            }
            LoginMessage::RequestValidRegister(Err(err)) => {
                match err {
                    RequestValidRegisterError::ServerError => app.pop_up_manager.new_pop_up(
                        PopUpType::Major,
                        "Server had an Error!".to_string(),
                        "Server was either offline or had an internal error".to_string(),
                    ),
                    RequestValidRegisterError::UserAlreadyExists => {
                        app.login_state.error_text = "Username already exists!".to_string()
                    }
                }
                Task::none()
            }
            LoginMessage::RequestValidUser(Err(request_valid_error)) => {
                match request_valid_error {
                    RequestValidUserError::WrongPassword => {
                        app.login_state.password = "".to_string();
                        app.login_state.error_text = "Wrong password!".to_string();
                    }
                    RequestValidUserError::UserNotFound => {
                        app.login_state.username = "".to_string();
                        app.login_state.password = "".to_string();
                        app.login_state.error_text = "No user with that username!".to_string();
                    }
                    RequestValidUserError::ServerError => {
                        app.pop_up_manager.new_pop_up(
                            PopUpType::Major,
                            "Server had an Error!".to_string(),
                            "Server was either offline or had an internal error".to_string(),
                        );
                    }
                }
                Task::none()
            }
            LoginMessage::UsernameEntered(new_username) => {
                if new_username.clone().len() <= MAX_USERNAME_LENGTH {
                    app.login_state.username = new_username.clone();
                }
                Task::none()
            }
            LoginMessage::PasswordEntered(new_password) => {
                if new_password.clone().len() <= MAX_PASSWORD_LENGTH {
                    app.login_state.password = new_password.clone();
                }
                Task::none()
            }
        }
    }
}

pub fn view_login(app: &App) -> Element<'_, Message> {
    let login_text: Element<Message> = text("LOGIN")
        .color(TEXT_COLOR)
        .font(FIRA_SANS_EXTRABOLD)
        .size(30)
        .width(Fill)
        .center()
        .into();

    let error_text: Element<Message> = text(&app.login_state.error_text)
        .font(FIRA_SANS_EXTRABOLD)
        .width(Fill)
        .center()
        .color(ERROR_COLOR)
        .into();

    let username_field: Element<Message> =
        text_input("Enter username...", &app.login_state.username)
            .style(create_text_input_style(
                &app.mascot_manager.selected_mascot,
                BACKGROUND_COLOR,
            ))
            .font(FIRA_SANS_EXTRABOLD)
            .on_input(|new_name| -> Message {
                Message::Login(LoginMessage::UsernameEntered(new_name))
            })
            .into();

    let password_field: Element<Message> =
        text_input("Enter password...", &app.login_state.password)
            .style(create_text_input_style(
                &app.mascot_manager.selected_mascot,
                BACKGROUND_COLOR,
            ))
            .font(FIRA_SANS_EXTRABOLD)
            .on_input(|new_password| -> Message {
                Message::Login(LoginMessage::PasswordEntered(new_password))
            })
            .on_submit(Message::Login(LoginMessage::TryLogin))
            .secure(true)
            .into();

    let login_button_text: Element<Message> = format_button_text(text("Login")).width(Fill).into();
    let login_button = create_element_button(
        &app.mascot_manager.selected_mascot,
        login_button_text,
        ButtonStyle::Active,
        None,
    )
    .on_press(Message::Login(LoginMessage::TryLogin))
    .width(Fill)
    .height(40);

    let register_button_text: Element<Message> =
        format_button_text(text("Register")).width(Fill).into();
    let register_button = create_element_button(
        &app.mascot_manager.selected_mascot,
        register_button_text,
        ButtonStyle::Active,
        None,
    )
    .on_press(Message::Login(LoginMessage::TryRegister))
    .width(Fill)
    .height(40);

    let login_elements = Column::new()
        .push(login_text)
        .push(error_text)
        .push(username_field)
        .push(password_field)
        .push(Space::with_height(0))
        .push(login_button)
        .push(register_button)
        .width(Fill)
        .height(Fill)
        .spacing(20);

    let login_container = container(login_elements)
        .padding(20)
        .max_width(400)
        .max_height(420)
        .style(create_container_style(ContainerStyle::Default, None, None))
        .width(Fill)
        .height(Fill);

    container(login_container)
        .width(Fill)
        .height(Fill)
        .style(|_theme: &Theme| container::Style {
            text_color: None,
            background: Some(iced::Background::Color(BACKGROUND_COLOR)),
            border: Default::default(),
            shadow: Default::default(),
        })
        .center(Fill)
        .into()
}
