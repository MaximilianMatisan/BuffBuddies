use crate::client::gui::bb_theme::color;
use crate::common::exercise_mod::weight::Kg;
use iced::Renderer;
use iced_core::font::Family::Name;
use iced_core::font::Stretch::Normal;
use iced_core::font::Style;
use iced_core::font::Weight::ExtraBold;
use iced_core::widget::Text;
use iced_core::{Font, Theme};

/// default font
pub const FIRA_SANS_EXTRABOLD: Font = Font {
    family: Name("Fira Sans"),
    weight: ExtraBold,
    stretch: Normal,
    style: Style::Normal,
};

/// create default styling for text displayed in buttons
pub fn format_button_text(msg: Text<Theme, Renderer>) -> Text<Theme, Renderer> {
    msg.font(FIRA_SANS_EXTRABOLD)
        .color(color::TEXT_COLOR)
        .center()
}
/// create default styling for description text
pub fn format_description_text(msg: Text<Theme, Renderer>) -> Text<'_, Theme, Renderer> {
    msg.font(FIRA_SANS_EXTRABOLD)
        .color(color::DESCRIPTION_TEXT_COLOR)
}

// ------------------
// STRING FORMATTING
// ------------------

pub fn kg_to_string(kg: Kg) -> String {
    format!("{} kg", kg)
}
pub fn liter_to_string(liters: f32) -> String {
    format!("{} L", liters)
}
pub fn hours_to_string(hours: f32) -> String {
    format!("{} h", hours)
}
pub fn cm_to_string(cm: u32) -> String {
    format!("{} cm", cm)
}

/*pub fn option_to_content_or_none_string<X>(option: &Option<X>) -> String
where
    X: Display,
{
    option
        .as_ref()
        .map(|content| content.to_string())
        .unwrap_or("None".to_string())
}
*/
