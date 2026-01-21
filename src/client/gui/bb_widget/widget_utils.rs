use iced::Element;
use iced::widget::{text, Row, Space};
use crate::client::gui::bb_theme::color;
use iced_core::renderer::Quad;
use iced_core::{Border, Layout, Length, Shadow};
use crate::client::gui::bb_theme::text_format::{format_button_text, format_description_text};

pub const INDENT: f32 = 10.0;
pub const LARGE_INDENT: f32 = 30.0;

pub fn background_quad(layout: Layout<'_>) -> Quad {
    Quad {
        bounds: layout.bounds(),
        border: Border {
            color: color::LIGHTER_CONTAINER_COLOR,
            width: 1.0,
            radius: 10.0.into(),
        },
        shadow: Shadow::default(),
    }
}
pub fn descriptor_space_fill_text_row<'a,Msg>(
    description_text: &'a str,
    information_text: String,
) -> Row<'a, Msg>
where Msg: Clone + 'a
{
    descriptor_space_fill_element_row(
        description_text,
        format_button_text(text(information_text)).into(),
    )
}
pub fn descriptor_space_fill_element_row<'a, Msg>(
    description_text: &'a str,
    data_element: Element<'a, Msg>,
) -> Row<'a, Msg>
where Msg: Clone + 'a
{
    Row::new()
        .push(format_description_text(text(description_text)))
        .push(Space::with_width(Length::Fill))
        .push(data_element)
}
