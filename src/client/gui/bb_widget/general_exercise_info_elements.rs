use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::{
    ContainerStyle, DEFAULT_TEXT_CONTAINER_PADDING, create_container_style,
};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_description_text};
use crate::client::gui::bb_widget::widget_utils::{
    INDENT, LARGE_INDENT, descriptor_space_fill_text_row,
};
use crate::client::gui::user_interface::Message;
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::general_exercise::GeneralExerciseInfo;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::widget::{Column, Container, Row, Space, container, text};
use iced_core::alignment::Vertical;
use iced_core::{Length, Padding};

pub fn display_general_exercise_infos<'a>(
    active_mascot: &Mascot,
    exercise_manager: &'a ExerciseManager,
) -> Column<'a, Message> {
    let mut content = Column::new().spacing(5);
    for exercise in &exercise_manager.exercises {
        let show_extended_info = exercise_manager
            .extended_general_exercise_infos
            .contains(&exercise.general_exercise_info.id);
        content = content.push(general_exercise_info_element(
            active_mascot,
            exercise,
            show_extended_info,
        ));
    }
    content
}
fn general_exercise_info_element<'a>(
    active_mascot: &Mascot,
    exercise: &'a Exercise,
    show_extended: bool,
) -> Container<'a, Message> {
    let title_color = if exercise.is_tracked() {
        active_mascot.get_primary_color()
    } else {
        TEXT_COLOR
    };
    let exercise_name = text(&exercise.general_exercise_info.name)
        .font(FIRA_SANS_EXTRABOLD)
        .color(title_color)
        .size(20);

    let button_symbol = if show_extended { "^" } else { "v" };
    let accordion = text(button_symbol)
        .size(14)
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR);

    let toggle_more_info_button = create_element_button(
        active_mascot,
        accordion.into(),
        ButtonStyle::ActiveTab,
        None,
    )
    .on_press(Message::ToggleGeneralExerciseInfo(
        exercise.general_exercise_info.id,
    ));

    let header = Row::new()
        .push(exercise_name)
        .push(Space::with_width(Length::Fill))
        .push(toggle_more_info_button)
        .align_y(Vertical::Center);

    let mut content = Column::new().push(header).padding(Padding {
        left: LARGE_INDENT,
        ..Default::default()
    });

    if show_extended {
        content = content.push(create_extended_infos(&exercise.general_exercise_info));
    }
    container(content)
        .padding(3)
        .style(create_container_style(ContainerStyle::Light, None, None))
}
fn create_extended_infos(general_exercise_info: &GeneralExerciseInfo) -> Row<Message> {
    let content = Row::new()
        .push(instruction_container(&general_exercise_info.instructions))
        .push(detail_column(general_exercise_info))
        .spacing(INDENT)
        .padding(Padding {
            right: LARGE_INDENT,
            bottom: INDENT / 2.0,
            ..Default::default()
        });

    content
}

fn instruction_container(instructions: &String) -> Container<Message> {
    let instruction_title = format_description_text(text("Instruction:"));
    let instruction_text = text(instructions)
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR)
        .size(14);

    let instruction_column = Column::new().push(instruction_title).push(instruction_text);

    let instruction_container = container(instruction_column)
        .style(create_container_style(ContainerStyle::Default, None, None))
        .padding(DEFAULT_TEXT_CONTAINER_PADDING)
        .width(Length::FillPortion(7));

    instruction_container
}
fn detail_column(general_exercise_info: &GeneralExerciseInfo) -> Column<Message> {
    let mut content = Column::new().width(Length::FillPortion(5));

    let fields = [
        descriptor_space_fill_text_row("Force:", general_exercise_info.force.to_string()),
        descriptor_space_fill_text_row("Level:", general_exercise_info.level.to_string()),
        descriptor_space_fill_text_row("Equipment:", general_exercise_info.equipment.to_string()),
        descriptor_space_fill_text_row(
            "Primary muscle:",
            general_exercise_info.primary_muscle.to_string(),
        ),
        descriptor_space_fill_text_row("Category:", general_exercise_info.category.to_string()),
    ];
    for field in fields {
        content = content.push(field);
    }
    content
}
