use crate::client::gui::bb_widget::{new_widget, workout};
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::{column, row};

impl UserInterface {
    pub fn workout_screen(&self) -> Element<Message> {
        let recent_workout_row: Element<Message> = row![
            new_widget::new_workout_widget_button(),
            workout::WorkoutWidget::default_recent_workout_widget()
        ]
        .spacing(30)
        .into();

        let workout_preset_row: Element<Message> = row![
            new_widget::new_preset_widget_button(),
            workout::WorkoutWidget::default_workout_preset_widget()
        ]
        .spacing(30)
        .into();

        let recent_workouts = column![recent_workout_row, workout_preset_row]
            .padding(30)
            .spacing(30);

        recent_workouts.into()
    }
}
