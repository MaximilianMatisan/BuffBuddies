use crate::client::gui::bb_tab::settings::SettingsMessage;
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::common::mascot_mod::mascot::Mascot;
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Row, Scrollable, image};
use iced_core::image::Handle;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

pub const SMALL_PROFILE_PICTURE_DIMENSION: f32 = 50.0;
pub const MEDIUM_PROFILE_PICTURE_DIMENSION: f32 = 75.0;
pub const LARGE_PROFILE_PICTURE_DIMENSION: f32 = 150.0;
pub const PROFILE_PICTURE_FOLDER_PATH: &str = "assets/images/profile_pictures/";
pub const PROFILE_PICTURE_FILE_TYPE_SUFFIX: &str = ".png";

/// Types of preset profile pictures which the user can choose from
#[derive(EnumIter, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ProfilePictureTypes {
    ManBuff,
    ManBike,
    ManCoat,
    WomanSpeaker,
    WomanTrain,
    WomanSpray,
    Dog,
}

impl ProfilePictureTypes {
    pub fn get_image_path(&self) -> String {
        let image_path = format!(
            "{}{}{}",
            PROFILE_PICTURE_FOLDER_PATH, self, PROFILE_PICTURE_FILE_TYPE_SUFFIX
        );

        image_path
    }
}
/// Scrollable row of iced images wrapped in buttons to select the users new profile picture
/// Only used if the user has pending UserInfo changes
pub fn profile_picture_selection_row<'a>(mascot: &Mascot) -> Element<'a, SettingsMessage> {
    let mut image_row = Row::new();

    for profile_picture_type in ProfilePictureTypes::iter() {
        let image_path = profile_picture_type.get_image_path();

        let image = image(Handle::from_path(image_path.clone()))
            .width(SMALL_PROFILE_PICTURE_DIMENSION)
            .height(SMALL_PROFILE_PICTURE_DIMENSION);

        let image_button =
            create_element_button(mascot, image.into(), ButtonStyle::InactiveTransparent, None)
                .on_press(SettingsMessage::SelectProfilePicture(image_path));

        image_row = image_row.push(image_button);
    }
    Scrollable::new(image_row)
        .direction(Direction::Horizontal(Scrollbar::new()))
        .into()
}
