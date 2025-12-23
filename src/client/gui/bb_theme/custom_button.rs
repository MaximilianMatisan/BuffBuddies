use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::color;
use crate::client::backend::mascot::mascot::Mascot;
use crate::client::gui::user_interface::{Message};
use iced::gradient::{ColorStop, Linear};
use iced::widget::{button, Button};
use iced::{Background, Color, Element, Gradient, Renderer};
use iced_core::border::Radius;
use iced_core::Theme;
use iced_core::widget::text;
use crate::client::backend::mascot::mascot_trait::MascotTrait;

const TAB_BUTTON_WIDTH: f32 = 225.0;
const TAB_BUTTON_HEIGHT: f32 = 45.0;
pub const DEFAULT_BUTTON_RADIUS: f32 = 190.0;

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    InactiveTab,
    ActiveTab,
    Active,
    InactiveTransparent,
    InactiveSolid
}
fn create_preset_button(element: Element<Message,Theme,Renderer>,
                        active_color: Color,
                        disabled_color: Color,
                        hovered_color: Color,
                        custom_border_radius: Option<Radius>) 
    -> Button<Message, Theme, Renderer> 
{
    let radius = if let Some(border_radius) = custom_border_radius{
        border_radius
    } else {
        DEFAULT_BUTTON_RADIUS.into()
    };
    button(element)
        .style(move |_, status: button::Status| {

            let mut style = button::Style::default();

            style.border.radius = radius;

            match status {
                button::Status::Active => {
                    style.background = Some(Background::Color(active_color));
                    style
                }
                button::Status::Disabled =>  {
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
                        Some(ColorStop { offset: 0.0, color: hovered_color}),
                        Some(ColorStop { offset: 1.0, color: disabled_color}),
                        None, None, None, None, None, None
                    ];

                    style.background = Some(Background::Gradient(Gradient::Linear(linear)));
                    style
                }
            }
        })
}
pub fn create_text_button<'a>(mascot: Mascot,
                              text: String,
                              button_style: ButtonStyle,
                              custom_border_radius: Option<Radius>) 
    -> Button<'a,Message, Theme, Renderer> {

    let text_elem: Element<Message> =
        bb_theme::text_format::format_button_text(text::Text::new(text)).into();
    
    create_element_button(mascot, text_elem, button_style, custom_border_radius)
}

pub fn create_element_button(mascot: Mascot, 
                             element: Element<Message, Theme, Renderer>, 
                             button_style: ButtonStyle,
                             custom_border_radius: Option<Radius>) 
    -> Button<Message, Theme, Renderer> {
    match button_style {
        ButtonStyle::InactiveTab => create_preset_button(element,
                                                         color::DARKER_CONTAINER_COLOR,
                                                         color::CONTAINER_COLOR,
                                                         color::HIGHLIGHTED_CONTAINER_COLOR,
                                                         custom_border_radius)
            .width(TAB_BUTTON_WIDTH)
            .height(TAB_BUTTON_HEIGHT),
        ButtonStyle::ActiveTab => create_preset_button(element,
                                                       color::HIGHLIGHTED_CONTAINER_COLOR,
                                                       color::CONTAINER_COLOR,
                                                       color::HIGHLIGHTED_CONTAINER_COLOR,
                                                       custom_border_radius)
            .width(TAB_BUTTON_WIDTH)
            .height(TAB_BUTTON_HEIGHT),
        ButtonStyle::InactiveTransparent => create_preset_button(element,
                                                                 Color::TRANSPARENT,
                                                                 mascot.get_dark_color(),
                                                                 mascot.get_primary_color(),
                                                                 custom_border_radius),
        ButtonStyle::InactiveSolid => create_preset_button(element,
                                                           mascot.get_dark_color(),
                                                           mascot.get_dark_color(),
                                                           mascot.get_primary_color(),
                                                           custom_border_radius),
        ButtonStyle::Active => create_preset_button(element,
                                                    mascot.get_primary_color(),
                                                    mascot.get_dark_color(),
                                                    mascot.get_secondary_color(),
                                                    custom_border_radius)
    }
}
