use crate::Message;
use iced::Element;
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::{Quad, Style};
use iced_core::widget::Tree;
use iced_core::{image, renderer, text, Border, Image, Layout, Length, Rectangle, Size, Theme, Widget};
use crate::client::bb_theme;
use crate::client::bb_widget::widget_utils::INDENT;

const DEFAULT_PRESET_WIDTH: f32 = 208.0;
const DEFAULT_PRESET_HEIGHT: f32 = 252.0;
const IMAGE_HEIGHT: f32 = 125.0;

pub struct WorkoutPresetWidget<Renderer>
    where Renderer: image::Renderer
{
    image: image::Image<<Renderer as image::Renderer>::Handle>,
    title: String,
    exercises: Vec<String>,
    width: f32,
    height: f32,
}
impl<Renderer> Default for WorkoutPresetWidget<Renderer>
    where Renderer: image::Renderer<Handle = image::Handle>
{
    fn default() -> Self {
        Self {
            image: Image::new(image::Handle::from_path("assets/images/default_preset.png")),
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

impl<Renderer> Widget<Message, Theme, Renderer> for WorkoutPresetWidget<Renderer>
    where Renderer: renderer::Renderer + image::Renderer + text::Renderer
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(&self,
              _tree: &mut Tree,
              _renderer: &Renderer,
              _limits: &Limits) -> Node
    {
        Node::new(Size { width: self.width, height: self.height})
    }

    fn draw(&self,
            _tree: &Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &Style,
            layout: Layout<'_>,
            _cursor: Cursor,
            _viewport: &Rectangle) {

        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR,
                    width: 1.0,
                    radius: 15.0.into(),
                },
                shadow: Default::default(),
            },
            bb_theme::color::CONTAINER_COLOR);

        renderer.draw_image(self.image.clone(),
                            Rectangle {
                                x: layout.bounds().x + INDENT,
                                y: layout.bounds().y + INDENT,
                                width: self.width - 2.0 * INDENT,
                                height: IMAGE_HEIGHT, //TODO READ FROM IMAGE DIMENSIONS
                            });

    }
}
impl<'a, Renderer> From<WorkoutPresetWidget<Renderer>> for Element<'a, Message, Theme, Renderer>
    where Renderer: 'a + renderer::Renderer + image::Renderer + text::Renderer
{
    fn from(value: WorkoutPresetWidget<Renderer>) -> Self {
        Self::new(value)
    }
}

