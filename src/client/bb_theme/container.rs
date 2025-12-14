use iced_core::{Border, Theme};
use iced::widget::container::Style;
use crate::client::bb_theme;

pub enum ContainerStyle {
    Highlighted,
    Default,
    Dark,
    Background
}
impl ContainerStyle {
    fn get_color(&self) -> iced::Color{
        match self{
            ContainerStyle::Highlighted => bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR,
            ContainerStyle::Default => bb_theme::color::CONTAINER_COLOR,
            ContainerStyle::Dark => bb_theme::color::DARKER_CONTAINER_COLOR,
            ContainerStyle::Background => bb_theme::color::BACKGROUND_COLOR
        }
    }
}

pub fn create_style_container(style: ContainerStyle) -> impl Fn(&Theme) -> Style {
    move |_theme: &Theme| Style {
        text_color: None,
        background: Some(iced::Background::Color(style.get_color())),
        border: Border {
            color: style.get_color(),
            width: 1.0,
            radius: 15.0.into()
        },
        shadow: Default::default(),
    }
}
