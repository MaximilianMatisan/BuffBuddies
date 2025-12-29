use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::Row;

impl UserInterface {
    pub fn mascot_screen(&self) -> Element<Message> {
        Row::new().into()
    }
}
