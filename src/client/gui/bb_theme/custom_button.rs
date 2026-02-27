use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::color::{
    CONTAINER_COLOR, DARK_SHADOW, HIGHLIGHTED_CONTAINER_COLOR, create_color_stops,
    create_new_gradient_background,
};
use crate::client::gui::bb_widget::activity_widget::date_utils::Offset;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::gradient::{ColorStop, Linear};
use iced::widget::button::{Status, Style};
use iced::{Background, Color, Element, Gradient, Renderer};
use iced_anim::Motion;
use iced_anim::animated::Mode;
use iced_anim::widget::button;
use iced_core::border::Radius;
use iced_core::widget::text;
use iced_core::{Border, Shadow, Theme, Vector, color};
use std::f32::consts::PI;
use std::time::Duration;

pub const TAB_BUTTON_WIDTH: f32 = 225.0;
pub const TAB_BUTTON_HEIGHT: f32 = 45.0;
pub const DEFAULT_BUTTON_RADIUS: f32 = 190.0;

pub const BUTTON_RADIUS_LEFT_ZERO: Radius = Radius {
    top_left: 0.0,
    top_right: DEFAULT_BUTTON_RADIUS,
    bottom_right: DEFAULT_BUTTON_RADIUS,
    bottom_left: 0.0,
};

pub const BUTTON_RADIUS_RIGHT_ZERO: Radius = Radius {
    top_left: DEFAULT_BUTTON_RADIUS,
    top_right: 0.0,
    bottom_right: 0.0,
    bottom_left: DEFAULT_BUTTON_RADIUS,
};
pub const ANIMATION_MOTION: Motion = Motion {
    response: Duration::from_millis(400),
    damping: Motion::BOUNCY.damping,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    InactiveTab,
    ActiveTab,
    Active,
    InactiveTransparent,
    InactiveSolid,
}
pub fn create_preset_button<Msg>(
    element: Element<Msg, Theme, Renderer>,
    active_color: Color,
    disabled_color: Color,
    hovered_color: Color,
    custom_border_radius: Option<Radius>,
) -> iced_anim::widget::Button<Msg, Theme, Renderer>
where
    Msg: Clone,
{
    let radius = if let Some(border_radius) = custom_border_radius {
        border_radius
    } else {
        DEFAULT_BUTTON_RADIUS.into()
    };
    button(element)
        .style(move |_, status: Status| {
            let border = Border {
                color: Color::WHITE,
                width: 0.0,
                radius,
            };
            create_button_style(status, border, active_color, disabled_color, hovered_color)
        })
        .animation(ANIMATION_MOTION)
}

pub fn create_button_style(
    status: Status,
    border: Border,
    active_color: Color,
    disabled_color: Color,
    hovered_color: Color,
) -> Style {
    let mut style = Style {
        border,
        ..Default::default()
    };
    match status {
        Status::Active => {
            style.background = Some(create_new_gradient_background(
                0,
                create_color_stops(vec![(active_color, 0.0), (active_color, 1.0)]),
            ));
            style
        }
        Status::Disabled => {
            style.background = Some(create_new_gradient_background(
                0,
                create_color_stops(vec![(disabled_color, 0.0), (disabled_color, 1.0)]),
            ));
            style
        }
        Status::Hovered => {
            style.background = Some(create_new_gradient_background(
                0,
                create_color_stops(vec![(hovered_color, 0.0), (hovered_color, 1.0)]),
            ));
            style
        }
        Status::Pressed => {
            style.background = Some(create_new_gradient_background(
                PI / 2.0,
                create_color_stops(vec![(disabled_color, 0.0), (hovered_color, 1.0)]),
            ));
            style.border = morph_border(style.border, 15.0);
            style
        }
    }
}
/// Create an iced button containing a centered white fira sans text and a specified ButtonStyle
pub fn create_text_button<'a, Msg>(
    mascot: &Mascot,
    text: String,
    button_style: ButtonStyle,
    custom_border_radius: Option<Radius>,
) -> iced_anim::widget::Button<'a, Msg, Theme, Renderer>
where
    Msg: Clone,
{
    let text_elem: Element<Msg> =
        bb_theme::text_format::format_button_text(text::Text::new(text)).into();

    create_element_button(mascot, text_elem, button_style, custom_border_radius)
}

