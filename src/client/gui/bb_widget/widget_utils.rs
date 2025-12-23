use iced_core::{Border, Layout, Shadow};
use iced_core::renderer::Quad;
use crate::client::gui::bb_theme::color;

pub const INDENT: f32 = 10.0;

pub fn background_quad(layout: Layout<'_>) -> Quad {
    Quad {
        bounds: layout.bounds(),
        border: Border {
            color: color::DARKER_CONTAINER_COLOR,
            width: 1.0,
            radius: 10.0.into(),
        },
        shadow: Shadow::default(),
    }
}
