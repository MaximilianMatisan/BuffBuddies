use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::mascot_trait::MascotTrait;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::color;
use iced::gradient::{ColorStop, Linear};
use iced::widget::button::{Status, Style};
use iced::widget::{Button, button};
use iced::{Background, Color, Element, Gradient, Renderer};
use iced_core::border::Radius;
use iced_core::widget::text;
use iced_core::{Border, Theme};

pub const TAB_BUTTON_WIDTH: f32 = 225.0;
pub const TAB_BUTTON_HEIGHT: f32 = 45.0;
pub const DEFAULT_BUTTON_RADIUS: f32 = 190.0;

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
) -> Button<Msg, Theme, Renderer>
where
    Msg: Clone
{
    let radius = if let Some(border_radius) = custom_border_radius {
        border_radius
    } else {
        DEFAULT_BUTTON_RADIUS.into()
    };
    button(element).style(move |_, status: button::Status| {
        let border = Border {
            color: iced::color!(0, 0, 0),
            width: 0.0,
            radius,
        };
        create_button_style(status, border, active_color, disabled_color, hovered_color)
    })
}

pub fn create_button_style(
    status: Status,
    border: Border,
    active_color: Color,
    disabled_color: Color,
    hovered_color: Color,
) -> Style {
    let mut style = iced::widget::button::Style {
        border,
        ..Default::default()
    };
    match status {
        button::Status::Active => {
            style.background = Some(Background::Color(active_color));
            style
        }
        button::Status::Disabled => {
            style.background = Some(Background::Color(disabled_color));
            style
        }
        button::Status::Hovered => {
            style.background = Some(Background::Color(hovered_color));
            style
        }
        button::Status::Pressed => {
            let mut linear = Linear::new(0);
            linear.stops = [
                Some(ColorStop {
                    offset: 0.0,
                    color: hovered_color,
                }),
                Some(ColorStop {
                    offset: 1.0,
                    color: disabled_color,
                }),
                None,
                None,
                None,
                None,
                None,
                None,
            ];

            style.background = Some(Background::Gradient(Gradient::Linear(linear)));
            style
        }
    }
}
pub fn create_text_button<'a, Msg>(
    mascot: &Mascot,
    text: String,
    button_style: ButtonStyle,
    custom_border_radius: Option<Radius>,
) -> Button<'a, Msg, Theme, Renderer>
where
    Msg: Clone
{
    let text_elem: Element<Msg> =
        bb_theme::text_format::format_button_text(text::Text::new(text)).into();

    create_element_button(mascot, text_elem, button_style, custom_border_radius)
}

pub fn create_element_button<'a, Msg>(
    mascot: &Mascot,
    element: Element<'a, Msg, Theme, Renderer>,
    button_style: ButtonStyle,
    custom_border_radius: Option<Radius>,
) -> Button<'a, Msg, Theme, Renderer>
where
    Msg: Clone
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
            mascot.get_dark_color(),
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
            mascot.get_dark_color(),
            mascot.get_secondary_color(),
            custom_border_radius,
        ),
    }
}
