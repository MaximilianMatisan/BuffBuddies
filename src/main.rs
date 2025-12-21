use std::io;
use crate::client::gui::app::App;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_theme::custom_button::{create_text_button, ButtonStyle};
use iced::widget::{container, row, Column};
use iced::{Element, Task, Theme};
use crate::client::gui::{bb_theme, size};
use iced_core::window::{Position, Settings};
use iced_core::Length::Fill;
use iced_core::{Size};
use strum::IntoEnumIterator;
use crate::client::backend::login_state::LoginStateError;
use crate::client::gui::bb_tab::login::view_login;
use crate::client::gui::bb_theme::container::ContainerStyle;
use crate::client::gui::bb_widget::activity::activity::{ActivityMessage};
use crate::server::server_main::server_main;
use crate::client::server_communicator::server_communicator::{valid_login, RequestValidUserError};
use crate::client::gui::bb_theme::color;

mod client;
mod server;

#[derive(Default)]
struct UserInterface {
    app: App,
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
}

impl UserInterface {
    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::Select(Tab::Exit) => iced::exit(),
            Message::Select(tab) => {
                self.app.screen = tab;
                Task::none()
            },
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
                return match self.app.login_state.try_login() {
                    Err(err) => {
                        let error_text = &mut self.app.login_state.error_text;
                        match err {
                            LoginStateError::PasswordEmpty => *error_text = "Password can't be empty!".to_string(),
                            LoginStateError::UsernameEmpty => *error_text = "Username can't be empty!".to_string(),
                        }
                        Task::none()
                    },
                    //TODO check with server database
                    Ok(login_request) => {
                        self.app.loading = true;
                        Task::perform(
                            async {
                                valid_login(login_request)
                            },
                            Message::RequestValidUser,
                        )
                    },
                };
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
                let mut username = &mut self.app.login_state.username;
                *username = new_username;
                Task::none()
            }
            Message::PasswordEntered(new_password) => {
                let mut password = &mut self.app.login_state.password;
                *password = new_password;
                Task::none()
            }
            Message::SelectExercise(exercise) => {
                self.app.exercise_manager.update_selected_exercise(exercise);
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
                    create_text_button(self.app.active_mascot.clone(),
                                       tab.to_string(),
                                       if self.app.screen == tab
                                       { ButtonStyle::ActiveTab } else { ButtonStyle::InactiveTab },
                                       None)
                        .on_press(Message::Select(tab))
                );
            }
            let tab_container = container(tab_bar.spacing(10).padding(30))
                .padding(10)
                .style(bb_theme::container::create_style_container(ContainerStyle::Default, None))
                .height(Fill);

            let tab_window: Option<Element<Message>> =
                match self.app.screen {
                    Tab::Home => Some(self.homescreen()),
                    Tab::Workout => Some(self.workout_screen()),
                    Tab::Social => Some(self.social_screen()),
                    Tab::Mascot => Some(self.mascot_screen()),
                    Tab::Settings => Some(self.settings_screen()),
                    Tab::Exit => None
                };

            if let Some(tab_window_real) = tab_window {
                container(row![tab_container, tab_window_real])
                    .width(size::FRAME_WIDTH)
                    .height(size::FRAME_HEIGHT)
                    .style(|_theme: &Theme| container::Style {
                        text_color: None,
                        background: Some(iced::Background::Color(color::BACKGROUND_COLOR)),
                        border: Default::default(),
                        shadow: Default::default(),
                    }).padding(20)
                    .into()
            } else {
                container(row![tab_container])
                    .width(size::FRAME_WIDTH)
                    .height(size::FRAME_HEIGHT)
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
}

enum LaunchType {
    Server,
    Client,
}

fn input_launch_type() -> LaunchType {
    let mut input = String::new();
    println!("Please type what you want to launch (Server/Client): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let formated_input = input.trim().to_lowercase();
    if formated_input == "server" {
        return LaunchType::Server;
    } else if formated_input == "client" {
        return LaunchType::Client;
    } else {
        input_launch_type()
    }
}

#[tokio::main]
pub async fn main() -> iced::Result {

    match input_launch_type() {
        LaunchType::Client => {
            client_main()
        },
        LaunchType::Server => {
            let server = server_main();
            server.await;
            Ok(())
        },
    }

    
}

fn client_main() -> iced::Result {
    let settings: Settings = Settings {
        size: Size::new(size::FRAME_WIDTH, size::FRAME_HEIGHT),
        position: Position::Default,
        min_size: None,
        max_size: None,
        visible: true,
        resizable: false,

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