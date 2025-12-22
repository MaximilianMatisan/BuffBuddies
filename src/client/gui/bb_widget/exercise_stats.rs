use iced::widget::{container, text, Column, Row, Space};
use iced::{Length};
use iced_core::border::Radius;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::{create_style_container, ContainerStyle, DEFAULT_CONTAINER_RADIUS};
use crate::client::gui::bb_theme::custom_button::DEFAULT_BUTTON_RADIUS;
use crate::client::gui::bb_theme::text_format::{format_button_text, kg_to_string};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::Message;

const DISPLAYED_EXERCISE_STATS: usize = 5;
const DISPLAYED_EXERCISE_STAT_LAST_INDEX: usize = DISPLAYED_EXERCISE_STATS - 1;
pub fn exercise_stat_column(app: &App) -> Column<Message> {
    let mut lines: Column<Message> = Column::new();
    let stats: [(String, String); DISPLAYED_EXERCISE_STATS] = [
        ("Total lifted weight: ".to_string(), kg_to_string(app.exercise_manager.all_time_lifted_weight)),
        ("Total sets done: ".to_string(), app.exercise_manager.all_time_sets.to_string()),
        ("Total reps done: ".to_string(), app.exercise_manager.all_time_reps.to_string()),
        ("Weight record: ".to_string(), kg_to_string(app.exercise_manager.weight_personal_record)),
        ("Set with most total lifted weight: ".to_string(), format!("{} - {}",
            app.exercise_manager.set_with_most_total_lifted_weight.0.format("%d.%m.%y"),
            kg_to_string(app.exercise_manager.set_with_most_total_lifted_weight.1)))
    ];
    for (i, (title, data)) in stats.iter().enumerate() {
        let line = Row::new()
            .push(format_button_text(text(title.clone())))
            .push(Space::with_width(Length::Fill))
            .push(format_button_text(text(data.clone())));

        let container_style = if i % 2 == 0
            {ContainerStyle::Dark} else {ContainerStyle::Highlighted};
        let container_border = match i {
            0 => Some(
                Radius {
                    top_left: DEFAULT_CONTAINER_RADIUS,
                    top_right: DEFAULT_CONTAINER_RADIUS,
                    bottom_right: 0.0,
                    bottom_left: 0.0
                }),
            DISPLAYED_EXERCISE_STAT_LAST_INDEX => Some(
                Radius{
                    top_left:0.0,
                    top_right: 0.0,
                    bottom_left: DEFAULT_CONTAINER_RADIUS,
                    bottom_right: DEFAULT_BUTTON_RADIUS
                }),
            _ => Some(0.0.into())
        };
        lines = lines.push(
            container(line)
                .padding(INDENT)
                .style(create_style_container(container_style, container_border))
        );
    }
    lines
}