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
use crate::client::backend::user_mod::user::User;
use crate::client::gui::size::{LARGE_PROFILE_PICTURE_DIMENSION, MEDIUM_PROFILE_PICTURE_DIMENSION};

const FRIEND_BUTTON_WIDTH: f32 = 200.0;
const FRIEND_BUTTON_HEIGHT: f32 = 300.0;
const USER_BUTTON_WIDTH: f32 = 700.0;
const MAX_DISPLAYED_NAME_CHARS: usize = 8;

pub fn friend_button<'a>(
    app: &App,
    user: &User
) -> Button<'a, Message> {
    let profile_picture: Element<Message> = image(Handle::from_path(user.profile_picture_handle.clone()))
        .width(LARGE_PROFILE_PICTURE_DIMENSION)
        .height(LARGE_PROFILE_PICTURE_DIMENSION)
        .into();

    let cropped_username: String = user.username.chars().take(MAX_DISPLAYED_NAME_CHARS).collect();
    let name = format_button_text(text(cropped_username)).size(24);
    let streak = format_description_text(text(format!("{}-week-streak", user.weekly_workout_streak)));

    let mascot_handle = app
        .image_manager
        .cropped_mascot_image_handles
        .get(&user.favorite_mascot)
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
        user.favorite_mascot,
        contents.into(),
        ButtonStyle::InactiveTab,
        Some(DEFAULT_CONTAINER_RADIUS.into()),
    )
    .width(FRIEND_BUTTON_WIDTH)
    .height(FRIEND_BUTTON_HEIGHT)
    .on_press(Message::ViewProfile(user.username.clone()));

    button
}
pub fn user_profile_button<'a>(active_mascot: &Mascot, user: &User) -> Button<'a,Message>{

    let profile_picture: Element<Message> = image(Handle::from_path(user.profile_picture_handle.clone()))
        .width(MEDIUM_PROFILE_PICTURE_DIMENSION)
        .height(MEDIUM_PROFILE_PICTURE_DIMENSION)
        .into();

    let name = format_button_text(text(user.username.clone()));
    let streak = format_description_text(text(format!("{}-week-streak", user.weekly_workout_streak)));

    let text_column = Column::new().push(name).push(streak);

    let add_friend_button =
        create_element_button(*active_mascot, image("assets/images/user_plus.png").into(), ButtonStyle::Active, Some(DEFAULT_CONTAINER_RADIUS.into())).on_press(Message::AddUserAsFriend(user.username.clone()));

    let contents = Row::new()
        .push(profile_picture)
        .push(Space::with_width(50))
        .push(text_column)
        .push(Space::with_width(Length::Fill))
        .push(add_friend_button)
        .align_y(Vertical::Center);

    let user_profile_button =
        create_element_button(*active_mascot, contents.into(),ButtonStyle::InactiveTab, Some(DEFAULT_CONTAINER_RADIUS.into()))
            .width(USER_BUTTON_WIDTH)
            .height(Length::Shrink).on_press(Message::ViewProfile(user.username.clone()));

    user_profile_button
}
