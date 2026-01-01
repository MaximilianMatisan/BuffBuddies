use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{format_button_text, format_description_text};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{Button, Column, Space, image, text};
use iced_core::Length;
use iced_core::alignment::Horizontal;
use iced_core::image::Handle;

const FRIEND_BUTTON_WIDTH: f32 = 200.0;
const FRIEND_BUTTON_HEIGHT: f32 = 300.0;
const PROFILE_PICTURE_DIMENSION: f32 = 75.0;
const MAX_DISPLAYED_NAME_CHARS: usize = 8;

pub fn friend_button<'a>(
    app: &App,
    profile_picture_handle: String,
    username: String,
    week_streak: u32,
    favorite_mascot: &Mascot,
) -> Button<'a, Message> {
    let profile_picture: Element<Message> = image(Handle::from_path(profile_picture_handle))
        .width(PROFILE_PICTURE_DIMENSION)
        .height(PROFILE_PICTURE_DIMENSION)
        .into();

    let cropped_username: String = username.chars().take(MAX_DISPLAYED_NAME_CHARS).collect();
    let name = format_button_text(text(cropped_username)).size(24);
    let streak = format_description_text(text(format!("{week_streak}-week-streak")));

    let mascot_handle = app
        .image_manager
        .cropped_mascot_image_handles
        .get(favorite_mascot)
        .cloned() //TODO unschön hier?
        .unwrap_or_else(|| Handle::from_path("assets/images/default_preset.png"));

    let mascot_image = image(mascot_handle).width(FRIEND_BUTTON_WIDTH - 2.0 * INDENT);

    let contents = Column::new()
        .push(Space::with_height(Length::FillPortion(1)))
        .push(profile_picture)
        .push(name)
        .push(streak)
        .push(Space::with_height(Length::FillPortion(2)))
        .push(mascot_image)
        .align_x(Horizontal::Center);

    let button = create_element_button(
        *favorite_mascot,
        contents.into(),
        ButtonStyle::InactiveTab,
        Some(DEFAULT_CONTAINER_RADIUS.into()),
    )
    .width(FRIEND_BUTTON_WIDTH)
    .height(FRIEND_BUTTON_HEIGHT);

    button
}
