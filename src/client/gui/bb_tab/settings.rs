use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::{TEXT_COLOR};
use crate::client::gui::bb_theme::combo_box::{create_menu_style, create_text_input_style};
use crate::client::gui::bb_theme::container::{ContainerStyle, create_style_container};
use crate::client::gui::bb_theme::custom_button::{
    ButtonStyle, create_element_button, create_text_button,
};
use crate::client::gui::bb_theme::text_format::{
    FIRA_SANS_EXTRABOLD, cm_to_string, format_button_text, format_description_text, kg_to_string,
};
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::size;
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::{Element, Task};
use iced::widget::{Column, ComboBox, Container, Row, Space, TextInput, combo_box, container, image, text, text_input, Button};
use iced_core::image::Handle;
use iced_core::{Length, Padding, Theme};
use crate::client::backend::exercise_mod::weight::Kg;
use crate::client::backend::user_mod::user::{Gender, MAX_DESCRIPTION_CHARACTERS};

const SETTINGS_TEXT_INPUT_WIDTH: f32 = 250.0;
impl UserInterface {
    pub fn settings_screen(&self) -> Element<Message> {
        settings_user_info_preview(&self.app).map(|msg| Message::Settings(msg))
    }
}
fn settings_user_info_preview(app: &App) -> Element<SettingsMessage> {
    let user_info = &app.user_manager.user_info;

    let profile_picture = image(Handle::from_path(user_info.profile_picture_handle.clone()))
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
        .style(create_style_container(ContainerStyle::Default, None, None))
        .height(Length::Shrink)
        .width(Length::FillPortion(10))
        .padding(Padding {
            top: LARGE_INDENT,
            bottom: LARGE_INDENT,
            ..Default::default()
        });

    let user_info_element: Element<SettingsMessage> = Row::new()
        .push(Space::with_width(Length::FillPortion(1)))
        .push(user_info_container)
        .push(Space::with_width(Length::FillPortion(1)))
        .into();

    user_info_element
}
fn preview_user_info_column(app: &App) -> Column<SettingsMessage> {
    let user_info = &app.user_manager.user_info;
    let username = text(user_info.username.clone())
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
        text(user_info.description.clone())
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR)
    };
    let description_text_container: Container<SettingsMessage> = container(description_text)
        .style(create_style_container(
            ContainerStyle::Background,
            None,
            None,
        ))
        .width(SETTINGS_TEXT_INPUT_WIDTH)
        .padding([3.0, INDENT]); //[top/bottom, left/right]

    let description = Row::new()
        .push(format_description_text(text("Description:")))
        .push(Space::with_width(Length::Fill))
        .push(description_text_container);

    let user_data_column = Column::new()
        .push(create_user_data_preview(
            "Gender:",
            user_info.gender.to_string(),
        ))
        .push(create_user_data_preview(
            "Weight:",
            kg_to_string(user_info.weight),
        ))
        .push(create_user_data_preview(
            "Height:",
            cm_to_string(user_info.height),
        ))
        .push(create_user_data_preview(
            "Weekly workout goal:",
            user_info.weekly_workout_goal.to_string(),
        ))
        .push(create_user_data_preview(
            "Favorite Mascot:",
            app.mascot_manager.favorite_mascot.to_string(),
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
    let pending_info = if let Some(pending) = &app.user_manager.pending_user_info_changes {
        pending
    } else {
        return Column::new();
    };
    //Username should currently not be changeable
    let username = text(pending_info.username.clone())
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

    let height_text_input = text_input("Enter your height in cm", &pending_info.height.to_string())
        .on_input(SettingsMessage::EditHeight);

    let weight_text_input = text_input("Enter your weight in kg", &pending_info.weight.to_string())
        .on_input(SettingsMessage::EditWeight);

    let weekly_workout_goal_text_input = text_input(
        "Enter your weekly workout goal",
        &pending_info.weekly_workout_goal.to_string(),
    )
    .on_input(SettingsMessage::EditWeeklyWorkoutGoal);

    let mascot_combo_box: ComboBox<Message, Theme>; //TODO create and add to user_data_column

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
        .push(create_user_data_entry("Gender:", gender_combo_box.into()));

    for (description_text, mut text_input) in text_input_data_fields {
        text_input = text_input
            .style(create_text_input_style(&app.mascot_manager.selected_mascot))
            .font(FIRA_SANS_EXTRABOLD)
            .width(SETTINGS_TEXT_INPUT_WIDTH);
        user_data_column =
            user_data_column.push(create_user_data_entry(description_text, text_input.into()));
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

fn create_user_data_preview(description_text: &str, information_text: String) -> Row<SettingsMessage> {
    create_user_data_entry(
        description_text,
        format_button_text(text(information_text)).into(),
    )
}
fn create_user_data_entry<'a>(
    description_text: &'a str,
    data_element: Element<'a, SettingsMessage>,
) -> Row<'a, SettingsMessage> {
    Row::new()
        .push(format_description_text(text(description_text)))
        .push(Space::with_width(Length::Fill))
        .push(data_element)
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    StartEditingProfile,
    SelectGender(Gender),
    EditHeight(String),
    EditWeight(String),
    EditWeeklyWorkoutGoal(String),
    EditDescription(String),
    SavePendingUserInfoChanges,
    DiscardPendingUserInfoChanges,
}
impl SettingsMessage {
    pub fn update(self, ui: &mut UserInterface) -> Task<Message> {
        let pending_user_info_changes = &mut ui.app.user_manager.pending_user_info_changes;
        match self {
            SettingsMessage::StartEditingProfile => {
                ui.app.user_manager.pending_user_info_changes =
                    Some(ui.app.user_manager.user_info.clone());
            }
            SettingsMessage::SelectGender(new_gender) => {
                if let Some(pending) = pending_user_info_changes {
                    pending.gender = new_gender;
                }
            }
            SettingsMessage::EditHeight(new_height) => {
                let numbers: u32 = new_height.parse().unwrap_or_else(|_| 0);
                if let Some(pending) = pending_user_info_changes {
                    pending.height = numbers;
                }
            }
            SettingsMessage::EditWeight(new_weight) => {
                //TODO floats don't work rn idk what's the issue
                let number: Kg = new_weight.parse().unwrap_or_else(|_| 0.0);
                if let Some(pending) = pending_user_info_changes {
                    pending.weight = number;
                }
            }
            SettingsMessage::EditWeeklyWorkoutGoal(new_goal) => {
                let number: u32 = new_goal.parse().unwrap_or_else(|_| 0);
                if let Some(pending) = pending_user_info_changes {
                    pending.weekly_workout_goal = number;
                }
            }
            SettingsMessage::EditDescription(new_description) => {
                let cut_description = new_description
                    .chars()
                    .take(MAX_DESCRIPTION_CHARACTERS)
                    .collect();
                if let Some(pending) = pending_user_info_changes {
                    pending.description = cut_description;
                }
            }
            SettingsMessage::SavePendingUserInfoChanges => {
                if let Some(pending_changes) =
                    ui.app.user_manager.pending_user_info_changes.take()
                {
                    ui.app.user_manager.user_info = pending_changes;
                    //TODO SEND TO DATABASE
                }
            }
            SettingsMessage::DiscardPendingUserInfoChanges => {
                ui.app.user_manager.pending_user_info_changes = None;
            }
        }
        Task::none()
    }
}