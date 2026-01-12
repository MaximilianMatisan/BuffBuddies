use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::user_mod::user::{ForeignUser, UserType};
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{format_button_text, format_description_text};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::size::{MEDIUM_PROFILE_PICTURE_DIMENSION, SMALL_PROFILE_PICTURE_DIMENSION};
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{Button, Column, Row, Space, column, image, text};
use iced_core::Length;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::image::Handle;

const FRIEND_BUTTON_WIDTH: f32 = 200.0;
const FRIEND_BUTTON_HEIGHT: f32 = 300.0;
const USER_BUTTON_WIDTH: f32 = 700.0;
const MAX_DISPLAYED_NAME_CHARS: usize = 8;

pub fn friend_button<'a>(app: &App, user: &ForeignUser) -> Button<'a, Message> {
    let profile_picture: Element<Message> = image(Handle::from_path(
        user.user_information.profile_picture_handle.clone(),
    ))
    .width(MEDIUM_PROFILE_PICTURE_DIMENSION)
    .height(MEDIUM_PROFILE_PICTURE_DIMENSION)
    .into();

    let cropped_username: String = user
        .user_information
        .username
        .chars()
        .take(MAX_DISPLAYED_NAME_CHARS)
        .collect();
    let name = format_button_text(text(cropped_username)).size(24);
    let streak = format_description_text(text(format!(
        "{}-week-streak",
        user.user_information.weekly_workout_streak
    )));

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
        &user.favorite_mascot,
        contents.into(),
        ButtonStyle::InactiveTab,
        Some(DEFAULT_CONTAINER_RADIUS.into()),
    )
    .width(FRIEND_BUTTON_WIDTH)
    .height(FRIEND_BUTTON_HEIGHT)
    .on_press(Message::ViewProfile(UserType::Other(
        user.user_information.username.clone(),
    )));

    button
}
pub fn user_profile_button<'a>(active_mascot: &Mascot, user: &ForeignUser) -> Button<'a, Message> {
    let profile_picture: Element<Message> = image(Handle::from_path(
        user.user_information.profile_picture_handle.clone(),
    ))
    .width(SMALL_PROFILE_PICTURE_DIMENSION)
    .height(SMALL_PROFILE_PICTURE_DIMENSION)
    .into();

    let name = format_button_text(text(user.user_information.username.clone()));
    let streak = format_description_text(text(format!(
        "{}-week-streak",
        user.user_information.weekly_workout_streak
    )));

    let text_column = Column::new().push(name).push(streak);

    let add_friend_button = create_element_button(
        active_mascot,
        image("assets/images/user_plus.png").into(),
        ButtonStyle::Active,
        Some(DEFAULT_CONTAINER_RADIUS.into()),
    )
    .on_press(Message::AddUserAsFriend(
        user.user_information.username.clone(),
    ));

    let contents = Row::new()
        .push(profile_picture)
        .push(Space::with_width(50))
        .push(text_column)
        .push(Space::with_width(Length::Fill))
        .push(add_friend_button)
        .align_y(Vertical::Center);

    let user_profile_button = create_element_button(
        active_mascot,
        contents.into(),
        ButtonStyle::InactiveTab,
        Some(DEFAULT_CONTAINER_RADIUS.into()),
    )
    .width(USER_BUTTON_WIDTH)
    .height(Length::Shrink)
    .on_press(Message::ViewProfile(UserType::Other(
        user.user_information.username.clone(),
    )));

    user_profile_button
}

pub fn profile_tab_button(app: &App) -> Button<Message> {
    let user: Element<Message> = Row::new()
        .push(
            iced::widget::image(Handle::from_path(
                app.user_manager.user_info.profile_picture_handle.clone(),
            ))
            .width(100)
            .height(100),
        )
        .push(Space::with_width(Length::FillPortion(2)))
        .push(column![
            format_button_text(iced::widget::text(
                app.user_manager.user_info.username.clone()
            ))
            .size(25),
            format_button_text(
                iced::widget::text(format!(
                    "{} week streak",
                    app.user_manager.user_info.weekly_workout_streak
                ))
                .size(12)
            )
        ])
        .push(Space::with_width(Length::FillPortion(7)))
        .align_y(Vertical::Center)
        .into();

    create_element_button(
        &app.mascot_manager.selected_mascot,
        user,
        ButtonStyle::InactiveTransparent,
        Some(DEFAULT_CONTAINER_RADIUS.into()),
    )
    .width(Length::Fill)
    .height(Length::Shrink)
    .on_press(Message::ViewProfile(UserType::Own))
}
