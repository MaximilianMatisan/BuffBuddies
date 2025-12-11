use iced::{Element, Renderer};
use iced_core::{image, Image, Layout, Length, Rectangle, Size, Theme, Widget};
use iced_core::image::Handle;
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::Style;
use iced_core::widget::Tree;
use crate::Message;

const DEFAULT_PRESET_WIDTH: f32 = 208.0;
const DEFAULT_PRESET_HEIGHT: f32 = 252.0;

pub struct WorkoutPresetWidget {
    image: Image,
    title: String,
    exercises: Vec<String>,
    width: f32,
    height: f32,
}
impl Default for WorkoutPresetWidget {
    fn default() -> Self {
        Self {
            image: Image::new(Handle::from_path("assets/images/default_preset.png")),
            title: "Default Preset".to_string(),
            //EXAMPLES
            exercises: vec!["Preacher Curl".to_string(),
                            "Bench Press".to_string(),
                            "Lateral Raises".to_string(),
                            "Chest Supported Row".to_string(),
                            "Triceps Extension".to_string()
            ],
            width: DEFAULT_PRESET_WIDTH,
            height: DEFAULT_PRESET_HEIGHT,
        }
    }
}

impl Widget<Message, Theme, Renderer> for WorkoutPresetWidget {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        Node::new(Size { width: self.width, height: self.height})
    }

    fn draw(&self,
            tree: &Tree,
            renderer: &mut Renderer,
            theme: &Theme,
            style: &Style,
            layout: Layout<'_>,
            cursor: Cursor,
            viewport: &Rectangle) {
        todo!()
    }
}
impl From<WorkoutPresetWidget> for Element<'_, Message> {
    fn from(value: WorkoutPresetWidget) -> Self {
        Self::new(value)
    }
}

