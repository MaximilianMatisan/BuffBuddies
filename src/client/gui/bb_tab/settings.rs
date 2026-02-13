use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::combo_box::{create_menu_style, create_text_input_style};
use crate::client::gui::bb_theme::container::{
    ContainerStyle, DEFAULT_TEXT_CONTAINER_PADDING, create_container_style,
};
use crate::client::gui::bb_theme::custom_button::{
    ButtonStyle, create_element_button, create_text_button,
};
use crate::client::gui::bb_theme::text_format::{
    FIRA_SANS_EXTRABOLD, cm_to_string, format_description_text, kg_to_string,
};
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::bb_widget::widget_utils::{
    descriptor_space_fill_element_row, descriptor_space_fill_text_row,
};
use crate::client::gui::size;
use crate::client::gui::user_interface::Message;
use crate::client::server_communication::user_communicator::update_user_info_on_server;
use crate::common::exercise_mod::weight::Kg;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::user_mod::user::{
    Gender, MAX_DESCRIPTION_CHARACTERS, UserInformation, UserInformationStrings,
};
use iced::widget::{
    Button, Column, Container, Row, Space, TextInput, combo_box, container, image, text, text_input,
};
use iced::{Element, Task};
use iced_core::alignment::Vertical;
use iced_core::image::Handle;
use iced_core::{Length, Padding};

