use crate::client::backend::login_state::LoginStates;
use crate::client::backend::pop_up_manager::PopUpType;
use crate::client::gui::app::App;
use crate::client::gui::bb_tab::loading::view_loading_screen;
use crate::client::gui::bb_tab::login::view_login;
use crate::client::gui::bb_tab::settings::SettingsMessage;
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
use crate::client::gui::bb_widget::chart::ChartTypes;
use crate::client::gui::bb_widget::graph::{GraphMessage, MAX_AMOUNT_POINTS};
use crate::client::gui::bb_widget::pop_up::view_pop_up;
use crate::client::gui::bb_widget::social_elements::profile_tab_button;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::{bb_theme, size};
use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::client::server_communication::mascot_communicator::update_selected_mascot_on_server;
use crate::client::server_communication::request_data::{
    LoginServerRequestData, request_login_data,
};
use crate::client::server_communication::server_communicator::{
    SaveMascotError, SaveWorkoutError, save_mascot, valid_login,
};
use crate::client::server_communication::user_communicator::{
    add_foreign_user_as_friend_on_server, remove_foreign_user_as_friend_on_server,
};
use crate::common::login::RequestValidUserError;
use crate::common::mascot_mod::epic_mascot::EpicMascot;
use crate::common::mascot_mod::mascot::{Mascot, MascotRarity};
use crate::common::mascot_mod::rare_mascot::RareMascot;
use crate::common::user_mod::friend_request::FriendRequest;
use crate::common::user_mod::user::UserType;
use iced::widget::{Column, Space, Stack, container, row};
use iced::{Element, Task};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::image::Handle;
use iced_core::keyboard::Key;
use iced_core::window::{Position, Settings};
use iced_core::{Length, Size, Theme};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    Select(Tab),
    Activity(ActivityMessage),
    BuyMascot(MascotRarity),
    SaveMascot(Result<Mascot, SaveMascotError>),
    SelectMascot(Mascot),
    TryRegister,
    TryLogin,
    RequestValidUser(Result<String, RequestValidUserError>),
    RequestLoginData(Result<Arc<LoginServerRequestData>, ServerRequestError>), //Arc necessary to receive non-cloneable Vec<Exercise>
    UsernameEntered(String),
    PasswordEntered(String),
    SelectExercise(String),
    Graph(GraphMessage),
    ChangeShownChartType(ChartTypes),
    AddUserAsFriend(String),
    RemoveUserAsFriend(String),
    ViewProfile(UserType),
    ResetPopUp,
    Settings(SettingsMessage),
    UpdateInfoOnServerResult(Result<(), ServerRequestError>, String),
    ToggleGeneralExerciseInfo(u32),
    WorkoutCreation(WorkoutCreationMessage),
    SaveWorkout(Result<(), SaveWorkoutError>),
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Select(Tab::Exit) => iced::exit(),
            Message::Select(tab) => {
                self.activity_widget.update_data(
                    self.mascot_manager.selected_mascot,
                    self.user_manager
                        .user_info
                        .profile_stat_manager
                        .activity_data
                        .clone(),
                );

                if let Tab::CreateWorkout = tab {
                    self.exercise_manager.start_workout();
                }

                self.screen = tab;
                Task::none()
            }
            Message::BuyMascot(rarity) => {
                if match rarity {
                    MascotRarity::Rare => self.user_manager.user_info.coin_balance >= 50,
                    MascotRarity::Epic => self.user_manager.user_info.coin_balance >= 100,
                } {
                    self.loading = true;
                    let mut mascot_maybe: Option<Mascot> = None;
                    match rarity {
                        MascotRarity::Rare => {
                            match RareMascot::random_new_rare(&self.mascot_manager) {
                                Ok(mascot) => mascot_maybe = Some(mascot.into()),
                                Err(_err) => self.pop_up_manager.new_pop_up(
                                    PopUpType::Minor,
                                    "Failed to buy mascot!".to_string(),
                                    "All mascots of this rarity have already been purchased!"
                                        .to_string(),
                                ),
                            }
                        }
                        MascotRarity::Epic => {
                            match EpicMascot::random_new_epic(&self.mascot_manager) {
                                Ok(mascot) => mascot_maybe = Some(mascot.into()),
                                Err(_err) => self.pop_up_manager.new_pop_up(
                                    PopUpType::Minor,
                                    "Failed to buy mascot!".to_string(),
                                    "All mascots of this rarity have already been purchased!"
                                        .to_string(),
                                ),
                            }
                        }
                    };
                    if let Some(mascot) = mascot_maybe {
                        Task::perform(save_mascot(mascot), Message::SaveMascot)
                    } else {
                        Task::none()
                    }
                } else {
                    self.pop_up_manager.new_pop_up(
                        PopUpType::Minor,
                        "Funds lacking!".to_string(),
                        "You do not have enough money to buy a mascot of this type".to_string(),
                    );
                    Task::none()
                }
            }
            Message::SaveMascot(Ok(mascot)) => {
                self.loading = false;
                match mascot {
                    Mascot::Epic(_) => self.user_manager.user_info.coin_balance -= 100,
                    Mascot::Rare(_) => self.user_manager.user_info.coin_balance -= 50,
                }
                self.mascot_manager.add_mascot(mascot);
                Task::none()
            }
            Message::SaveMascot(Err(_err)) => {
                self.loading = false;
                self.pop_up_manager.new_pop_up(
                    PopUpType::Minor,
                    "Server error!".to_string(),
                    "Server is either offline or had an internal error!\nPlease start server or report bug".to_string(),
                );
                Task::none()
            }
            Message::SelectMascot(mascot) => {
                let active_mascot = &mut self.mascot_manager.selected_mascot;
                *active_mascot = mascot;
                self.activity_widget.update_active_mascot(*active_mascot);

                if let Some(jwt) = self.jsonwebtoken.clone() {
                    Task::perform(update_selected_mascot_on_server(jwt, mascot), |result| {
                        Message::UpdateInfoOnServerResult(result, "selected Mascot".to_string())
                    })
                } else {
                    println!("Log in to select a Mascot!");
                    Task::none()
                }
            }
            Message::Activity(activity_message) => self.activity_widget.update(activity_message),
            Message::TryRegister => {
                self.login_state.state = LoginStates::LoggedIn;
                Task::none()
            }
            Message::TryLogin => {
                match self.login_state.try_login() {
                    Err(err) => {
                        self.login_state.error_text = err.to_error_message();
                        Task::none()
                    }
                    //TODO check with server database
                    Ok(login_request) => {
                        self.loading = true; //TODO do we need this?
                        Task::perform(valid_login(login_request), Message::RequestValidUser)
                    }
                }
            }
            Message::RequestValidUser(Ok(jwt)) => {
                self.loading = false;
                self.jsonwebtoken = Some(jwt);
                self.login_state.state = LoginStates::FetchingLoginData;
                Task::perform(
                    request_login_data(self.jsonwebtoken.clone()),
                    Message::RequestLoginData,
                )
            }
            Message::RequestLoginData(Ok(data)) => {
                match Arc::try_unwrap(data) {
                    Ok(data) => {
                        self.update_app_on_login(data);
                    }
                    Err(_) => self.login_state.error_text = "Internal error: Arc".to_string(),
                }
                self.login_state.state = LoginStates::LoggedIn;
                Task::none()
            }
            Message::RequestLoginData(Err(_err)) => {
                self.login_state.error_text =
                    "Could not fetch login data from the server.".to_string();
                Task::none()
            }
            Message::RequestValidUser(Err(request_valid_error)) => {
                match request_valid_error {
                    RequestValidUserError::WrongPassword => {
                        self.login_state.password = "".to_string();
                        self.login_state.error_text = "Wrong password!".to_string();
                    }
                    RequestValidUserError::UserNotFound => {
                        self.login_state.username = "".to_string();
                        self.login_state.password = "".to_string();
                        self.login_state.error_text = "No user with that username!".to_string();
                    }
                    RequestValidUserError::ServerError => {
                        println!("Server had err during user login check")
                    }
                }
                self.loading = false;
                Task::none()
            }
            Message::UsernameEntered(new_username) => {
                let username = &mut self.login_state.username;
                *username = new_username;
                Task::none()
            }
            Message::PasswordEntered(new_password) => {
                let password = &mut self.login_state.password;
                *password = new_password;
                Task::none()
            }
            Message::SelectExercise(exercise) => {
                self.exercise_manager.update_selected_exercise(exercise);
                Task::none()
            }
            Message::Graph(graph_message) => {
                match graph_message {
                    GraphMessage::GraphCursorMoved(_point) => {}

                    GraphMessage::GraphKeyPressed(Key::Character(char)) => match char.as_str() {
                        "h" => self.graph_widget_state.invert_visible_points(),
                        "c" => self.graph_widget_state.invert_visible_cursor_information(),
                        _ => {}
                    },
                    GraphMessage::IncrementCounter => {
                        if self.graph_widget_state.get_counter() < MAX_AMOUNT_POINTS {
                            self.graph_widget_state.increment_counter();
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
                        if self.graph_widget_state.get_counter() > 1 {
                            self.graph_widget_state.decrement_counter();
                        }
                    }
                    GraphMessage::UpdateAnimatedSelection(event) => {
                        self.graph_widget_state.animation_progress.update(event);
                        self.graph_widget_state.update_graph();
                    }

                    GraphMessage::ToggleDots => self.graph_widget_state.invert_visible_points(),

                    GraphMessage::ToggleCursor => {
                        self.graph_widget_state.invert_visible_cursor_information()
                    }

                    GraphMessage::ToggleVerticalLines => {
                        self.graph_widget_state.invert_visible_vertical_lines()
                    }
                    _other_key_enums => {}
                };
                Task::none()
            }

            Message::ChangeShownChartType(chart_type) => {
                self.graph_widget_state.shown_chart_type = chart_type;
                Task::none()
            }

            Message::AddUserAsFriend(username) => {
                self.user_manager.add_user_as_friend(&username);
                if let Some(jwt) = self.jsonwebtoken.clone() {
                    Task::perform(
                        add_foreign_user_as_friend_on_server(jwt, FriendRequest { username }),
                        |result| {
                            Message::UpdateInfoOnServerResult(result, "added-Friend".to_string())
                        },
                    )
                } else {
                    println!("Log in to add a friend!");
                    Task::none()
                }
            }
            Message::RemoveUserAsFriend(username) => {
                self.user_manager.remove_user_as_friend(&username);
                if let Some(jwt) = self.jsonwebtoken.clone() {
                    Task::perform(
                        remove_foreign_user_as_friend_on_server(jwt, FriendRequest { username }),
                        |result| {
                            Message::UpdateInfoOnServerResult(result, "removed-Friend".to_string())
                        },
                    )
                } else {
                    println!("Log in to remove a friend!");
                    Task::none()
                }
            }
            Message::ViewProfile(user_type) => {
                match user_type {
                    UserType::Own => {
                        self.activity_widget.update_data(
                            self.user_manager.user_info.favorite_mascot,
                            self.user_manager
                                .user_info
                                .profile_stat_manager
                                .activity_data
                                .clone(),
                        );
                        self.user_manager.most_recently_viewed_user = UserType::Own
                    }
                    UserType::Other(username) => {
                        let opt_user = self.user_manager.get_user_by_username(&username);
                        if let Some(user) = opt_user {
                            self.activity_widget.update_data(
                                user.user_information.favorite_mascot,
                                user.user_information
                                    .profile_stat_manager
                                    .activity_data
                                    .clone(), //TODO maybe without clone possible?
                            );
                        }
                        self.user_manager.most_recently_viewed_user = UserType::Other(username);
                    }
                }
                self.screen = Tab::ViewProfile;
                Task::none()
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
            Message::WorkoutCreation(workout_creation_msg) => workout_creation_msg.update(self),
            Message::SaveWorkout(Err(err)) => {
                match err {
                    SaveWorkoutError::ServerError => self.pop_up_manager.new_pop_up(
                        PopUpType::Minor,
                        "Error while sending workout to server!".to_string(),
                        "Server offline or had internal error \nTry again later".to_string(),
                    ),
                }
                Task::none()
            }
            Message::SaveWorkout(Ok(())) => Task::none(),
        }
    }
    fn view(&self) -> Element<'_, Message> {
        if self.pop_up_manager.major_pop_up {
            return view_pop_up(self);
        }
        match self.login_state.state {
            // COULD BE INTEGRATED AS A TAB
            LoginStates::NotLoggedIn => return view_login(self),
            LoginStates::FetchingLoginData => return view_loading_screen(self),
            LoginStates::LoggedIn => (),
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
        let money_button: iced::widget::Button<'_, Message, Theme, iced::Renderer> =
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

        let tab_container = container(tab_buttons.spacing(INDENT))
            .style(bb_theme::container::create_container_style(
                ContainerStyle::Default,
                None,
                None,
            ))
            .height(Length::Fill)
            .width(310);

        let tab_window: Option<Element<Message>> = match self.screen {
            Tab::Home => Some(self.homescreen()),
            Tab::Workout => Some(self.workout_screen()),
            Tab::Social => Some(self.social_screen()),
            Tab::Mascot => Some(self.mascot_screen()),
            Tab::Settings => Some(self.settings_screen()),
            Tab::Exit => None,
            Tab::CreateWorkout => Some(self.workout_creation_screen()),
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
        let mut stack = Stack::new();
        let content = if let Some(tab_content) = tab_window {
            stack = stack.push(tab_content);
            if self.pop_up_manager.minor_pop_up {
                stack = stack.push(view_pop_up(self));
            }
            row![tab_container, stack]
        } else {
            row![tab_container]
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                text_color: None,
                background: Some(iced::Background::Color(color::BACKGROUND_COLOR)),
                ..Default::default()
            })
            .padding(20)
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
