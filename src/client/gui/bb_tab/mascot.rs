use iced::Element;
use iced::widget::Row;
use crate::client::gui::user_interface::{Message, UserInterface};

impl UserInterface {

    pub fn mascot_screen(&self) -> Element<Message> {
        Row::new().into()
    }
}
