use iced::Element;
use crate::client::backend::exercise::exercise_manager::ExerciseManager;
use crate::client::backend::exercise::general_exercise::GeneralExerciseInfo;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style, DEFAULT_TEXT_CONTAINER_PADDING};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{format_description_text, FIRA_SANS_EXTRABOLD};
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::user_interface::Message;
use iced::widget::{Column, Container, Row, Space, container, text};
use iced_core::Length;
use iced_core::alignment::Vertical;

pub fn display_general_exercise_infos<'a>(
    active_mascot: &Mascot,
    exercise_manager: &'a ExerciseManager,
) -> Column<'a, Message> {
    let mut content = Column::new().spacing(INDENT);
    for exercise_info in &exercise_manager.general_exercise_info {
        let show_extended_info = exercise_manager
            .extended_general_exercise_infos
            .contains(&exercise_info.id);
        content = content.push(general_exercise_info_element(
            active_mascot,
            exercise_info,
            show_extended_info,
        ));
    }
    content
}
fn general_exercise_info_element<'a>(
    active_mascot: &Mascot,
    general_exercise_info: &'a GeneralExerciseInfo,
    show_extended: bool,
) -> Container<'a, Message> {
    let exercise_name = text(&general_exercise_info.name)
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR);

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
    .on_press(Message::ToggleGeneralExerciseInfo(general_exercise_info.id));

    let header = Row::new()
        .push(Space::with_width(LARGE_INDENT))
        .push(exercise_name)
        .push(Space::with_width(Length::Fill))
        .push(toggle_more_info_button)
        .align_y(Vertical::Center);

    let mut content = Column::new()
        .push(header);

    if show_extended {
        content = content.push(create_further_infos(general_exercise_info));
    }
    container(content)
        .padding(3)
        .style(create_container_style(ContainerStyle::Light, None, None))
}
fn create_further_infos(general_exercise_info: &GeneralExerciseInfo) -> Row<Message> {
    let mut content = Row::new().height(Length::Shrink);

    let instruction_title = format_description_text(text("Instruction:"));
    let instruction_text = text(&general_exercise_info.instructions)
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR)
        .size(14);

    let instruction_column = Column::new()
        .push(instruction_title)
        .push(instruction_text);

    let instruction_container = container(instruction_column)
        .style(create_container_style(ContainerStyle::Background, None, None))
        .padding(DEFAULT_TEXT_CONTAINER_PADDING)
        .width(Length::Fill);

    let detail_column = Column::new()
        .width(Length::Fill);

    content = content.push(instruction_container);
    content = content.push(detail_column);

    content
}
