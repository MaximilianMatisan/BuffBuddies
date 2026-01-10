use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_theme::color::{
    CONTAINER_COLOR, HIGHLIGHTED_CONTAINER_COLOR, LIGHTER_CONTAINER_COLOR,
};
use crate::client::gui::bb_theme::custom_button::create_button_style;
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::bb_widget::workout::{
    DEFAULT_RECENT_WORKOUT_WIDGET_HEIGHT, DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT,
    DEFAULT_WORKOUT_WIDGET_WIDTH,
};
use crate::client::gui::user_interface::Message;
use iced::Renderer;
use iced::widget::column;
use iced::widget::{Button, text};
use iced::widget::{Space, button};
use iced_core::alignment::Horizontal;
use iced_core::border::Radius;
use iced_core::{Border, Length, Theme};

const SCALE: f32 = 0.9;
const BUTTONS_WIDTH: f32 = DEFAULT_WORKOUT_WIDGET_WIDTH;
const NEW_WORKOUT_BUTTON_HEIGHT: f32 = DEFAULT_RECENT_WORKOUT_WIDGET_HEIGHT;
const NEW_PRESET_BUTTON_HEIGHT: f32 = DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT;
const DEFAULT_TITLE_FONT_SIZE: f32 = 24.0 * SCALE;
const BUTTON_BORDER_SIZE: f32 = 1.2;
const ADD_SYMBOL_SIZE: f32 = 72.0 * SCALE;

const BORDER: Border = Border {
    color: iced::color!(146, 142, 142),
    width: BUTTON_BORDER_SIZE,
    radius: Radius {
        top_left: 10.0,
        top_right: 10.0,
        bottom_right: 10.0,
        bottom_left: 10.0,
    },
};

pub fn new_workout_widget_button<'a>() -> Button<'a, Message, Theme, Renderer> {
    button(
        column!(
            Space::with_height(Length::Fill),
            format_button_text(text("New workout")).size(DEFAULT_TITLE_FONT_SIZE),
            format_button_text(text("+").size(ADD_SYMBOL_SIZE)),
            Space::with_height(Length::Fill)
        )
        .width(BUTTONS_WIDTH)
        .height(NEW_WORKOUT_BUTTON_HEIGHT)
        .align_x(Horizontal::Center),
    )
    .style(|_, status| {
        create_button_style(
            status,
            BORDER,
            HIGHLIGHTED_CONTAINER_COLOR,
            LIGHTER_CONTAINER_COLOR,
            CONTAINER_COLOR,
        )
    })
    .width(BUTTONS_WIDTH)
    .height(NEW_WORKOUT_BUTTON_HEIGHT)
    .on_press(Message::Select(Tab::Home))
}

pub fn new_preset_widget_button<'a>() -> Button<'a, Message, Theme, Renderer> {
    button(
        column!(
            Space::with_height(Length::Fill),
            format_button_text(text("New preset")).size(DEFAULT_TITLE_FONT_SIZE),
            format_button_text(text("+").size(ADD_SYMBOL_SIZE)),
            Space::with_height(Length::Fill)
        )
        .width(BUTTONS_WIDTH)
        .height(NEW_PRESET_BUTTON_HEIGHT)
        .align_x(Horizontal::Center),
    )
    .style(|_, status| {
        create_button_style(
            status,
            BORDER,
            HIGHLIGHTED_CONTAINER_COLOR,
            LIGHTER_CONTAINER_COLOR,
            CONTAINER_COLOR,
        )
    })
    .width(BUTTONS_WIDTH)
    .height(NEW_PRESET_BUTTON_HEIGHT)
    .on_press(Message::Select(Tab::Home))
}
