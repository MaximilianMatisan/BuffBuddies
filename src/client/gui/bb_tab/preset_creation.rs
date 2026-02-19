use crate::client::backend::pop_up_manager::PopUpType;
use crate::client::backend::workout_preset_manager::PresetSafeError;
use crate::client::gui::app::App;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_tab::workout_creation::ExerciseNumber;
use crate::client::gui::bb_theme::color::BACKGROUND_COLOR;
use crate::client::gui::bb_theme::combo_box::{create_menu_style, create_text_input_style};
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::separator::separator_line;
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_button_text};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::client::server_communication::preset_communicator::save_preset;
use crate::common::mascot_mod::mascot::Mascot;
use iced::Color;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, combo_box, container, image, row, text, text_input};
use iced::{Element, Task};
use iced_core::Length::{Fill, FillPortion, Shrink};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::image::Handle;
use iced_core::text::LineHeight;
use iced_core::{Alignment, Length, Pixels};

#[derive(Clone, Debug)]
pub enum PresetCreationMessage {
    StartTitleEdit,
    EndTitleEdit,
    EditTitle(String),
    AddExercise(String),
    DeleteExercise(ExerciseNumber),
    FinishPresetCreation,
    SavePreset(Result<(), ServerRequestError>),
}

impl PresetCreationMessage {
    pub fn update(&self, app: &mut App) -> Task<Message> {
        match self {
            PresetCreationMessage::StartTitleEdit => {
                if let Some(preset) = &mut app.workout_preset_manager.preset_in_creation {
                    preset.edit_title = true;
                }
                Task::none()
            }
            PresetCreationMessage::EndTitleEdit => {
                if let Some(preset) = &mut app.workout_preset_manager.preset_in_creation {
                    if preset.workout_preset.name.is_empty() {
                        preset.workout_preset.name = "Preset Title".to_string();
                    }
                    preset.edit_title = false;
                }
                Task::none()
            }
            PresetCreationMessage::EditTitle(str) => {
                if let Some(preset) = &mut app.workout_preset_manager.preset_in_creation {
                    preset.workout_preset.name = str.to_string();
                }
                Task::none()
            }
            PresetCreationMessage::AddExercise(exercise_name) => {
                if let Some(preset) = &mut app.workout_preset_manager.preset_in_creation {
                    preset
                        .workout_preset
                        .exercises
                        .push(exercise_name.to_string())
                }
                Task::none()
            }
            PresetCreationMessage::DeleteExercise(exercise_number) => {
                if let Some(preset) = &mut app.workout_preset_manager.preset_in_creation {
                    preset.workout_preset.exercises.remove(exercise_number - 1);
                }
                Task::none()
            }
            PresetCreationMessage::FinishPresetCreation => {
                if let Err(preset_safe_error) = app.workout_preset_manager.check_preset() {
                    match preset_safe_error {
                        PresetSafeError::NameAlreadyExists => {
                            app.pop_up_manager.new_pop_up(PopUpType::Minor, "Preset name already exists!".to_string(), "A preset with that name already exists.\nPlease choose another preset name!".to_string())
                        }
                        PresetSafeError::NoPresetToSafe => {
                            app.pop_up_manager.new_pop_up(PopUpType::Minor, "Something went wrong during Preset saving!".to_string(), "Please try again or report bug".to_string())
                        }
                        PresetSafeError::PresetEmpty => {
                            app.pop_up_manager.new_pop_up(PopUpType::Minor, "Preset can't be empty!".to_string(), "Please first add exercises before saving preset".to_string())
                        }
                        PresetSafeError::NameEmpty => {
                            app.pop_up_manager.new_pop_up(PopUpType::Minor, "Preset needs to have a name!".to_string(), "Please add a name to your preset".to_string())
                        }
                    }
                    Task::none()
                } else if let Some(jwt) = &app.jsonwebtoken.clone() {
                    Task::perform(
                        save_preset(
                            jwt.clone(),
                            app.workout_preset_manager
                                .preset_in_creation
                                .as_ref()
                                .unwrap()
                                .workout_preset
                                .clone(),
                        ),
                        |result| -> Message {
                            Message::PresetCreation(PresetCreationMessage::SavePreset(result))
                        },
                    )
                } else {
                    app.pop_up_manager.new_pop_up(
                        PopUpType::Minor,
                        "Log in to save preset!".to_string(),
                        "You need to be logged in to save a preset.".to_string(),
                    );
                    Task::none()
                }
            }
            PresetCreationMessage::SavePreset(Ok(())) => {
                app.workout_preset_manager.add_preset();
                app.screen = Tab::Workout;
                Task::none()
            }
            PresetCreationMessage::SavePreset(Err(err)) => {
                app.pop_up_manager.new_pop_up(
                    PopUpType::Minor,
                    err.to_error_message().to_string(),
                    "".to_string(),
                );
                app.screen = Tab::Workout;
                Task::none()
            }
        }
    }
}

