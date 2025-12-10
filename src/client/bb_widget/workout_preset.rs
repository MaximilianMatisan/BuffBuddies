use iced::{Element, Renderer};
use iced_core::{image, Layout, Length, Rectangle, Size, Theme, Widget};
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::Style;
use iced_core::widget::Tree;
use crate::Message;

pub struct WorkoutPresetWidget {
    image: image::Handle,
    title: String,
    exercises: Vec<String>,
    width: f32,
    height: f32,
}

impl Widget<Message, Theme, Renderer> for WorkoutPresetWidget {
    fn size(&self) -> Size<Length> {
        todo!()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        todo!()
    }

    fn draw(&self, tree: &Tree, renderer: &mut Renderer, theme: &Theme, style: &Style, layout: Layout<'_>, cursor: Cursor, viewport: &Rectangle) {
        todo!()
    }
}
impl From<WorkoutPresetWidget> for Element<'_, Message> {
    fn from(value: WorkoutPresetWidget) -> Self {
        Self::new(value)
    }
}

