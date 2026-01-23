use crate::client::backend::exercise::exercise_create::ExerciseCreate;
use crate::client::backend::exercise::set::StrengthSet;
use crate::client::backend::exercise::weight::Kg;
use crate::client::backend::pop_up_manager::PopUpType;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_theme::combo_box::{create_menu_style, create_text_input_style};
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{
    FIRA_SANS_EXTRABOLD, format_button_text, format_description_text,
};
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{
    Column, Row, Scrollable, Space, combo_box, container, image, row, stack, text, text_input,
};
use iced::{Element, Task};
use iced_core::Length::{Fill, FillPortion, Shrink};
use iced_core::image::Handle;
use iced_core::text::LineHeight;
use iced_core::{Alignment, Pixels};

type SetNumber = usize;
pub type ExerciseNumber = usize;

#[derive(Clone, Debug)]
pub enum WorkoutCreationMessage {
    AddSet,
    DeleteSet(SetNumber),
    AddExercise(String),
    DeleteExercise(ExerciseNumber),
    BeginExerciseEdit(ExerciseNumber),
    FinishExerciseEdit,
    EditKg(SetNumber, String),
    EditReps(SetNumber, String),
    FinishWorkoutCreation,
}

impl WorkoutCreationMessage {
    pub fn update(&self, user_interface: &mut UserInterface) -> Task<Message> {
        let exercise_in_edit_number =
            &mut user_interface.app.exercise_manager.exercise_in_edit_number;
        let workout_in_creation = &mut user_interface.app.exercise_manager.workout_in_creation;
        let exercise_in_edit_strings =
            &mut user_interface.app.exercise_manager.exercise_in_edit_strings;

        match self {
            WorkoutCreationMessage::AddSet => {
                let mut name: String = "".to_string();
                if let Some(workout) = workout_in_creation {
                    name = workout[exercise_in_edit_number.unwrap() - 1]
                        .name
                        .clone()
                        .to_string();
                }
                add_set(user_interface, &name);
                Task::none()
            }
            WorkoutCreationMessage::DeleteSet(set_number) => {
                if let Some(workout) = &mut user_interface.app.exercise_manager.workout_in_creation
                {
                    workout[user_interface
                        .app
                        .exercise_manager
                        .exercise_in_edit_number
                        .unwrap()
                        - 1]
                    .sets
                    .remove(*set_number - 1);
                    *exercise_in_edit_strings = Some(
                        workout[user_interface
                            .app
                            .exercise_manager
                            .exercise_in_edit_number
                            .unwrap()
                            - 1]
                        .clone()
                        .into(),
                    );
                }
                Task::none()
            }
            WorkoutCreationMessage::AddExercise(name) => {
                if let Some(workout) = workout_in_creation {
                    workout.push(ExerciseCreate::new(name.to_string()));
                    let exercise_number = workout.len();
                    *exercise_in_edit_number = Some(exercise_number);
                    *exercise_in_edit_strings = Some(workout[exercise_number - 1].clone().into());
                    add_set(user_interface, name);
                }
                Task::none()
            }
            WorkoutCreationMessage::DeleteExercise(exercise_number) => {
                if let Some(workout) = workout_in_creation {
                    (*workout).remove(*exercise_number - 1);
                    if let Some(exercise_number_edit) = *exercise_in_edit_number {
                        if *exercise_number == exercise_number_edit {
                            *exercise_in_edit_number = None;
                            user_interface.app.exercise_manager.exercise_in_edit_strings = None;
                        }
                    }
                }
                Task::none()
            }
            WorkoutCreationMessage::BeginExerciseEdit(exercise_number) => {
                *exercise_in_edit_number = Some(*exercise_number);
                if let Some(workout) = workout_in_creation {
                    *exercise_in_edit_strings = Some(workout[*exercise_number - 1].clone().into())
                }
                Task::none()
            }
            WorkoutCreationMessage::FinishExerciseEdit => {
                *exercise_in_edit_number = None;
                *exercise_in_edit_strings = None;
                Task::none()
            }
            WorkoutCreationMessage::EditKg(set_number, new_kg) => {
                if let Some(workout) = workout_in_creation {
                    let mut dot: bool = false;
                    let mut digits_before_dot: usize = 0;
                    let mut digits_after_dot: usize = 0;

                    let digit_string: String = new_kg
                        .chars()
                        .filter(|c| {
                            if c.is_ascii_digit() {
                                if !dot {
                                    if digits_before_dot < 3 {
                                        digits_before_dot += 1;
                                        true
                                    } else {
                                        false
                                    }
                                } else if digits_after_dot < 2 {
                                    digits_after_dot += 1;
                                    true
                                } else {
                                    false
                                }
                            } else if *c == '.' && !dot {
                                dot = true;
                                true
                            } else {
                                false
                            }
                        })
                        .collect();
                    if let Some(exercise_strings) = exercise_in_edit_strings {
                        exercise_strings.sets[set_number - 1].kg = digit_string.clone();
                    }
                    let new_kg_integer: Kg = digit_string.parse().unwrap_or(0.0);

                    workout[exercise_in_edit_number.unwrap() - 1].sets[*set_number - 1].weight =
                        new_kg_integer;
                }
                Task::none()
            }
            WorkoutCreationMessage::EditReps(set_number, new_reps) => {
                if let Some(workout) = workout_in_creation {
                    let digit_string: String = new_reps
                        .chars()
                        .filter(|char| char.is_ascii_digit())
                        .take(3)
                        .collect();
                    if let Some(exercise_strings) = exercise_in_edit_strings {
                        exercise_strings.sets[set_number - 1].reps = digit_string.clone();
                    }
                    let new_reps_integer: u32 = digit_string.parse().unwrap_or(0);

                    workout[exercise_in_edit_number.unwrap() - 1].sets[*set_number - 1].reps =
                        new_reps_integer;
                }
                Task::none()
            }
            WorkoutCreationMessage::FinishWorkoutCreation => {
                if let Some(workout) = &user_interface.app.exercise_manager.workout_in_creation {
                    if let Err(()) = user_interface
                        .app
                        .exercise_manager
                        .save_workout(&workout.clone())
                    {
                        user_interface.app.pop_up_manager.new_pop_up(
                            PopUpType::Minor,
                            "Failed Saving Workout".to_string(),
                            "Something went wrong during the saving progress of your workout"
                                .to_string(),
                        );
                    }
                }
                user_interface.app.exercise_manager.clear_workout();
                user_interface.app.screen = Tab::Workout;
                Task::none()
            }
        }
    }
}

