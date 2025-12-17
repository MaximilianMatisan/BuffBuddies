use iced::Renderer;
use iced::widget::canvas;
use iced::widget::canvas::Geometry;
use iced_core::{Color, Rectangle, Theme};
use iced_core::mouse::Cursor;

pub const INDENT: f32 = 10.0;

pub struct Circle {
    pub(crate) radius: f32,
    pub(crate) background_color: Color
}

impl<Message> canvas::Program<Message> for Circle {
    type State = ();
    fn draw(&self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            cursor: Cursor)
        -> Vec<Geometry<Renderer>>
    {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let circle = canvas::Path::circle(frame.center(), self.radius);

        frame.fill(&circle, self.background_color);

        vec![frame.into_geometry()]
    }
}