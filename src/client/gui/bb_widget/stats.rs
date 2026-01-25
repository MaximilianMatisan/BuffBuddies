use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::{
    ContainerStyle, DEFAULT_CONTAINER_RADIUS, create_container_style,
};
use crate::client::gui::bb_theme::custom_button::DEFAULT_BUTTON_RADIUS;
use crate::client::gui::bb_theme::text_format::{format_button_text, kg_to_string};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use iced::Length;
use iced::widget::{Column, Container, Row, Space, container, image, text};
use iced_core::alignment::Horizontal;
use iced_core::border::Radius;
use iced_core::image::Handle;

const DISPLAYED_EXERCISE_STATS: usize = 5;
const DISPLAYED_EXERCISE_STAT_LAST_INDEX: usize = DISPLAYED_EXERCISE_STATS - 1;
pub fn exercise_stat_column(app: &App) -> Column<Message> {
    let mut lines: Column<Message> = Column::new();
    let stats: [(String, String); DISPLAYED_EXERCISE_STATS] = [
        (
            "Total lifted weight: ".to_string(),
            kg_to_string(app.exercise_manager.all_time_lifted_weight),
        ),
        (
            "Total sets done: ".to_string(),
            app.exercise_manager.all_time_sets.to_string(),
        ),
        (
            "Total reps done: ".to_string(),
            app.exercise_manager.all_time_reps.to_string(),
        ),
        (
            "Weight record: ".to_string(),
            kg_to_string(app.exercise_manager.weight_personal_record),
        ),
        (
            "Set with most total lifted weight: ".to_string(),
            format!(
                "{} - {}",
                app.exercise_manager
                    .set_with_most_total_lifted_weight
                    .0
                    .format("%d.%m.%y"),
                kg_to_string(app.exercise_manager.set_with_most_total_lifted_weight.1)
            ),
        ),
    ];
    for (i, (title, data)) in stats.iter().enumerate() {
        let line = Row::new()
            .push(format_button_text(text(title.clone())))
            .push(Space::with_width(Length::Fill))
            .push(format_button_text(text(data.clone())));

        let container_style = if i % 2 == 0 {
            ContainerStyle::Light
        } else {
            ContainerStyle::Highlighted
        };
        let container_border = match i {
            0 => Some(Radius {
                top_left: DEFAULT_CONTAINER_RADIUS,
                top_right: DEFAULT_CONTAINER_RADIUS,
                bottom_right: 0.0,
                bottom_left: 0.0,
            }),
            DISPLAYED_EXERCISE_STAT_LAST_INDEX => Some(Radius {
                top_left: 0.0,
                top_right: 0.0,
                bottom_left: DEFAULT_CONTAINER_RADIUS,
                bottom_right: DEFAULT_BUTTON_RADIUS,
            }),
            _ => Some(0.0.into()),
        };
        lines = lines.push(
            container(line)
                .padding(INDENT)
                .style(create_container_style(
                    container_style,
                    container_border,
                    None,
                )),
        );
    }
    lines
}

const PROFILE_STAT_CONTAINER_WIDTH: f32 = 165.0;
pub const PROFILE_STAT_CONTAINER_HEIGHT: f32 = 180.0;

pub fn profile_stat_container<'a>(
    image_handle: Handle,
    value: String,
    description_line_one: &'a str,
    description_line_two: &'a str,
) -> Container<'a, Message> {
    let image = image(image_handle).height(PROFILE_STAT_CONTAINER_HEIGHT / 2.0);

    let value_text_element = format_button_text(text(value)).size(20);
    let description_one_text_element = format_button_text(text(description_line_one));
    let description_two_text_element = format_button_text(text(description_line_two));

    let contents = Column::new()
        .push(image)
        .push(value_text_element)
        .push(description_one_text_element)
        .push(description_two_text_element)
        .align_x(Horizontal::Center);

    container(contents)
        .style(create_container_style(ContainerStyle::Default, None, None))
        .center_x(Length::Fixed(PROFILE_STAT_CONTAINER_WIDTH))
        .center_y(Length::Fixed(PROFILE_STAT_CONTAINER_HEIGHT))
}