pub fn add_set(user_interface: &mut UserInterface, exercise_name: &String) {
    let set = user_interface
        .app
        .exercise_manager
        .get_last_done_set(exercise_name)
        .unwrap_or_default();
    if let Some(workout) = &mut user_interface.app.exercise_manager.workout_in_creation {
        let exercise_sets = &mut workout[user_interface
            .app
            .exercise_manager
            .exercise_in_edit_number
            .unwrap()
            - 1]
        .sets;
        if !exercise_sets.is_empty() {
            //take the last set and add a copy of it to the sets
            (*exercise_sets).push(exercise_sets[exercise_sets.len() - 1].clone());
        } else {
            (*exercise_sets).push(set.clone());
        }
        user_interface.app.exercise_manager.exercise_in_edit_strings = Some(
            workout[user_interface
                .app
                .exercise_manager
                .exercise_in_edit_number
                .unwrap()
                - 1]
            .clone()
            .into(),
        );
    }
}

pub fn view_exercise<'a>(
    exercise: &'a ExerciseCreate,
    number: ExerciseNumber,
    user_interface: &'a UserInterface,
) -> Element<'a, Message> {
    if let Some(edited_exercise) = user_interface.app.exercise_manager.exercise_in_edit_number {
        if edited_exercise == number {
            return view_exercise_edit(exercise, number, user_interface);
        }
    }
    view_exercise_no_edit(exercise, number, user_interface)
}

pub fn view_exercise_edit<'a>(
    exercise: &ExerciseCreate,
    exercise_number: ExerciseNumber,
    user_interface: &'a UserInterface,
) -> Element<'a, Message> {
    let done_text = format_button_text(text(" Done")).size(20).center();

    let check_box_image = image(Handle::from_path("assets/images/check_box.png")).height(25);

    let done_row = row![check_box_image, done_text];

    let done_button: Element<Message> = container(
        create_element_button(
            &user_interface.app.mascot_manager.selected_mascot,
            done_row.into(),
            ButtonStyle::Active,
            None,
        )
        .width(Shrink)
        .on_press(Message::WorkoutCreation(
            WorkoutCreationMessage::FinishExerciseEdit,
        )),
    )
    .height(Shrink)
    .width(FillPortion(1))
    .into();

    let mut exercise_name: Element<Message> = Column::new().into();

    if let Some(workout) = &user_interface.app.exercise_manager.workout_in_creation {
        exercise_name = view_exercise_name(workout, exercise_number);
    }

    let delete_button: Element<Message> = view_delete_button(exercise_number, user_interface);

    let top_row = Row::new()
        .push(done_button)
        .push(exercise_name)
        .push(delete_button)
        .spacing(10);

    let mut counter: SetNumber = 1;
    let mut sets_column = Column::new().spacing(10).width(Fill);

    let descriptions: Element<Message> = view_descriptions(user_interface);

    sets_column = sets_column.push(descriptions);

    for _set in &exercise.sets {
        sets_column = sets_column.push(view_set_edit(counter, user_interface));
        counter += 1;
    }

    let new_set_text = format_button_text(text("+")).size(30).center();

    let new_set_text_centered = container(new_set_text).center_x(Fill);

    let new_set_button: Element<Message> = create_element_button(
        &user_interface.app.mascot_manager.selected_mascot,
        new_set_text_centered.into(),
        ButtonStyle::ActiveTab,
        None,
    )
    .width(Fill)
    .on_press(Message::WorkoutCreation(WorkoutCreationMessage::AddSet))
    .into();

    sets_column = sets_column.push(new_set_button);

    let column = Column::new().push(top_row).push(sets_column).spacing(20);

    let exercise_container = container(column)
        .style(create_container_style(ContainerStyle::Light, None, None))
        .height(Shrink)
        .padding(20);

    exercise_container.into()
}