fn view_exercise_preset(
    exercise_name: String,
    exercise_number: ExerciseNumber,
    selected_mascot: &Mascot,
) -> Element<Message> {
    let number: Element<Message> = format_button_text(text(format!("{exercise_number}.")))
        .size(30)
        .width(FillPortion(1))
        .into();

    let exercise_name: Element<Message> = format_button_text(text(exercise_name))
        .size(20)
        .align_x(Horizontal::Left)
        .align_y(Vertical::Center)
        .width(FillPortion(6))
        .into();

    let delete_button: Element<Message> = create_element_button(
        selected_mascot,
        image(Handle::from_path("assets/images/trash_black.png")).into(),
        ButtonStyle::InactiveTransparent,
        None,
    )
    .width(Shrink)
    .on_press(Message::PresetCreation(
        PresetCreationMessage::DeleteExercise(exercise_number),
    ))
    .into();

    let row = Row::new()
        .push(number)
        .push(exercise_name)
        .push(delete_button)
        .spacing(30)
        .align_y(Vertical::Center)
        .height(Shrink);

    container(row)
        .style(create_container_style(ContainerStyle::Light, None, None))
        .height(Shrink)
        .padding(5)
        .into()
}
impl App {
    pub fn preset_creation_screen(&self) -> Element<Message> {
        let preset_name =
            if let Some(current_preset) = &self.workout_preset_manager.preset_in_creation {
                current_preset.workout_preset.name.clone().to_string()
            } else {
                "".to_string()
            };

        let mut title_row = Row::new();

        if let Some(preset) = &self.workout_preset_manager.preset_in_creation {
            if preset.edit_title {
                let title: Element<Message> = container(
                    text_input("Enter title...", &preset.workout_preset.name)
                        .style(create_text_input_style(
                            &self.mascot_manager.selected_mascot,
                            Color::default(),
                        ))
                        .font(FIRA_SANS_EXTRABOLD)
                        .on_input(|str| -> Message {
                            Message::PresetCreation(PresetCreationMessage::EditTitle(str))
                        })
                        .align_x(Alignment::Center)
                        .size(40),
                )
                .width(300.0)
                .height(50.0)
                .into();
                let check_image = image(Handle::from_path("assets/images/check_box.png")).into();
                let done_button: Element<Message> = create_element_button(
                    &self.mascot_manager.selected_mascot,
                    check_image,
                    ButtonStyle::Active,
                    None,
                )
                .on_press(Message::PresetCreation(PresetCreationMessage::EndTitleEdit))
                .into();
                title_row = Row::new().push(title).push(done_button).spacing(INDENT);
            } else {
                let title_text = format_button_text(text(preset_name))
                    .size(40)
                    .width(300.0)
                    .height(50.0);
                let edit_image = image(Handle::from_path("assets/images/edit.png")).into();
                let edit_button: Element<Message> = create_element_button(
                    &self.mascot_manager.selected_mascot,
                    edit_image,
                    ButtonStyle::Active,
                    None,
                )
                .on_press(Message::PresetCreation(
                    PresetCreationMessage::StartTitleEdit,
                ))
                .into();
                title_row = Row::new()
                    .push(title_text)
                    .push(edit_button)
                    .spacing(INDENT);
            }
        }
        let title_container = container(title_row).center(Fill).height(Shrink).width(Fill);

        let separator = separator_line(
            &self.mascot_manager.selected_mascot,
            Length::Fixed(5.0),
        );

        let mut column = Column::new().spacing(INDENT);

        if let Some(current_preset) = &self.workout_preset_manager.preset_in_creation {
            let mut counter: ExerciseNumber = 1;
            for exercise in current_preset.workout_preset.exercises.clone() {
                column = column.push(view_exercise_preset(
                    exercise,
                    counter,
                    &self.mascot_manager.selected_mascot,
                ));
                counter += 1;
            }
        }

        let exercises = Scrollable::new(column)
            .direction(Direction::Vertical(Scrollbar::new()))
            .height(FillPortion(5));

        let add_exercise: Element<Message> = combo_box(
            &self.exercise_manager.all_exercise_state,
            "Search for exercise...",
            None,
            |exercise: String| -> Message {
                Message::PresetCreation(PresetCreationMessage::AddExercise(exercise))
            },
        )
        .menu_style(create_menu_style(&self.mascot_manager.selected_mascot))
        .input_style(create_text_input_style(
            &self.mascot_manager.selected_mascot,
            BACKGROUND_COLOR,
        ))
        .font(FIRA_SANS_EXTRABOLD)
        .line_height(LineHeight::Absolute(Pixels(30.0)))
        .into();

        let finish_preset_text = format_button_text(text("Finish Preset Creation"))
            .size(25)
            .center();

        let check_box_image = image(Handle::from_path("assets/images/check_box.png"));

        let finish_preset_image_and_text =
            container(row![check_box_image, finish_preset_text]).center_x(Fill);

        let finish_preset_button: Element<Message> = create_element_button(
            &self.mascot_manager.selected_mascot,
            finish_preset_image_and_text.into(),
            ButtonStyle::Active,
            None,
        )
        .width(Fill)
        .on_press(Message::PresetCreation(
            PresetCreationMessage::FinishPresetCreation,
        ))
        .into();

        let add_exercise_and_finish: Column<Message> = Column::new()
            .push(add_exercise)
            .push(finish_preset_button)
            .spacing(10)
            .height(FillPortion(1))
            .width(Fill);

        let exercises_and_bottom_stuff = Column::new()
            .push(title_container)
            .push(separator)
            .push(exercises)
            .push(add_exercise_and_finish)
            .spacing(20);

        let inner_container = container(exercises_and_bottom_stuff)
            .height(Fill)
            .width(Fill)
            .padding(20)
            .style(create_container_style(ContainerStyle::Default, None, None));

        container(inner_container)
            .width(Fill)
            .height(Fill)
            .style(create_container_style(
                ContainerStyle::Background,
                None,
                None,
            ))
            .padding(50)
            .into()
    }
}
