use crate::client::backend::exercise_mod::weight::Kg;
use crate::client::gui::bb_theme::color;
use iced::Renderer;
use iced_core::font::Family::Name;
use iced_core::font::Stretch::Normal;
use iced_core::font::Style;
use iced_core::font::Weight::ExtraBold;
use iced_core::widget::Text;
use iced_core::{Font, Theme};

pub const FIRA_SANS_EXTRABOLD: Font = Font {
    family: Name("Fira Sans"),
    weight: ExtraBold,
    stretch: Normal,
    style: Style::Normal,
};

// pub const ARIAL_ROUNDED_B: Font = Font::with_name("Arial Rounded MT Bold");

pub fn format_button_text(msg: Text<Theme, Renderer>) -> Text<Theme, Renderer> {
    msg.font(FIRA_SANS_EXTRABOLD)
        .color(color::TEXT_COLOR)
        .center()
}
pub fn format_description_text(msg: Text<Theme, Renderer>) -> Text<'_, Theme, Renderer> {
    msg.font(FIRA_SANS_EXTRABOLD)
        .color(color::DESCRIPTION_TEXT_COLOR)
}
pub fn kg_to_string(kg: Kg) -> String {
    format!("{} kg", kg)
}
pub fn cm_to_string(cm: u32) -> String {
    format!("{} cm", cm)
}