const SETTINGS_TEXT_INPUT_WIDTH: f32 = 250.0;
impl App {
    pub fn settings_screen(&self) -> Element<Message> {
        let user_info_container = settings_user_info_preview(self).map(Message::Settings);
        let log_out_button =
            log_out_button(&self.mascot_manager.selected_mascot).map(Message::Settings);

        let content = Column::new()
            .push(user_info_container)
            .push(log_out_button)
            .spacing(INDENT);

        let padded_content: Element<Message> = Row::new()
            .push(Space::with_width(Length::FillPortion(1)))
            .push(content)
            .push(Space::with_width(Length::FillPortion(1)))
            .into();

        padded_content
    }
}
fn settings_user_info_preview(app: &App) -> Element<SettingsMessage> {
    let user_info = &app.user_manager.user_info;

    let profile_picture = image(Handle::from_path(&user_info.profile_picture_handle))
        .width(size::LARGE_PROFILE_PICTURE_DIMENSION)
        .height(size::LARGE_PROFILE_PICTURE_DIMENSION);

    let username_and_data_column = if app.user_manager.pending_user_info_changes.is_none() {
        preview_user_info_column(app)
    } else {
        edit_user_info_column(app)
    };

    let contents = Row::new()
        .push(Space::with_width(Length::FillPortion(1)))
        .push(profile_picture)
        .push(Space::with_width(Length::FillPortion(1)))
        .push(username_and_data_column)
        .push(Space::with_width(Length::FillPortion(1)));

    let user_info_container = container(contents)
        .style(create_container_style(ContainerStyle::Default, None, None))
        .height(Length::Shrink)
        .width(Length::FillPortion(10))
        .padding(Padding {
            top: LARGE_INDENT,
            bottom: LARGE_INDENT,
            ..Default::default()
        });

    user_info_container.into()
}
fn preview_user_info_column(app: &App) -> Column<SettingsMessage> {
    let user_info = &app.user_manager.user_info;
    let username = text(&user_info.username)
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR)
        .size(40);
    let edit_profile_button: Button<SettingsMessage> = create_element_button(
        &app.mascot_manager.selected_mascot,
        image(Handle::from_path("assets/images/edit.png")).into(),
        ButtonStyle::InactiveTransparent,
        None,
    )
    .on_press(SettingsMessage::StartEditingProfile);

    let username_and_edit_button: Row<SettingsMessage> = Row::new()
        .push(username)
        .push(edit_profile_button)
        .spacing(INDENT);

    let description_text = if user_info.description.is_empty() {
        format_description_text(text("Tell something about you!"))
    } else {
        text(&user_info.description)
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR)
    };
    let description_text_container: Container<SettingsMessage> = container(description_text)
        .style(create_container_style(
            ContainerStyle::Background,
            None,
            None,
        ))
        .width(SETTINGS_TEXT_INPUT_WIDTH)
        .padding(DEFAULT_TEXT_CONTAINER_PADDING); //[top/bottom, left/right]

    let description = Row::new()
        .push(format_description_text(text("Description:")))
        .push(Space::with_width(Length::Fill))
        .push(description_text_container);

    let user_data_column = Column::new()
        .push(descriptor_space_fill_text_row(
            "Favorite Mascot:",
            app.user_manager.user_info.favorite_mascot.to_string(),
        ))
        .push(descriptor_space_fill_text_row(
            "Gender:",
            user_info.gender.to_string(),
        ))
        .push(descriptor_space_fill_text_row(
            "Weight:",
            kg_to_string(user_info.weight),
        ))
        .push(descriptor_space_fill_text_row(
            "Height:",
            cm_to_string(user_info.height),
        ))
        .push(descriptor_space_fill_text_row(
            "Weekly workout goal:",
            user_info.weekly_workout_goal.to_string(),
        ))
        .push(description)
        .width(Length::FillPortion(15));

    let username_and_data_column: Column<SettingsMessage> = Column::new()
        .push(username_and_edit_button)
        .push(Space::with_height(INDENT))
        .push(user_data_column);

    username_and_data_column
}
fn edit_user_info_column(app: &App) -> Column<SettingsMessage> {
    let pending_info: &UserInformation;
    //These strings are necessary for proper text_input functionality
    let pending_info_strings: &UserInformationStrings;

    if let Some((user_info, user_info_strings)) = &app.user_manager.pending_user_info_changes {
        pending_info = user_info;
        pending_info_strings = user_info_strings;
    } else {
        return Column::new();
    };

    //Username should currently not be changeable
    let username = text(&pending_info.username)
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR)
        .size(40);

    let gender_combo_box = combo_box(
        &app.user_manager.gender_combo_box_state,
        "Select gender...",
        Some(&pending_info.gender),
        SettingsMessage::SelectGender,
    )
    .font(FIRA_SANS_EXTRABOLD)
    .width(SETTINGS_TEXT_INPUT_WIDTH)
    .input_style(create_text_input_style(&app.mascot_manager.selected_mascot))
    .menu_style(create_menu_style(&app.mascot_manager.selected_mascot));

    let height_text_input = text_input("Enter your height in cm", &pending_info_strings.height)
        .on_input(SettingsMessage::EditHeight);

    let weight_text_input = text_input("Enter your weight in kg", &pending_info_strings.weight)
        .on_input(SettingsMessage::EditWeight);

    let weekly_workout_goal_text_input = text_input(
        "Enter your weekly workout goal",
        &pending_info_strings.weekly_workout_goal,
    )
    .on_input(SettingsMessage::EditWeeklyWorkoutGoal);

    let mascot_combo_box: Element<SettingsMessage> = combo_box(
        &app.mascot_manager.owned_mascots_state,
        "Select mascot...",
        Some(&pending_info.favorite_mascot),
        SettingsMessage::EditMascot,
    )
    .font(FIRA_SANS_EXTRABOLD)
    .width(SETTINGS_TEXT_INPUT_WIDTH)
    .input_style(create_text_input_style(&app.mascot_manager.selected_mascot))
    .menu_style(create_menu_style(&app.mascot_manager.selected_mascot))
    .into();

    //Didn't use pending_info_strings here as description is already a string in pending_info
    let description_text_input = text_input("Tell something about you!", &pending_info.description)
        .on_input(SettingsMessage::EditDescription);

    let text_input_data_fields: [(&str, TextInput<SettingsMessage>); 4] = [
        ("Weight:", weight_text_input),
        ("Height:", height_text_input),
        ("Weekly workout goal:", weekly_workout_goal_text_input),
        ("Description:", description_text_input),
    ];

    let mut user_data_column = Column::new()
        .spacing(3)
        .push(descriptor_space_fill_element_row(
            "Favorite Mascot:",
            mascot_combo_box,
        ))
        .push(descriptor_space_fill_element_row(
            "Gender:",
            gender_combo_box.into(),
        ));

    for (description_text, mut text_input) in text_input_data_fields {
        text_input = text_input
            .style(create_text_input_style(&app.mascot_manager.selected_mascot))
            .font(FIRA_SANS_EXTRABOLD)
            .width(SETTINGS_TEXT_INPUT_WIDTH);
        user_data_column = user_data_column.push(descriptor_space_fill_element_row(
            description_text,
            text_input.into(),
        ));
    }

    let save_changes_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        "Save changes".to_string(),
        ButtonStyle::Active,
        None,
    )
    .width(Length::Fill)
    .on_press(SettingsMessage::SavePendingUserInfoChanges);

    let discard_changes_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        "Discard changes".to_string(),
        ButtonStyle::InactiveTab,
        None,
    )
    .width(Length::Fill)
    .on_press(SettingsMessage::DiscardPendingUserInfoChanges);

    let button_row = Row::new()
        .push(save_changes_button)
        .push(discard_changes_button)
        .spacing(INDENT);

    let username_and_data_column = Column::new()
        .push(username)
        .push(user_data_column)
        .push(button_row)
        .spacing(INDENT)
        .width(Length::FillPortion(15));

    username_and_data_column
}
fn log_out_button(active_mascot: &Mascot) -> Element<SettingsMessage> {
    let log_out_button_text = text("Log out")
        .font(FIRA_SANS_EXTRABOLD)
        .color(color::ERROR_COLOR);

    let row = Row::new()
        .push(Space::with_width(INDENT))
        .push(image(Handle::from_path("assets/images/log-out.png")).height(20))
        .push(Space::with_width(LARGE_INDENT))
        .push(log_out_button_text)
        .align_y(Vertical::Center)
        .padding(5);

    create_element_button(active_mascot, row.into(), ButtonStyle::InactiveTab, None)
        .on_press(SettingsMessage::LogOut)
        .width(Length::Fill)
        .into()
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    StartEditingProfile,
    SelectGender(Gender),
    EditHeight(String),
    EditWeight(String),
    EditWeeklyWorkoutGoal(String),
    EditDescription(String),
    EditMascot(Mascot),
    SavePendingUserInfoChanges,
    DiscardPendingUserInfoChanges,
    LogOut,
}
impl SettingsMessage {
    pub fn update(self, app: &mut App, opt_jwt: Option<String>) -> Task<Message> {
        let existing_user_info = &app.user_manager.user_info;
        let pending_user_info_changes = &mut app.user_manager.pending_user_info_changes;
        match self {
            SettingsMessage::StartEditingProfile => {
                app.user_manager.pending_user_info_changes = Some((
                    app.user_manager.user_info.clone(),
                    UserInformationStrings::new(
                        existing_user_info.weight.to_string(),
                        existing_user_info.height.to_string(),
                        existing_user_info.weekly_workout_goal.to_string(),
                    ),
                ));
            }
            SettingsMessage::SelectGender(new_gender) => {
                if let Some((user_info, _)) = pending_user_info_changes {
                    user_info.gender = new_gender;
                }
            }
            SettingsMessage::EditHeight(new_height) => {
                if let Some((pending_info, pending_user_info_strings)) = pending_user_info_changes {
                    let digit_string = new_height
                        .chars()
                        .filter(|char| char.is_ascii_digit())
                        .take(3)
                        .collect();
                    pending_user_info_strings.height = digit_string;

                    let new_height_integer: u32 = pending_user_info_strings
                        .height
                        .parse()
                        .unwrap_or(existing_user_info.height);
                    pending_info.height = new_height_integer;
                }
            }
            SettingsMessage::EditWeight(new_weight) => {
                if let Some((pending_info, pending_user_info_strings)) = pending_user_info_changes {
                    let digit_string: String = new_weight
                        .chars()
                        .map(|char| if char == ',' { '.' } else { char })
                        .scan(false, |dot_seen, char| {
                            if char.is_ascii_digit() {
                                return Some(char);
                            }
                            if char.eq(&'.') && !*dot_seen {
                                *dot_seen = true;
                                return Some(char);
                            }
                            None
                        })
                        .take(5)
                        .collect();
                    pending_user_info_strings.weight = digit_string;

                    let new_weight_float: Kg = pending_user_info_strings
                        .weight
                        .parse()
                        .unwrap_or(existing_user_info.weight);
                    let new_weight_float_shortened = (new_weight_float * 10.0).round() / 10.0;
                    pending_info.weight = new_weight_float_shortened;
                }
            }
            SettingsMessage::EditWeeklyWorkoutGoal(new_goal) => {
                if let Some((pending_info, pending_user_info_strings)) = pending_user_info_changes {
                    let digit_string: String = new_goal
                        .chars()
                        .filter(|char| char.is_ascii_digit())
                        .take(2)
                        .collect();
                    pending_user_info_strings.weekly_workout_goal = digit_string;

                    let new_goal_integer: u32 = pending_user_info_strings
                        .weekly_workout_goal
                        .parse()
                        .map(|num: u32| if num == 0 { 1 } else { num })
                        .unwrap_or(existing_user_info.weekly_workout_goal);
                    pending_info.weekly_workout_goal = new_goal_integer;
                }
            }
            SettingsMessage::EditDescription(new_description) => {
                let cut_description = new_description
                    .chars()
                    .take(MAX_DESCRIPTION_CHARACTERS)
                    .collect();
                if let Some((user_info, _)) = pending_user_info_changes {
                    user_info.description = cut_description;
                }
            }
            SettingsMessage::EditMascot(new_mascot) => {
                if let Some((user_info, _)) = pending_user_info_changes {
                    user_info.favorite_mascot = new_mascot;
                }
            }
            SettingsMessage::SavePendingUserInfoChanges => {
                if let Some((pending_user_info, _)) =
                    app.user_manager.pending_user_info_changes.take()
                {
                    app.user_manager.user_info = pending_user_info.clone();
                    if let Some(jwt) = opt_jwt {
                        return Task::perform(
                            update_user_info_on_server(jwt, pending_user_info),
                            |result| Message::UpdateInfoOnServerResult(result, "user".to_string()),
                        );
                    } else {
                        println!("Log in to update User info!");
                    }
                }
            }
            SettingsMessage::DiscardPendingUserInfoChanges => {
                app.user_manager.pending_user_info_changes = None;
            }
            SettingsMessage::LogOut => {
                *app = App::default();
            }
        }
        Task::none()
    }
}
