use crate::client::gui::bb_theme;
use iced::widget::container::Style;
use iced_core::border::Radius;
use iced_core::{Border, Color, Theme};

pub const DEFAULT_CONTAINER_RADIUS: f32 = 15.0;
pub enum ContainerStyle {
    Highlighted,
    Default,
    Light,
    Background,
}
impl ContainerStyle {
    fn get_color(&self) -> Color {
        match self {
            ContainerStyle::Highlighted => bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR,
            ContainerStyle::Default => bb_theme::color::CONTAINER_COLOR,
            ContainerStyle::Light => bb_theme::color::LIGHTER_CONTAINER_COLOR,
            ContainerStyle::Background => bb_theme::color::BACKGROUND_COLOR,
        }
    }
}

pub fn create_style_container(
    style: ContainerStyle,
    custom_border_radius: Option<Radius>,
    custom_border_color: Option<Color>,
) -> impl Fn(&Theme) -> Style {
    let border_radius = custom_border_radius.unwrap_or_else(|| DEFAULT_CONTAINER_RADIUS.into());
    let border_color = custom_border_color.unwrap_or_else(|| style.get_color());

    move |_theme: &Theme| Style {
        text_color: None,
        background: Some(iced::Background::Color(style.get_color())),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: border_radius,
        },
        shadow: Default::default(),
    }
}