/// Create a iced button containing an iced element and a specified ButtonStyle
pub fn create_element_button<'a, Msg>(
    mascot: &Mascot,
    element: Element<'a, Msg, Theme, Renderer>,
    button_style: ButtonStyle,
    custom_border_radius: Option<Radius>,
) -> iced_anim::widget::Button<'a, Msg, Theme, Renderer>
where
    Msg: Clone,
{
    match button_style {
        ButtonStyle::InactiveTab => create_preset_button(
            element,
            color::LIGHTER_CONTAINER_COLOR,
            color::CONTAINER_COLOR,
            color::HIGHLIGHTED_CONTAINER_COLOR,
            custom_border_radius,
        ),
        ButtonStyle::ActiveTab => create_preset_button(
            element,
            color::HIGHLIGHTED_CONTAINER_COLOR,
            color::CONTAINER_COLOR,
            color::HIGHLIGHTED_CONTAINER_COLOR,
            custom_border_radius,
        ),
        ButtonStyle::InactiveTransparent => create_preset_button(
            element,
            Color::TRANSPARENT,
            mascot.get_secondary_color(),
            mascot.get_primary_color(),
            custom_border_radius,
        ),
        ButtonStyle::InactiveSolid => create_preset_button(
            element,
            mascot.get_dark_color(),
            mascot.get_dark_color(),
            mascot.get_primary_color(),
            custom_border_radius,
        ),
        ButtonStyle::Active => create_preset_button(
            element,
            mascot.get_primary_color(),
            mascot.get_primary_color(),
            mascot.get_secondary_color(),
            custom_border_radius,
        ),
    }
}

pub fn create_gradient_mascot_style(status: Status, mascot: Mascot) -> iced::widget::button::Style {
    let active_color = HIGHLIGHTED_CONTAINER_COLOR;
    let pressed_color = mascot.get_primary_color();
    let hovered_color = mascot.get_secondary_color();

    let active_color_stops = create_color_stops(vec![(active_color, 0.0), (active_color, 1.0)]);

    let pressed_color_stops = create_color_stops(vec![(pressed_color, 0.0), (pressed_color, 1.0)]);

    let hovered_color_stops =
        create_color_stops(vec![(hovered_color, 0.0), (CONTAINER_COLOR, 0.7)]);

    let gradient = match status {
        Status::Active => Gradient::Linear(Linear::new(0).add_stops(active_color_stops)),

        Status::Disabled => Gradient::Linear(Linear::new(0.0).add_stops([ColorStop {
            ..Default::default()
        }])), //buttons using this gradient never are disabled

        button::Status::Pressed => {
            Gradient::Linear(Linear::new(0.0).add_stops(pressed_color_stops))
        }

        Status::Hovered => Gradient::Linear(Linear::new(0.0).add_stops(hovered_color_stops)),
    };

    iced::widget::button::Style {
        background: Some(Background::Gradient(gradient)),

        border: match status {
            Status::Active => Border::default().color(pressed_color).rounded(10),
            _ => Border::default().width(2.5).color(Color::WHITE).rounded(24),
        },

        ..Default::default()
    }
}

///Takes a border and returns a new one with a customized radius.
/// You should pass a radius value which is not negative
fn morph_border(border: Border, radius: f32) -> Border {
    let mut border_radius = border.radius;
    if (border_radius.top_right) > 0.0 {
        border_radius.top_right = radius
    }
    if (border_radius.top_left) > 0.0 {
        border_radius.top_left = radius
    }
    if (border_radius.bottom_right) > 0.0 {
        border_radius.bottom_right = radius
    }
    if (border_radius.bottom_left) > 0.0 {
        border_radius.bottom_left = radius
    }
    Border {
        radius: border_radius,
        ..Default::default()
    }
}
