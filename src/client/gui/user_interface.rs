use crate::client::backend::exercise_create::WorkoutCreate;
use crate::client::backend::login_state::LoginStates;
use crate::client::backend::pop_up_manager::PopUpType;
pub use crate::client::gui::app::App;
use crate::client::gui::bb_tab::health::HealthTabMessage;
use crate::client::gui::bb_tab::login::{LoginMessage, view_login};
use crate::client::gui::bb_tab::mascot::MascotMessage;
use crate::client::gui::bb_tab::preset_creation::PresetCreationMessage;
use crate::client::gui::bb_tab::settings::SettingsMessage;
use crate::client::gui::bb_tab::social::SocialMessage;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_tab::user::view_profile;
use crate::client::gui::bb_tab::workout_creation::WorkoutCreationMessage;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::container::ContainerStyle;
use crate::client::gui::bb_theme::custom_button::{
    ButtonStyle, TAB_BUTTON_HEIGHT, TAB_BUTTON_WIDTH, create_element_button, create_text_button,
};
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::bb_widget::activity_widget::activity::ActivityMessage;
use crate::client::gui::bb_widget::bmi_calculator::BMIMessage;
use crate::client::gui::bb_widget::chart::ChartTypes;
use crate::client::gui::bb_widget::circle_widget::CircleMessage;
use crate::client::gui::bb_widget::graph::{GraphMessage, MAX_AMOUNT_POINTS};
use crate::client::gui::bb_widget::pop_up::view_pop_up;
use crate::client::gui::bb_widget::progress_bar::ProgressBarMessage;
use crate::client::gui::bb_widget::social_elements::profile_tab_button;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::{bb_theme, size};
use crate::client::server_communication::request_data::LoginServerRequestData;
use crate::client::server_communication::server_communicator::ServerRequestError;
use crate::common::exercise_mod::general_exercise::Id;
use crate::common::user_mod::user::UserType;
use iced::widget::{Column, Row, Space, Stack, container, row};
use iced::{Element, Task};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::image::Handle;
use iced_core::keyboard::Key;
use iced_core::window::{Position, Settings};
use iced_core::{Length, Padding, Size, Theme};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    Select(Tab),

    // ChartMessage (Combine) maybe include in WidgetMessage?
    SelectExercise(String),
    Graph(GraphMessage),
    ChangeShownChartType(ChartTypes),

    // WidgetMessage? (Combine)
    Activity(ActivityMessage),
    Circle(CircleMessage),
    Bmi(BMIMessage),
    ProgressBar(ProgressBarMessage),
    HealthTab(HealthTabMessage),
    ToggleGeneralExerciseInfo(u32),

    Social(SocialMessage),
    ResetPopUp,
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
            Message::Activity(activity_message) => {
                self.widget_manager.activity_widget.update(activity_message)
            }
            Message::SelectExercise(exercise) => {
                self.exercise_manager.update_selected_exercise(exercise);
                Task::none()
            }
            Message::Graph(graph_message) => {
                match graph_message {
                    GraphMessage::GraphCursorMoved(_point) => {}

                    GraphMessage::GraphKeyPressed(Key::Character(char)) => match char.as_str() {
                        "h" => self
                            .widget_manager
                            .graph_widget_state
                            .invert_visible_points(),
                        "c" => self
                            .widget_manager
                            .graph_widget_state
                            .invert_visible_cursor_information(),
                        _ => {}
                    },
                    GraphMessage::IncrementCounter => {
                        if self.widget_manager.graph_widget_state.get_counter() < MAX_AMOUNT_POINTS
                        {
                            self.widget_manager.graph_widget_state.increment_counter();
                        } else {
                            self.pop_up_manager.new_pop_up(
                                PopUpType::Minor,
                                "Limit reached ".to_string(),
                                format!(
                                    "The graph can’t display more than {MAX_AMOUNT_POINTS} points"
                                ),
                            );
                        }
                    }
                    GraphMessage::DecrementCounter => {
                        if self.widget_manager.graph_widget_state.get_counter() > 1 {
                            self.widget_manager.graph_widget_state.decrement_counter();
                        }
                    }
                    GraphMessage::UpdateAnimatedSelection(event) => {
                        self.widget_manager
                            .graph_widget_state
                            .animation_progress
                            .update(event);
                        self.widget_manager.graph_widget_state.update_graph();
                    }

                    GraphMessage::ToggleDots => self
                        .widget_manager
                        .graph_widget_state
                        .invert_visible_points(),

                    GraphMessage::ToggleCursor => self
                        .widget_manager
                        .graph_widget_state
                        .invert_visible_cursor_information(),

                    GraphMessage::ToggleVerticalLines => self
                        .widget_manager
                        .graph_widget_state
                        .invert_visible_vertical_lines(),
                    _other_key_enums => {}
                };
                Task::none()
            }

            Message::ChangeShownChartType(chart_type) => {
                self.widget_manager.graph_widget_state.shown_chart_type = chart_type;
                Task::none()
            }
            Message::Circle(circle_message) => match circle_message {
                CircleMessage::UpdateCircleAnimation(event) => {
                    self.widget_manager
                        .circle_widget_state
                        .animation_progress
                        .update(event);
                    self.widget_manager.circle_widget_state.update_circle();
                    Task::none()
                }
            },

            Message::Bmi(bmi_message) => BMIMessage::update_bmi_message(bmi_message, self),
            Message::ProgressBar(progress_bar_message) => {
                ProgressBarMessage::update_progress_bar_message(progress_bar_message, self)
            }
            Message::HealthTab(health_message) => {
                HealthTabMessage::update_health_tab(health_message, self)
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
            Message::ToggleGeneralExerciseInfo(id) => {
                let extended_set = &mut self.exercise_manager.extended_general_exercise_infos;
                if extended_set.contains(&id) {
                    extended_set.remove(&id);
                } else {
                    extended_set.insert(id);
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

        let mut tab_buttons: Column<Message> =
            Column::new().padding(INDENT).align_x(Horizontal::Center);
        tab_buttons = tab_buttons.push(profile_tab_button(self));
        for tab in Tab::get_tab_button_categories() {
            tab_buttons = tab_buttons.push(
                create_text_button(
                    &self.mascot_manager.selected_mascot,
                    tab.to_string(),
                    if self.screen == tab {
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
        let money_button: iced_anim::widget::Button<'_, Message, Theme, iced::Renderer> =
            create_element_button(
                &self.mascot_manager.selected_mascot,
                row![
                    iced::widget::image(Handle::from_path("assets/images/coin.png"))
                        .width(25)
                        .height(25),
                    Space::with_width(Length::Fill),
                    format_button_text(iced::widget::text(
                        self.user_manager.user_info.coin_balance
                    ))
                ]
                .align_y(Vertical::Center)
                .into(),
                ButtonStyle::InactiveTab,
                None,
            )
            .on_press(Message::Select(Tab::Mascot))
            .width(Length::Fill)
            .height(Length::Shrink);

        let lower_tab_container_buttons =
            row![Space::with_width(Length::Fill), money_button].width(310);

        tab_buttons = tab_buttons.push(Space::with_height(Length::Fill));
        tab_buttons = tab_buttons.push(lower_tab_container_buttons);

        let mut tab_container = container(tab_buttons.spacing(INDENT))
            .style(bb_theme::container::create_container_style(
                ContainerStyle::Default,
                None,
                None,
            ))
            .height(Length::Fill)
            .width(310);

        tab_container = container(tab_container).padding(Padding {
            right: 0.0,
            ..15.into()
        });

        let tab_window: Option<Element<Message>> = match self.screen {
            Tab::Loading => return self.view_loading_screen(),
            Tab::Home => Some(self.homescreen()),
            Tab::Workout => Some(self.workout_screen()),
            Tab::Health => Some(self.health_screen()),
            Tab::Social => Some(self.social_screen()),
            Tab::Mascot => Some(self.mascot_screen()),
            Tab::Settings => Some(self.settings_screen()),
            Tab::Exit => None,
            Tab::CreateWorkout => Some(self.workout_creation_screen()),
            Tab::CreatePreset => Some(self.preset_creation_screen()),
            Tab::ViewProfile => {
                let user_type = &self.user_manager.most_recently_viewed_user;

                match user_type {
                    UserType::Own => Some(view_profile(
                        self,
                        &self.user_manager.user_info,
                        &self.mascot_manager.owned_mascots,
                        false,
                    )),
                    UserType::Other(username) => {
                        let viewed_profile = self.user_manager.get_user_by_username(username);

                        viewed_profile.map(|profile| {
                            view_profile(
                                self,
                                &profile.user_information,
                                &profile.owned_mascots,
                                profile.friends_with_active_user,
                            )
                        })
                    }
                }
            }
        };

        let mut content = Row::new().push(tab_container).spacing(20);

        if let Some(tab_content) = tab_window {
            if self.pop_up_manager.minor_pop_up {
                let mut tab_window_with_popup = Stack::new();
                tab_window_with_popup = tab_window_with_popup.push(tab_content);
                tab_window_with_popup = tab_window_with_popup.push(view_pop_up(self));

                content = content.push(tab_window_with_popup);
            } else {
                content = content.push(tab_content);
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
    iced::application("BuffBuddies", App::update, App::view)
        .window(settings)
        .font(include_bytes!(
            "../../../assets/Fira_Sans/FiraSans-ExtraBold.ttf"
        ))
        .run_with(|| (App::default(), Task::none()))
}
