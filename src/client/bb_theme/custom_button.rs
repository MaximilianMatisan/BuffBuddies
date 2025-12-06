use crate::client::app::App;
use crate::client::bb_theme::color;
use crate::{Message};
use iced::gradient::{ColorStop, Linear};
use iced::widget::{button, Button};
use iced::{border, Background, Color, Element, Gradient, Renderer};
use iced_core::Theme;
use iced_core::widget::text;
use crate::client::bb_theme;

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonStyle {
    #[default]
    ClickableTabButton,
    ClickedTabButton,
    ClickedSelection,
    ClickableSelection
}
fn create_preset_button(element: Element<'_, Message,Theme,Renderer>,
                        active_color: Color,
                        disabled_color: Color,
                        hovered_color: Color
) -> Button<'_, Message, Theme, Renderer> {
    button(element)
        .style(move |_, status: button::Status| {

            let mut style = button::Style::default();

            style.border.radius = border::radius(193.0);

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
pub fn create_text_button<'a>(app: &'a App, text: &'a str, button_style: ButtonStyle)
    -> Button<'a, Message, Theme, Renderer>{

    let text_elem: Element<Message> =
        <Element<Message>>::from(bb_theme::text_format::format_button_text(text::Text::new(text)));

    create_elem_button(app, text_elem, button_style)
}

pub fn create_elem_button<'a>(app: &'a App, element: Element<'a,Message, Theme, Renderer>, button_style: ButtonStyle)
                              -> Button<'a, Message, Theme, Renderer> {
    match button_style {
        ButtonStyle::ClickableTabButton => create_preset_button(element,
                                                                color::DARKER_CONTAINER_COLOR,
                                                                color::CONTAINER_COLOR,
                                                                color::HIGHLIGHTED_CONTAINER_COLOR)
            .width(225)
            .height(44),
        ButtonStyle::ClickedTabButton => create_preset_button(element,
                                                              color::HIGHLIGHTED_CONTAINER_COLOR,
                                                              color::CONTAINER_COLOR,
                                                              color::HIGHLIGHTED_CONTAINER_COLOR)
            .width(225)
            .height(44),
        ButtonStyle::ClickableSelection => create_preset_button(element,
                                                                Color::TRANSPARENT,
                                                                app.active_mascot.get_disabled_color(),
                                                                app.active_mascot.get_primary_color()),
        ButtonStyle::ClickedSelection => create_preset_button(element,
                                                              app.active_mascot.get_primary_color(),
                                                              app.active_mascot.get_disabled_color(),
                                                              app.active_mascot.get_secondary_color())
    }
}
