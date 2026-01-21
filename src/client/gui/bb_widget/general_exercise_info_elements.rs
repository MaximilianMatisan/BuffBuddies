use crate::client::backend::exercise::exercise_manager::ExerciseManager;
use crate::client::backend::exercise::general_exercise::GeneralExerciseInfo;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::user_interface::Message;
use iced::widget::{Column, Container, Row, Space, container, text};
use iced_core::Length;
use iced_core::alignment::Vertical;

pub fn display_general_exercise_infos<'a>(
    active_mascot: &Mascot,
    exercise_manager: &ExerciseManager,
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
    general_exercise_info: &GeneralExerciseInfo,
    show_extended: bool,
) -> Container<'a, Message> {
    let exercise_name = text(general_exercise_info.name.clone())
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

    let content = Column::new().push(header);

    container(content)
        .padding(3)
        .style(create_container_style(ContainerStyle::Light, None, None))
}
