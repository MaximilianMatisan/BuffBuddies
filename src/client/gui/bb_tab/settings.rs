use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::color::{BACKGROUND_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::combo_box::{create_menu_style, create_text_input_style};
use crate::client::gui::bb_theme::container::{
    ContainerStyle, DEFAULT_TEXT_CONTAINER_PADDING, create_container_style,
};
use crate::client::gui::bb_theme::custom_button::{
    BUTTON_RADIUS_LEFT_ZERO, BUTTON_RADIUS_RIGHT_ZERO, ButtonStyle, create_element_button,
    create_text_button,
};
use crate::client::gui::bb_theme::separator::{DEFAULT_SEPARATOR_HEIGHT, separator_line};
use crate::client::gui::bb_theme::text_format::{
    FIRA_SANS_EXTRABOLD, cm_to_string, format_button_text, format_description_text, kg_to_string,
};
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::bb_widget::widget_utils::{
    descriptor_space_fill_element_row, descriptor_space_fill_text_row,
};
use crate::client::gui::user_interface::Message;
use crate::client::server_communication::user_communicator::update_user_info_on_server;
use crate::common::exercise_mod::weight::Kg;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::profile_picture::{
    LARGE_PROFILE_PICTURE_DIMENSION, profile_picture_selection_row,
};
use crate::common::user_mod::user::{
    Gender, MAX_DESCRIPTION_CHARACTERS, UserInformation, UserInformationStrings,
};
use crate::common::user_mod::user_goals::GoalType;
use iced::widget::{
    Column, Container, Image, Row, Space, TextInput, combo_box, container, image, text, text_input,
};
use iced::{Element, Task};
use iced_core::alignment::Vertical;
use iced_core::image::Handle;
use iced_core::{Length, Padding};
use strum::IntoEnumIterator;

const SETTINGS_ENTRY_SPACING: f32 = 5.0;
const SETTINGS_TEXT_INPUT_WIDTH: f32 = 250.0;
const TITLE_SIZE: f32 = 30.0;
const USER_DATA_TITLE: &str = "General info";
const GOAL_DATA_TITLE: &str = "Goals";
const PROFILE_PICTURE_TITLE: &str = "Profile picture";

impl App {
    pub fn settings_screen(&self) -> Element<Message> {
        let user_info_container = user_settings(self).map(Message::Settings);
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
            .padding(Padding {
                top: 20.0,
                ..0.0.into()
            })
            .into();

        padded_content
    }
}
fn user_settings(app: &App) -> Element<SettingsMessage> {
    if app.user_manager.pending_user_info_changes.is_none() {
        user_settings_preview(app)
    } else {
        user_settings_edit(app)
    }
}
fn user_settings_preview(app: &App) -> Element<SettingsMessage> {
    let user_info = &app.user_manager.user_info;

    let profile_picture = image(Handle::from_path(&user_info.profile_picture_path))
        .width(LARGE_PROFILE_PICTURE_DIMENSION)
        .height(LARGE_PROFILE_PICTURE_DIMENSION);

    let username_and_data_column = Column::new()
        .spacing(INDENT)
        .push(preview_user_info_column(app))
        .push(preview_goals_column(app));

    arrange_and_wrap_user_settings_in_container(profile_picture, username_and_data_column).into()
}
fn user_settings_edit(app: &App) -> Element<SettingsMessage> {
    if let Some((pending_info, _)) = &app.user_manager.pending_user_info_changes {
        let profile_picture = image(Handle::from_path(pending_info.profile_picture_path.clone()))
            .width(LARGE_PROFILE_PICTURE_DIMENSION)
            .height(LARGE_PROFILE_PICTURE_DIMENSION);

        let username_and_data_column = Column::new()
            .spacing(INDENT)
            .push(edit_user_info_column(app))
            .push(edit_goals_column(app))
            .push(edit_profile_picture_column(
                &app.mascot_manager.selected_mascot,
            ))
            .push(save_or_discard_pending_user_info(
                &app.mascot_manager.selected_mascot,
            ));

        arrange_and_wrap_user_settings_in_container(profile_picture, username_and_data_column)
            .into()
    } else {
        Column::new().into()
    }
}
fn arrange_and_wrap_user_settings_in_container(
    profile_picture: Image,
    username_and_data_column: Column<SettingsMessage>,
) -> Container<SettingsMessage> {
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

    user_info_container
}
fn preview_user_info_column(app: &App) -> Column<SettingsMessage> {
    let user_info = &app.user_manager.user_info;

    let username = text(&user_info.username)
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR)
        .size(40);

    let edit_profile_button: iced_anim::widget::Button<SettingsMessage> = create_element_button(
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
        .width(Length::Shrink)
        .padding(DEFAULT_TEXT_CONTAINER_PADDING); //[top/bottom, left/right]

    let description = Row::new()
        .push(format_description_text(text("Description:")))
        .push(Space::with_width(Length::Fill))
        .push(description_text_container);

    let user_data_title =
        create_settings_sub_header(&app.mascot_manager.selected_mascot, USER_DATA_TITLE);

    let user_data_column = Column::new()
        .push(user_data_title)
        .push(descriptor_space_fill_text_row(
            "Favorite Mascot:".to_string(),
            app.user_manager.user_info.favorite_mascot.to_string(),
        ))
        .push(descriptor_space_fill_text_row(
            "Gender:".to_string(),
            user_info.gender.to_string(),
        ))
        .push(descriptor_space_fill_text_row(
            "Weight:".to_string(),
            kg_to_string(user_info.weight),
        ))
        .push(descriptor_space_fill_text_row(
            "Height:".to_string(),
            cm_to_string(user_info.height),
        ))
        .push(description)
        .spacing(SETTINGS_ENTRY_SPACING)
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

    let user_data_title =
        create_settings_sub_header(&app.mascot_manager.selected_mascot, USER_DATA_TITLE);

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
    .input_style(create_text_input_style(
        &app.mascot_manager.selected_mascot,
        BACKGROUND_COLOR,
    ))
    .menu_style(create_menu_style(&app.mascot_manager.selected_mascot));

    let height_text_input = text_input("Enter your height in cm", &pending_info_strings.height)
        .on_input(SettingsMessage::EditHeight);

    let weight_text_input = text_input("Enter your weight in kg", &pending_info_strings.weight)
        .on_input(SettingsMessage::EditWeight);

    let mascot_combo_box: Element<SettingsMessage> = combo_box(
        &app.mascot_manager.owned_mascots_state,
        "Select mascot...",
        Some(&pending_info.favorite_mascot),
        SettingsMessage::EditMascot,
    )
    .font(FIRA_SANS_EXTRABOLD)
    .width(SETTINGS_TEXT_INPUT_WIDTH)
    .input_style(create_text_input_style(
        &app.mascot_manager.selected_mascot,
        BACKGROUND_COLOR,
    ))
    .menu_style(create_menu_style(&app.mascot_manager.selected_mascot))
    .into();

    //Didn't use pending_info_strings here as description is already a string in pending_info
    let description_text_input = text_input("Tell something about you!", &pending_info.description)
        .on_input(SettingsMessage::EditDescription);

    let text_input_data_fields: [(&str, TextInput<SettingsMessage>); 3] = [
        ("Weight:", weight_text_input),
        ("Height:", height_text_input),
        ("Description:", description_text_input),
    ];

    let mut user_data_column = Column::new()
        .spacing(SETTINGS_ENTRY_SPACING)
        .push(descriptor_space_fill_element_row(
            "Favorite Mascot:".to_string(),
            mascot_combo_box,
        ))
        .push(descriptor_space_fill_element_row(
            "Gender:".to_string(),
            gender_combo_box.into(),
        ));

    for (description_text, mut text_input) in text_input_data_fields {
        text_input = text_input
            .style(create_text_input_style(
                &app.mascot_manager.selected_mascot,
                BACKGROUND_COLOR,
            ))
            .font(FIRA_SANS_EXTRABOLD)
            .width(SETTINGS_TEXT_INPUT_WIDTH);
        user_data_column = user_data_column.push(descriptor_space_fill_element_row(
            description_text.to_string(),
            text_input.into(),
        ));
    }

    let username_and_data_column = Column::new()
        .push(username)
        .push(user_data_title)
        .push(user_data_column)
        .spacing(INDENT)
        .width(Length::FillPortion(15));

    username_and_data_column
}
fn preview_goals_column(app: &App) -> Column<SettingsMessage> {
    let goal_title =
        create_settings_sub_header(&app.mascot_manager.selected_mascot, GOAL_DATA_TITLE);

    let mut goal_previews = Column::new()
        .spacing(SETTINGS_ENTRY_SPACING)
        .push(goal_title);

    for goal in GoalType::iter() {
        let descriptor = format!("{}:", goal);
        goal_previews = goal_previews.push(descriptor_space_fill_text_row(
            descriptor,
            goal.get_formatted_user_goal_strings(&app.user_manager.user_info.user_goals),
        ));
    }

    goal_previews
}
fn edit_goals_column(app: &App) -> Column<SettingsMessage> {
    let pending_info = if let Some((user_info, _)) = &app.user_manager.pending_user_info_changes {
        user_info
    } else {
        return Column::new();
    };

    let goal_title =
        create_settings_sub_header(&app.mascot_manager.selected_mascot, GOAL_DATA_TITLE);

    let mut contents = Column::new()
        .spacing(SETTINGS_ENTRY_SPACING)
        .push(goal_title);

    for goal in GoalType::iter() {
        let descriptor = format!("{}:", goal);
        contents = contents.push(descriptor_space_fill_element_row(
            descriptor,
            number_inc_decrementer_buttons(
                &app.mascot_manager.selected_mascot,
                goal.get_formatted_user_goal_strings(&pending_info.user_goals),
                SettingsMessage::IncrementGoalValue(goal.clone()),
                SettingsMessage::DecrementGoalValue(goal),
            )
            .into(),
        ))
    }
    contents
}
fn edit_profile_picture_column(mascot: &Mascot) -> Column<SettingsMessage> {
    let header = create_settings_sub_header(mascot, PROFILE_PICTURE_TITLE);

    let picture_selection_row = profile_picture_selection_row(mascot);

    let content = Column::new()
        .push(header)
        .push(picture_selection_row)
        .spacing(SETTINGS_ENTRY_SPACING);

    content
}
fn number_inc_decrementer_buttons(
    mascot: &Mascot,
    number: String,
    increment_message: SettingsMessage,
    decrement_message: SettingsMessage,
) -> Row<SettingsMessage> {
    let number_text = format_button_text(text(number));
    let increment_button = create_text_button(
        mascot,
        "+".to_string(),
        ButtonStyle::Active,
        Some(BUTTON_RADIUS_RIGHT_ZERO),
    )
    .on_press(increment_message);
    let decrement_button = create_text_button(
        mascot,
        "-".to_string(),
        ButtonStyle::InactiveTab,
        Some(BUTTON_RADIUS_LEFT_ZERO),
    )
    .on_press(decrement_message);

    let row = Row::new()
        .push(number_text)
        .push(Space::with_width(INDENT / 2.0))
        .push(increment_button)
        .push(decrement_button)
        .align_y(Vertical::Center);

    row
}
fn create_settings_sub_header<'a>(
    mascot: &'a Mascot,
    header_text: &'a str,
) -> Column<'a, SettingsMessage> {
    let separator = separator_line(mascot, DEFAULT_SEPARATOR_HEIGHT);
    let user_data_title = format_button_text(text(header_text)).size(TITLE_SIZE);

    Column::new()
        .spacing(INDENT / 2.0)
        .push(separator)
        .push(user_data_title)
}
fn save_or_discard_pending_user_info(mascot: &Mascot) -> Row<SettingsMessage> {
    let save_changes_button = create_text_button(
        mascot,
        "Save changes".to_string(),
        ButtonStyle::Active,
        None,
    )
    .width(Length::Fill)
    .on_press(SettingsMessage::SavePendingUserInfoChanges);

    let discard_changes_button = create_text_button(
        mascot,
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
    button_row
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
    SelectProfilePicture(String),
    EditHeight(String),
    EditWeight(String),
    EditDescription(String),
    EditMascot(Mascot),
    IncrementGoalValue(GoalType),
    DecrementGoalValue(GoalType),
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
                    ),
                ));
            }
            SettingsMessage::SelectGender(new_gender) => {
                if let Some((user_info, _)) = pending_user_info_changes {
                    user_info.gender = new_gender;
                }
            }
            SettingsMessage::SelectProfilePicture(new_profile_picture_path) => {
                if let Some((user_info, _)) = pending_user_info_changes {
                    user_info.profile_picture_path = new_profile_picture_path;
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
                app.widget_manager
                    .bmi_widget_state
                    .animation_progress
                    .settle_at(0.0);
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
                app.widget_manager
                    .bmi_widget_state
                    .animation_progress
                    .settle_at(0.0);
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
            SettingsMessage::IncrementGoalValue(goal_type) => {
                if let Some((pending_info, _)) = pending_user_info_changes {
                    pending_info.user_goals.update_user_goals(&goal_type, true);
                }
            }
            SettingsMessage::DecrementGoalValue(goal_type) => {
                if let Some((pending_info, _)) = pending_user_info_changes {
                    pending_info.user_goals.update_user_goals(&goal_type, false);
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

                app.widget_manager
                    .progress_bar_state_manager
                    .update_goals(&app.user_manager.user_info)
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
