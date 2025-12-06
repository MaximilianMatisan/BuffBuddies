use crate::client::bb_theme::color;
use iced::Renderer;
use iced_core::widget::Text;
use iced_core::{Font, Theme};

pub const ARIAL_ROUNDED_B: Font = Font::with_name("Arial Rounded MT Bold");

pub fn format_button_text(msg: Text<'_, Theme, Renderer>) -> Text<'_, Theme, Renderer>{
    msg.font(ARIAL_ROUNDED_B).color(color::TEXT_COLOR).center()
}
pub fn format_description_text(msg: Text<'_, Theme, Renderer>) -> Text<'_, Theme, Renderer> {
    msg.font(ARIAL_ROUNDED_B).color(color::DESCRIPTION_TEXT_COLOR).size(15)
}