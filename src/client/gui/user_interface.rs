use crate::client::backend::exercise_create::WorkoutCreate;
use crate::client::backend::login_state::LoginStates;
use crate::client::backend::pop_up_manager::PopUpType;
use crate::client::backend::widget_state::widget_state_manager::WidgetMessage;
pub use crate::client::gui::app::App;
use crate::client::gui::bb_tab::health::HealthMessage;
use crate::client::gui::bb_tab::login::{LoginMessage, view_login};
use crate::client::gui::bb_tab::mascot::MascotMessage;
use crate::client::gui::bb_tab::preset_creation::PresetCreationMessage;
use crate::client::gui::bb_tab::settings::SettingsMessage;
use crate::client::gui::bb_tab::social::SocialMessage;
use crate::client::gui::bb_tab::tab::{Tab, view_tab_button_bar, view_tab_content};
use crate::client::gui::bb_tab::workout_creation::WorkoutCreationMessage;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_widget::pop_up::view_pop_up;
use crate::client::gui::size;
use crate::client::server_communication::request_data::LoginServerRequestData;
use crate::client::server_communication::server_communicator::ServerRequestError;
use crate::common::exercise_mod::general_exercise::Id;
use iced::widget::{Row, Stack, container};
use iced::{Element, Task};
use iced_core::window::{Position, Settings};
use iced_core::{Length, Size, Theme};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    Select(Tab),

    Widget(WidgetMessage),

    Social(SocialMessage),
    ResetPopUp,
    HealthTab(HealthMessage),
    Settings(SettingsMessage),
    UpdateInfoOnServerResult(Result<(), ServerRequestError>, String),

    // WorkoutMessage (Combine)
    WorkoutCreation(WorkoutCreationMessage),
    SaveWorkout(Result<Id, ServerRequestError>, WorkoutCreate),

    // PresetMessage (Can stay)
    PresetCreation(PresetCreationMessage),

    // Login Messages
    Login(LoginMessage),
    RequestLoginData(Result<Arc<LoginServerRequestData>, ServerRequestError>), //Arc necessary to receive non-cloneable Vec<Exercise>
    Mascot(MascotMessage),
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Select(Tab::Exit) => iced::exit(),
            Message::Select(tab) => {
                self.widget_manager.activity_widget.update_data(
                    self.mascot_manager.selected_mascot,
                    self.user_manager
                        .user_info
                        .profile_stat_manager
                        .activity_data
                        .clone(),
                );

                if let Tab::CreateWorkout = tab {
                    self.exercise_manager.start_workout();
                } else if let Tab::CreatePreset = tab {
                    self.workout_preset_manager.start_preset_creation();
                }

                self.screen = tab;
                Task::none()
            }

            Message::Widget(widget_message) => WidgetMessage::update(widget_message, self),

            Message::HealthTab(health_message) => {
                HealthMessage::update_health_tab(health_message, self)
            }

            Message::ResetPopUp => {
                self.pop_up_manager.reset();
                Task::none()
            }
            Message::Settings(settings_msg) => settings_msg.update(self, self.jsonwebtoken.clone()),

            Message::UpdateInfoOnServerResult(res, info_type) => {
                match res {
                    Ok(_) => {
                        println!("Updated {info_type} info was successfully sent to the server!")
                    }
                    Err(err) => println!("{}", err.to_error_message()),
                }
                Task::none()
            }
            Message::Social(social_message) => social_message.update(self),
            Message::WorkoutCreation(workout_creation_msg) => workout_creation_msg.update(self),
            Message::SaveWorkout(Err(_), _) => {
                self.screen = Tab::Workout;
                self.pop_up_manager.new_pop_up(
                    PopUpType::Minor,
                    "Error while sending workout to server!".to_string(),
                    "Server offline or had internal error \nTry again later".to_string(),
                );
                Task::none()
            }
            Message::SaveWorkout(Ok(id), workout_create) => {
                self.exercise_manager.save_workout(
                    &workout_create,
                    id,
                    &mut self.user_manager.user_info,
                );
                self.screen = Tab::Workout;
                Task::none()
            }

            Message::PresetCreation(preset_creation_msg) => preset_creation_msg.update(self),
            Message::Login(login_msg) => login_msg.update(self),
            Message::RequestLoginData(Ok(data)) => {
                match Arc::try_unwrap(data) {
                    Ok(data) => {
                        self.update_app_on_login(data);
                    }
                    Err(_) => self.login_state.error_text = "Internal error: Arc".to_string(),
                }
                self.login_if_fetching_login_data_successful();
                Task::none()
            }
            Message::RequestLoginData(Err(_err)) => {
                *self = App::default();
                //TODO FIX MAJOR POPUP BACKGROUND
                self.pop_up_manager.new_pop_up(
                    PopUpType::Major,
                    "Error while fetching login data!".to_string(),
                    "There was an error while fetching the login data. \nTry again later!"
                        .to_string(),
                );
                Task::none()
            }
            Message::Mascot(mascot_message) => mascot_message.update(self),
        }
    }
    fn view(&self) -> Element<'_, Message> {
        if self.pop_up_manager.major_pop_up {
            return view_pop_up(self);
        }
        if self.login_state.state == LoginStates::NotLoggedIn && self.screen != Tab::Loading {
            return view_login(self);
        }
        if self.screen == Tab::Loading {
            return self.view_loading_screen();
        }

        let tab_button_container = view_tab_button_bar(
            &self.mascot_manager.selected_mascot,
            &self.screen,
            &self.user_manager.user_info,
        );

        let tab_content = view_tab_content(self);

        let mut content = Row::new().push(tab_button_container).spacing(20);

        if let Some(tab) = tab_content {
            if self.pop_up_manager.minor_pop_up {
                let tab_window_with_popup = Stack::new().push(tab).push(view_pop_up(self));

                content = content.push(tab_window_with_popup);
            } else {
                content = content.push(tab);
            }
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                text_color: None,
                background: Some(iced::Background::Color(color::BACKGROUND_COLOR)),
                ..Default::default()
            })
            .into()
    }
}

pub fn client_main() -> iced::Result {
    let default_size = Size::new(size::FRAME_WIDTH, size::FRAME_HEIGHT);
    let settings: Settings = Settings {
        size: default_size,
        maximized: false,
        fullscreen: false,
        position: Position::Default,
        min_size: Some(default_size),
        max_size: None,
        visible: true,
        resizable: true,

        closeable: true,
        minimizable: true,
        decorations: true,
        transparent: true,
        blur: false,
        level: Default::default(),
        icon: None,
        platform_specific: Default::default(),
        exit_on_close_request: true,
    };
    iced::application(App::default, App::update, App::view)
        .window(settings)
        .title("BuffBuddies")
        .font(include_bytes!(
            "../../../assets/Fira_Sans/FiraSans-ExtraBold.ttf"
        ))
        .run()
}
