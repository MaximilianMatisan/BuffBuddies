use iced::Element;
use iced::widget::column;
use crate::client::gui::user_interface::{Message, UserInterface};
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::new_widget;
use crate::client::gui::user_interface::Message::Select;

impl UserInterface {

    pub fn workout_screen(&self) -> Element<Message> {
        let recent_workouts = column![
            new_widget::new_workout_widget_button(),
            new_widget::new_preset_widget_button()
        ].padding(30).spacing(30);
        recent_workouts.into()
    }
}