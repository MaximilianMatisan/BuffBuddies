use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::Row;

impl UserInterface {
    pub fn settings_screen(&self) -> Element<Message> {
        Row::new().into()
    }
}