pub fn view_exercise_no_edit<'a>(
    exercise: &'a ExerciseCreate,
    exercise_number: ExerciseNumber,
    user_interface: &'a UserInterface,
) -> Element<'a, Message> {
    let edit_button: Element<Message> = container(
        create_element_button(
            &user_interface.app.mascot_manager.selected_mascot,
            image(Handle::from_path("assets/images/edit.png")).into(),
            ButtonStyle::InactiveTransparent,
            None,
        )
        .on_press(Message::WorkoutCreation(
            WorkoutCreationMessage::BeginExerciseEdit(exercise_number),
        )),
    )
    .width(FillPortion(1))
    .into();

    let mut exercise_name: Element<Message> = Column::new().into();

    if let Some(workout) = &user_interface.app.exercise_manager.workout_in_creation {
        exercise_name = view_exercise_name(workout, exercise_number);
    }

    let delete_button: Element<Message> = view_delete_button(exercise_number, user_interface);

    let top_row = Row::new()
        .push(edit_button)
        .push(exercise_name)
        .push(delete_button)
        .spacing(30);

    let mut counter: SetNumber = 1;
    let mut sets_column = Column::new().spacing(10).width(Fill);

    let descriptions: Element<Message> = view_descriptions(user_interface);

    sets_column = sets_column.push(descriptions);

    for set in &exercise.sets {
        sets_column = sets_column.push(view_set_no_edit(set, counter));
        counter += 1;
    }

    let column = Column::new().push(top_row).push(sets_column).spacing(20);

    let exercise_container = container(column)
        .style(create_container_style(ContainerStyle::Light, None, None))
        .height(Shrink)
        .padding(20);

    exercise_container.into()
}

pub fn view_descriptions(_user_interface: &UserInterface) -> Element<Message> {
    Row::new()
        .push(
            format_description_text(text("SETS"))
                .center()
                .width(FillPortion(1)),
        )
        .push(
            format_description_text(text("KG"))
                .center()
                .width(FillPortion(1)),
        )
        .push(
            format_description_text(text("REPS"))
                .center()
                .width(FillPortion(1)),
        )
        .spacing(10)
        .into()
}

pub fn view_exercise_name(
    workout: &[ExerciseCreate],
    exercise_number: ExerciseNumber,
) -> Element<Message> {
    format_button_text(text(workout[exercise_number - 1].name.clone()))
        .size(30)
        .width(FillPortion(3))
        .into()
}

pub fn view_delete_button(
    exercise_number: ExerciseNumber,
    user_interface: &UserInterface,
) -> Element<Message> {
    container(row![
        Space::with_width(Fill),
        create_element_button(
            &user_interface.app.mascot_manager.selected_mascot,
            image(Handle::from_path("assets/images/trash_red.png")).into(),
            ButtonStyle::InactiveTransparent,
            None,
        )
        .width(Shrink)
        .on_press(Message::WorkoutCreation(
            WorkoutCreationMessage::DeleteExercise(exercise_number)
        ))
    ])
    .width(FillPortion(1))
    .into()
}

pub fn view_set_no_edit(set: &StrengthSet, number: SetNumber) -> Element<Message> {
    let set_number: Element<Message> = container(format_button_text(text(number.to_string())))
        .width(FillPortion(1))
        .center(Fill)
        .into();

    let kg: Element<Message> = container(format_button_text(text(set.weight.to_string())))
        .width(FillPortion(1))
        .center(Fill)
        .into();

    let reps: Element<Message> = container(format_button_text(text(set.reps.to_string())))
        .width(FillPortion(1))
        .center(Fill)
        .into();

    let set_row: Element<Message> = Row::new()
        .push(set_number)
        .push(kg)
        .push(reps)
        .spacing(10)
        .height(30)
        .into();

    container(set_row)
        .style(create_container_style(
            ContainerStyle::Highlighted,
            None,
            None,
        ))
        .height(Shrink)
        .into()
}

