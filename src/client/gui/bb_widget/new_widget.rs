use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_theme::color::{
    CONTAINER_COLOR, HIGHLIGHTED_CONTAINER_COLOR, LIGHTER_CONTAINER_COLOR,
};
use crate::client::gui::bb_theme::custom_button::{create_button_style, create_gradient_mascot_style};
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::bb_widget::workout::{
    DEFAULT_RECENT_WORKOUT_WIDGET_HEIGHT, DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT,
    DEFAULT_WORKOUT_WIDGET_WIDTH,
};
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::Renderer;
use iced::widget::{Button, text};
use iced::widget::{Column, column};
use iced::widget::{Space, button};
use iced_anim::Motion;
use iced_core::alignment::Horizontal;
use iced_core::border::Radius;
use iced_core::{Border, Length, Theme};
use std::time::Duration;

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

pub fn new_workout_widget_button<'a>(
    mascot: Mascot,
) -> iced_anim::widget::Button<'a, Message, Theme, Renderer> {
    let content = Column::new()
        .push(Space::with_height(Length::Fill))
        .push(format_button_text(text("New workout")).size(DEFAULT_TITLE_FONT_SIZE))
        .push(format_button_text(text("+").size(ADD_SYMBOL_SIZE)))
        .push(Space::with_height(Length::Fill))
        .width(BUTTONS_WIDTH)
        .height(NEW_WORKOUT_BUTTON_HEIGHT)
        .align_x(Horizontal::Center);

    iced_anim::widget::button(content)
        .animation(Motion {
            response: Duration::from_millis(300),
            damping: Motion::SMOOTH.damping(),
        })
        .style(move |_, status| {
            create_gradient_mascot_style(
                status,
                mascot
            )
        })
        .width(BUTTONS_WIDTH)
        .height(NEW_WORKOUT_BUTTON_HEIGHT)
        .on_press(Message::Select(Tab::CreateWorkout))
}

pub fn new_preset_widget_button<'a>(mascot: Mascot) -> iced_anim::widget::Button<'a, Message, Theme, Renderer> {

    let content = Column::new()
        .push(Space::with_height(Length::Fill))
        .push(format_button_text(text("New preset")).size(DEFAULT_TITLE_FONT_SIZE))
        .push(format_button_text(text("+").size(ADD_SYMBOL_SIZE)))
        .push(Space::with_height(Length::Fill))
        .width(BUTTONS_WIDTH)
        .height(NEW_PRESET_BUTTON_HEIGHT)
        .align_x(Horizontal::Center);

    iced_anim::widget::button(content)
        .animation(Motion {
            damping: Motion::SMOOTH.damping,
            response: Duration::from_millis(350),
        })
    .style(move |_, status| {
        create_gradient_mascot_style(
            status,
            mascot
        )
    })
    .width(BUTTONS_WIDTH)
    .height(NEW_PRESET_BUTTON_HEIGHT)
    .on_press(Message::Select(Tab::CreatePreset))
}
