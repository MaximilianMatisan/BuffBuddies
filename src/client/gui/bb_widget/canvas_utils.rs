use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use iced::widget::canvas;
use iced::widget::canvas::{Frame, LineCap, LineJoin, Stroke, stroke};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Color, Point};

pub fn generate_stroke<'a>(width: f32, color: Color) -> Stroke<'a> {
    Stroke {
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Solid(color),
        line_dash: Default::default(),
    }
}
pub fn draw_text(frame: &mut Frame, content: String, font_size: f32, position: Point) {
    frame.fill_text(canvas::Text {
        content,
        size: font_size.into(),
        position,
        color: TEXT_COLOR,
        font: FIRA_SANS_EXTRABOLD,
        horizontal_alignment: Horizontal::Center,
        vertical_alignment: Vertical::Center,
        line_height: Default::default(),
        shaping: iced_core::text::Shaping::Advanced,
    });
}
