use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::{image, Row};

impl UserInterface {
    pub fn mascot_screen(&self) -> Element<Message> {
        
        let mut row: Row<Message> = Row::new();
        for (_, handle) in self.app.image_manager.cropped_mascot_image_handles.iter() {
            let image: Element<Message> = image(handle).width(200).into();
            row = row.push(image);
        }

        row.into()
    }
}
