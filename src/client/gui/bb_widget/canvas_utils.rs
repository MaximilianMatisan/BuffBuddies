use iced::widget::canvas::{stroke, LineCap, LineJoin, Stroke};
use iced_core::{color, Color};

pub fn generate_stroke <'a> (width: f32,color: Color) -> Stroke<'a> {
    
    Stroke {
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Solid(color),
        line_dash: Default::default()
    }
    
}