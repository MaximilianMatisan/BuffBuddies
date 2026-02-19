use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_description_text};
use crate::client::gui::bb_widget::general_exercise_info_elements::display_general_exercise_infos;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::bb_widget::{circle_widget, new_widget, workout};
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, column, container, row, text};
use iced_core::alignment::Vertical;

impl App {
    pub fn workout_screen(&self) -> Element<Message> {
        let recent_workout_row: Element<Message> = row![
            new_widget::new_workout_widget_button(),
            workout::WorkoutWidget::default_recent_workout_widget()
        ]
        .spacing(LARGE_INDENT)
        .into();

        let mut workout_preset_row = Row::new()
            .push(new_widget::new_preset_widget_button())
            .spacing(LARGE_INDENT);

        for preset in &self.workout_preset_manager.presets {
            workout_preset_row =
                workout_preset_row.push(workout::WorkoutWidget::new_workout_preset_widget(preset));
        }

        let workout_preset_scrollable =
            Scrollable::new(workout_preset_row).direction(Direction::Horizontal(Scrollbar::new()));

        let general_exercise_info_elements = display_general_exercise_infos(
            &self.mascot_manager.selected_mascot,
            &self.exercise_manager,
        );
        let browse_exercises_title = text("Browse exercises")
            .size(30)
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR);

        let title_bar = Row::new()
            .push(browse_exercises_title)
            .push(format_description_text(text(format!(
                " - {} results",
                self.exercise_manager.exercises.len()
            ))))
            .align_y(Vertical::Center);

        let exercise_browser = Column::new()
            .push(title_bar)
            .push(general_exercise_info_elements)
            .spacing(INDENT);

        let exercise_info_container = container(exercise_browser)
            .style(create_container_style(ContainerStyle::Default, None, None))
            .padding(LARGE_INDENT);

        let circle_widget = circle_widget::CircleWidget::new(self).view();

        let content = column![
            circle_widget,
            recent_workout_row,
            workout_preset_scrollable,
            exercise_info_container,
        ]
        .padding(LARGE_INDENT)
        .spacing(LARGE_INDENT);

        Scrollable::new(content)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}
