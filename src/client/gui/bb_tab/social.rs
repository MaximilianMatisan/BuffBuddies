use iced::Element;
use iced::widget::Row;
use crate::{Message, UserInterface};

impl UserInterface {

    pub fn social_screen(&self) -> Element<Message> {
        Row::new().into()
    }
}