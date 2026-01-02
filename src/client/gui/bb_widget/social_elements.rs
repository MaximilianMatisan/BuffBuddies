use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{format_button_text, format_description_text};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{Button, Column, Space, image, text, Row};
use iced_core::Length;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::image::Handle;

const FRIEND_BUTTON_WIDTH: f32 = 200.0;
const FRIEND_BUTTON_HEIGHT: f32 = 300.0;
const USER_BUTTON_WIDTH: f32 = 700.0;
const LARGE_PROFILE_PICTURE_DIMENSION: f32 = 75.0;
const MEDIUM_PROFILE_PICTURE_DIMENSION: f32 = 50.0;
const MAX_DISPLAYED_NAME_CHARS: usize = 8;

pub fn friend_button<'a>(
    app: &App,
    profile_picture_handle: &str,
    username: String,
    week_streak: u32,
    favorite_mascot: &Mascot,
) -> Button<'a, Message> {
    let profile_picture: Element<Message> = image(Handle::from_path(profile_picture_handle))
        .width(LARGE_PROFILE_PICTURE_DIMENSION)
        .height(LARGE_PROFILE_PICTURE_DIMENSION)
        .into();

    let cropped_username: String = username.chars().take(MAX_DISPLAYED_NAME_CHARS).collect();
    let name = format_button_text(text(cropped_username)).size(24);
    let streak = format_description_text(text(format!("{week_streak}-week-streak")));

    let mascot_handle = app
        .image_manager
        .cropped_mascot_image_handles
        .get(favorite_mascot)
        .unwrap();

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
    .height(FRIEND_BUTTON_HEIGHT)
    .on_press(Message::ViewProfile(username));

    button
}
pub fn user_profile_button<'a>(active_mascot: &Mascot ,profile_picture_handle: &str, username: String, week_streak: u32) -> Button<'a,Message>{

    let profile_picture: Element<Message> = image(Handle::from_path(profile_picture_handle))
        .width(MEDIUM_PROFILE_PICTURE_DIMENSION)
        .height(MEDIUM_PROFILE_PICTURE_DIMENSION)
        .into();

    let name = format_button_text(text(username.clone()));
    let streak = format_description_text(text(format!("{week_streak}-week-streak")));

    let text_column = Column::new().push(name).push(streak);

    let add_friend_button =
        create_element_button(*active_mascot, image("assets/images/user_plus.png").into(), ButtonStyle::Active, Some(DEFAULT_CONTAINER_RADIUS.into())).on_press(Message::AddUserAsFriend(username.clone()));

    let contents = Row::new()
        .push(Space::with_width(50))
        .push(profile_picture)
        .push(Space::with_width(50))
        .push(text_column)
        .push(Space::with_width(Length::Fill))
        .push(add_friend_button)
        .push(Space::with_width(50))
        .align_y(Vertical::Center);

    let user_profile_button =
        create_element_button(*active_mascot, contents.into(),ButtonStyle::InactiveTab, Some(DEFAULT_CONTAINER_RADIUS.into()))
            .width(USER_BUTTON_WIDTH)
            .height(Length::Shrink).on_press(Message::ViewProfile(username));

    user_profile_button
}
