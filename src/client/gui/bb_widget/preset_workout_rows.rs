use iced::Element;
use iced::widget::Row;
use crate::client::backend::recent_workouts::RecentWorkoutVisualization;
use crate::client::gui::bb_theme::scrollable::{create_scrollable, ScrollableExtension, ScrollableStyle, SCROLLBAR_PADDING, WIDGET_SCROLLBAR_WIDTH};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::bb_widget::workout::{WorkoutWidget, DEFAULT_RECENT_WORKOUT_WIDGET_HEIGHT};
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;

pub fn view_recent_workout_row<'a>(mascot: &Mascot, recent_workouts: &'a Vec<RecentWorkoutVisualization>) -> Element<'a, Message> {
    let mut recent_workout_row = Row::new()
        .height(DEFAULT_RECENT_WORKOUT_WIDGET_HEIGHT + SCROLLBAR_PADDING)
        .spacing(INDENT);

    for recent_workout in recent_workouts {
        recent_workout_row =
            recent_workout_row.push(WorkoutWidget::new_recent_workout_widget(recent_workout));
    }

    create_scrollable(recent_workout_row, *mascot, ScrollableStyle::Mascot)
        .add_horizontal_scrollbar(WIDGET_SCROLLBAR_WIDTH, 0.0)
        .into()
}
//TODO move view_preset_row here