pub fn view_set_edit(number: SetNumber, user_interface: &UserInterface) -> Element<Message> {
    let set_number: Element<Message> = container(format_button_text(text(number.to_string())))
        .center(FillPortion(1))
        .into();

    let mut kg: Element<Message> = Column::new().into();
    let mut reps: Element<Message> = Column::new().into();
    if let Some(exercise_string) = &user_interface.app.exercise_manager.exercise_in_edit_strings {
        kg = container(
            text_input("Enter weight...", &exercise_string.sets[number - 1].kg)
                .style(create_text_input_style(
                    &user_interface.app.mascot_manager.selected_mascot,
                ))
                .font(FIRA_SANS_EXTRABOLD)
                .on_input(move |new_kg| -> Message {
                    Message::WorkoutCreation(WorkoutCreationMessage::EditKg(number, new_kg))
                })
                .align_x(Alignment::Center)
                .width(60),
        )
        .center(FillPortion(1))
        .into();
        reps = container(
            text_input("Enter reps...", &exercise_string.sets[number - 1].reps)
                .style(create_text_input_style(
                    &user_interface.app.mascot_manager.selected_mascot,
                ))
                .font(FIRA_SANS_EXTRABOLD)
                .on_input(move |new_reps| -> Message {
                    Message::WorkoutCreation(WorkoutCreationMessage::EditReps(number, new_reps))
                })
                .align_x(Alignment::Center)
                .width(60),
        )
        .center(FillPortion(1))
        .into();
    }

    let delete_button: Element<Message> = container(
        create_element_button(
            &user_interface.app.mascot_manager.selected_mascot,
            image(Handle::from_path("assets/images/trash_black.png")).into(),
            ButtonStyle::InactiveTransparent,
            None,
        )
        .width(Shrink)
        .on_press(Message::WorkoutCreation(WorkoutCreationMessage::DeleteSet(
            number,
        ))),
    )
    .center(FillPortion(1))
    .into();

    let set_row: Element<Message> = stack![
        Row::new()
            .push(set_number)
            .push(kg)
            .push(reps)
            .spacing(10)
            .height(30),
        Row::new()
            .push(Space::with_width(FillPortion(15)))
            .push(delete_button)
    ]
    .into();

    container(set_row)
        .style(create_container_style(
            ContainerStyle::Highlighted,
            None,
            None,
        ))
        .height(Shrink)
        .into()
}

impl UserInterface {
    pub fn workout_creation_screen(&self) -> Element<Message> {
        let mut column = Column::new().spacing(20);

        if let Some(current_workout) = &self.app.exercise_manager.workout_in_creation {
            let mut counter: ExerciseNumber = 1;
            for exercise in current_workout {
                column = column.push(view_exercise(exercise, counter, self));
                counter += 1;
            }
        }

        let exercises = Scrollable::new(column)
            .direction(Direction::Vertical(Scrollbar::new()))
            .height(FillPortion(6));

        let add_exercise: Element<Message> = combo_box(
            &self.app.exercise_manager.owned_exercise_state,
            "Search for exercise...",
            None,
            |exercise: String| -> Message {
                Message::WorkoutCreation(WorkoutCreationMessage::AddExercise(exercise))
            },
        )
        .menu_style(create_menu_style(&self.app.mascot_manager.selected_mascot))
        .input_style(create_text_input_style(
            &self.app.mascot_manager.selected_mascot,
        ))
        .font(FIRA_SANS_EXTRABOLD)
        .line_height(LineHeight::Absolute(Pixels(30.0)))
        .into();

        let finish_workout_text = format_button_text(text("Finish Workout")).size(25).center();

        let check_box_image = image(Handle::from_path("assets/images/check_box.png"));

        let finish_workout_image_and_text =
            container(row![check_box_image, finish_workout_text]).center_x(Fill);

        let finish_workout_button: Element<Message> = create_element_button(
            &self.app.mascot_manager.selected_mascot,
            finish_workout_image_and_text.into(),
            ButtonStyle::Active,
            None,
        )
        .width(Fill)
        .on_press(Message::WorkoutCreation(
            WorkoutCreationMessage::FinishWorkoutCreation,
        ))
        .into();

        let add_exercise_and_finish: Column<Message> = Column::new()
            .push(add_exercise)
            .push(finish_workout_button)
            .spacing(10)
            .height(FillPortion(1))
            .width(Fill);

        let exercises_and_bottom_stuff = Column::new()
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
