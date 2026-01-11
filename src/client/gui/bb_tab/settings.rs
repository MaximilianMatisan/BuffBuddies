use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::{image};
use iced_core::image::Handle;
use crate::client::gui::size;

impl UserInterface {
    pub fn settings_screen(&self) -> Element<Message> {
        let profile_picture = image(Handle::from_path(self.app.user_manager.user_information.profile_picture_handle.clone()))
            .width(size::LARGE_PROFILE_PICTURE_DIMENSION)
            .height(size::LARGE_PROFILE_PICTURE_DIMENSION);

        profile_picture.into()
    }
}
