use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use iced::widget::canvas;
use iced::widget::canvas::path::Arc;
use iced::widget::canvas::{Frame, LineCap, LineJoin, Path, Stroke, stroke};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Color, Degrees, Point};

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

/// # Arguments
///
/// * `radius` - Radius of the circle.
/// * `start_angle` - The start of the segment's angle, clockwise rotation from positive x-axis
/// * `end_angle` - The end of the segment's angle, clockwise rotation from positive x-axis
pub fn create_arc_path(center: Point, radius: f32, start_angle: f32, end_angle: f32) -> Path {
    Path::new(|builder| {
        builder.arc(Arc {
            center,
            radius,
            start_angle: Degrees(start_angle).into(),
            end_angle: Degrees(end_angle).into(),
        });
    })
}
