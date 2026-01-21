use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_widget::general_exercise_info_elements::display_general_exercise_infos;
use crate::client::gui::bb_widget::widget_utils::LARGE_INDENT;
use crate::client::gui::bb_widget::{new_widget, workout};
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::{column, container, row};

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

        let general_exercise_info_elements = display_general_exercise_infos(
            &self.app.mascot_manager.selected_mascot,
            &self.app.exercise_manager,
        );
        let exercise_info_container = container(general_exercise_info_elements)
            .style(create_container_style(ContainerStyle::Default, None, None))
            .padding(LARGE_INDENT);

        let content = column![
            recent_workout_row,
            workout_preset_row,
            exercise_info_container
        ]
        .padding(30)
        .spacing(30);

        content.into()
    }
}
