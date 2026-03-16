use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::{
    ContainerStyle, DEFAULT_CONTAINER_RADIUS, create_container_style,
};
use crate::client::gui::bb_theme::custom_button::DEFAULT_BUTTON_RADIUS;
use crate::client::gui::bb_theme::text_format::{
    format_button_text, format_description_text, kg_to_string,
};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use iced::Length;
use iced::widget::{Column, Container, Row, Space, container, image, text};
use iced_core::Padding;
use iced_core::alignment::Horizontal;
use iced_core::border::Radius;
use iced_core::image::Handle;

const DISPLAYED_EXERCISE_STATS: usize = 5;
const DISPLAYED_EXERCISE_STAT_LAST_INDEX: usize = DISPLAYED_EXERCISE_STATS - 1;
pub fn exercise_stat_column(app: &App) -> Column<'_, Message> {
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
            .push(Space::new().width(Length::Fill))
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

const PROFILE_STAT_CONTAINER_WIDTH: f32 = 150.0;
pub const PROFILE_STAT_CONTAINER_HEIGHT: f32 = 180.0;

pub fn profile_stat_container<'a>(
    image_handle: Handle,
    value: String,
    description_line_one: &'a str,
    description_line_two: &'a str,
) -> Container<'a, Message> {
    let image = image(image_handle).height(PROFILE_STAT_CONTAINER_HEIGHT / 2.0);

    let font_size_line_one = if description_line_one.len() <= 15 {
        18
    } else {
        10
    };
    let value_text_element = format_button_text(text(value)).size(20);
    let description_one_text_element =
        format_button_text(text(description_line_one).size(font_size_line_one));
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

const HEALTH_STAT_CONTAINER_WIDTH: f32 = 350.0;
pub const HEALTH_STAT_CONTAINER_HEIGHT: f32 = 135.0;
pub const HEALTH_STATS_TEXT_SIZE: u32 = 26;
pub const DESCRIPTION_TEXT_SIZE: u32 = 16;

pub fn health_stat_container<'a>() -> Container<'a, Message> {
    let stats_header = format_button_text(text("Stats")).size(HEALTH_STATS_TEXT_SIZE);
    let all_time_water_text =
        format_description_text(text("All-time-water")).size(DESCRIPTION_TEXT_SIZE);
    let all_time_steps_text =
        format_description_text(text("All-time-steps")).size(DESCRIPTION_TEXT_SIZE);
    let all_time_sleep_text =
        format_description_text(text("All-time-sleep")).size(DESCRIPTION_TEXT_SIZE);

    let water_value = format_button_text(text("coming soon")).size(DESCRIPTION_TEXT_SIZE);
    let steps_value = format_button_text(text("coming soon")).size(DESCRIPTION_TEXT_SIZE);
    let sleep_value = format_button_text(text("coming soon")).size(DESCRIPTION_TEXT_SIZE);

    let headers_column = Column::new()
        .push(stats_header)
        .push(all_time_water_text)
        .push(all_time_steps_text)
        .push(all_time_sleep_text)
        .spacing(5.0)
        .align_x(Horizontal::Left);

    let values_column = Column::new()
        .push(Space::new().height(Length::Fixed(35.0)))
        .push(water_value)
        .push(steps_value)
        .push(sleep_value)
        .spacing(5.0)
        .align_x(Horizontal::Right);

    let contents = Row::new()
        .push(headers_column)
        .push(Space::new().width(Length::Fill))
        .push(values_column)
        .padding(Padding {
            right: INDENT,
            bottom: INDENT,
            ..12.5.into()
        });

    container(contents)
        .style(create_container_style(ContainerStyle::Default, None, None))
        .width(HEALTH_STAT_CONTAINER_WIDTH)
        .height(HEALTH_STAT_CONTAINER_HEIGHT)
}
