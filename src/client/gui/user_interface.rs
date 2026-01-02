use crate::client::backend::login_state::LoginStateError;
use crate::client::gui::app::App;
use crate::client::gui::bb_tab::login::view_login;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::container::ContainerStyle;
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_text_button, TAB_BUTTON_WIDTH, TAB_BUTTON_HEIGHT};
use crate::client::gui::bb_widget::activity_widget::activity::ActivityMessage;
use crate::client::gui::{bb_theme, size};
use crate::client::server_communication::server_communicator::{
    RequestValidUserError, valid_login,
};
use iced::widget::{Column, container, row};
use iced::{Element, Task};
use iced_core::Length::Fill;
use iced_core::window::{Position, Settings};
use iced_core::{Size, Theme};
use strum::IntoEnumIterator;
use crate::client::backend::user_mod::user::User;
use crate::client::gui::bb_tab::user;

#[derive(Default)]
pub struct UserInterface {
    pub app: App,
}

#[derive(Debug, Clone)]
pub enum Message {
    Select(Tab),
    Activity(ActivityMessage),
    BuyMascot(),
    TryRegister,
    TryLogin,
    RequestValidUser(Result<(), RequestValidUserError>),
    UsernameEntered(String),
    PasswordEntered(String),
    SelectExercise(String),
    AddUserAsFriend(String),
    ViewProfile(String)
}

impl UserInterface {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Select(Tab::Exit) => iced::exit(),
            Message::Select(tab) => {
                self.app.screen = tab;
                Task::none()
            }
            Message::BuyMascot() => {
                self.app.screen = Tab::Settings;
                Task::none()
            }
            Message::Activity(activity_message) => {
                self.app.activity_widget.update(activity_message)
            }
            Message::TryRegister => {
                self.app.login_state.logged_in = true;
                Task::none()
            }
            Message::TryLogin => {
                match self.app.login_state.try_login() {
                    Err(err) => {
                        let error_text = &mut self.app.login_state.error_text;
                        match err {
                            LoginStateError::PasswordEmpty => {
                                *error_text = "Password can't be empty!".to_string()
                            }
                            LoginStateError::UsernameEmpty => {
                                *error_text = "Username can't be empty!".to_string()
                            }
                        }
                        Task::none()
                    }
                    //TODO check with server database
                    Ok(login_request) => {
                        self.app.loading = true;
                        Task::perform(
                            async { valid_login(login_request) },
                            Message::RequestValidUser,
                        )
                    }
                }
            }
            Message::RequestValidUser(Ok(_)) => {
                self.app.loading = false;
                self.app.login_state.logged_in = true;
                Task::none()
            }
            Message::RequestValidUser(Err(request_valid_error)) => {
                match request_valid_error {
                    RequestValidUserError::WrongPassword => {
                        self.app.login_state.password = "".to_string();
                        self.app.login_state.error_text = "Wrong password!".to_string();
                    }
                    RequestValidUserError::UserNotFound => {
                        self.app.login_state.username = "".to_string();
                        self.app.login_state.password = "".to_string();
                        self.app.login_state.error_text = "No user with that username!".to_string();
                    }
                    RequestValidUserError::ServerError => {
                        println!("Server had err during user login check")
                    }
                }
                self.app.loading = false;
                Task::none()
            }
            Message::UsernameEntered(new_username) => {
                let username = &mut self.app.login_state.username;
                *username = new_username;
                Task::none()
            }
            Message::PasswordEntered(new_password) => {
                let password = &mut self.app.login_state.password;
                *password = new_password;
                Task::none()
            }
            Message::SelectExercise(exercise) => {
                self.app.exercise_manager.update_selected_exercise(exercise);
                Task::none()
            }
            Message::AddUserAsFriend(username) => {
                self.app.user_manager.add_user_as_friend(&*username);
                Task::none()
            }
            Message::ViewProfile(username) => {
                //TODO Handle
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        if !self.app.login_state.logged_in {
            view_login(&self.app)
        } else {
            let mut tab_bar: Column<Message> = Column::new();
            for tab in Tab::iter() {
                tab_bar = tab_bar.push(
                    create_text_button(
                        self.app.mascot_manager.selected_mascot,
                        tab.to_string(),
                        if self.app.screen == tab {
                            ButtonStyle::ActiveTab
                        } else {
                            ButtonStyle::InactiveTab
                        },
                        None,
                    )
                    .width(TAB_BUTTON_WIDTH)
                    .height(TAB_BUTTON_HEIGHT)
                    .on_press(Message::Select(tab)),
                );
            }
            let tab_container = container(tab_bar.spacing(10).padding(30))
                .padding(10)
                .style(bb_theme::container::create_style_container(
                    ContainerStyle::Default,
                    None,
                    None,
                ))
                .height(Fill);

            let tab_window: Option<Element<Message>> = match self.app.screen {
                Tab::Home => Some(self.homescreen()),
                Tab::Workout => Some(self.workout_screen()),
                Tab::Social => Some(self.social_screen()),
                Tab::Mascot => Some(self.mascot_screen()),
                Tab::Settings => Some(user::view_profile(&User::default())),
                Tab::Exit => None,
            };

            let content = if let Some(tab_content) = tab_window {
                row![tab_container, tab_content]
            } else {
                row![tab_container]
            };

            container(content)
                .width(Fill)
                .height(Fill)
                .style(|_theme: &Theme| container::Style {
                    text_color: None,
                    background: Some(iced::Background::Color(color::BACKGROUND_COLOR)),
                    border: Default::default(),
                    shadow: Default::default(),
                }).padding(20)
                .into()
        }
    }
}

pub fn client_main() -> iced::Result {
    let default_size = Size::new(size::FRAME_WIDTH, size::FRAME_HEIGHT);
    let settings: Settings = Settings {
        size: default_size,
        position: Position::Default,
        min_size: Some(default_size),
        max_size: None,
        visible: true,
        resizable: true,

        decorations: true,
        transparent: true,
        level: Default::default(),
        icon: None,
        platform_specific: Default::default(),
        exit_on_close_request: true,
    };
    iced::application("BuffBuddies", UserInterface::update, UserInterface::view)
        .window(settings)
        .run_with(|| (UserInterface::default(), Task::none()))
}
