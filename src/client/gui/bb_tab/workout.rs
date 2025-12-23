use iced::Element;
use iced::widget::column;
use crate::client::gui::user_interface::{Message, UserInterface};
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::new_widget;
use crate::client::gui::user_interface::Message::Select;

impl UserInterface {

    pub fn workout_screen(&self) -> Element<Message> {
        let recent_workouts = column![
            new_widget::NewWidget::default_new_workout_widget().on_press(Select(Tab::Home)),
            new_widget::NewWidget::default_new_preset_widget().on_press(Select(Tab::Home))
        ].padding(30).spacing(30);
        recent_workouts.into()
    }
}