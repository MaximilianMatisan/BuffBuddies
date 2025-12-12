use crate::Message;
use iced::Element;
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::{Quad, Style};
use iced_core::widget::Tree;
use iced_core::{alignment, image, text, Border, Image, Layout, Length, Point, Rectangle, Size, Theme, Widget};
use crate::client::bb_theme;
use crate::client::bb_widget::widget_utils::INDENT;

const DEFAULT_PRESET_WIDTH: f32 = 208.0;
const DEFAULT_PRESET_HEIGHT: f32 = 252.0;
const DEFAULT_TITLE_FONT_SIZE: f32 = 17.0;
const DEFAULT_DESCRIPTION_FONT_SIZE: f32 = 15.0;
const IMAGE_HEIGHT: f32 = 125.0;

pub struct WorkoutPresetWidget<Renderer>
    where Renderer: image::Renderer + iced_core::text::Renderer
{
    image: image::Image<<Renderer as image::Renderer>::Handle>,
    title: String,
    title_font_size: f32,
    exercises: Vec<String>,
    description_font_size: f32,
    width: f32,
    height: f32,
    font: <Renderer as iced_core::text::Renderer>::Font
}
impl<Renderer> Default for WorkoutPresetWidget<Renderer>
    where Renderer: image::Renderer<Handle = image::Handle> + iced_core::text::Renderer<Font = iced::Font>
{
    fn default() -> Self {
        Self {
            image: Image::new(image::Handle::from_path("assets/images/default_preset.png")),
            title: "Default Preset".to_string(),
            title_font_size: DEFAULT_TITLE_FONT_SIZE,
            //EXAMPLES
            exercises: vec!["Preacher Curl".to_string(),
                            "Bench Press".to_string(),
                            "Lateral Raises".to_string(),
                            "Chest Supported Row".to_string(),
                            "Triceps Extension".to_string()
            ],
            description_font_size: DEFAULT_DESCRIPTION_FONT_SIZE,
            width: DEFAULT_PRESET_WIDTH,
            height: DEFAULT_PRESET_HEIGHT,
            font: bb_theme::text_format::FIRA_SANS_EXTRABOLD
        }
    }
}
impl<Renderer> WorkoutPresetWidget<Renderer>
    where Renderer: image::Renderer + iced_core::text::Renderer
{
    pub fn set_image(mut self, img: image::Image<<Renderer as iced_core::image::Renderer>::Handle>)
        -> Self
    {
        self.image = img;
        self
    }
}
impl<Renderer> Widget<Message, Theme, Renderer> for WorkoutPresetWidget<Renderer>
    where Renderer: image::Renderer + text::Renderer
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
            viewport: &Rectangle) {

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


        let title_start_location_y: f32 = layout.bounds().y
                                            + 2.0 * INDENT + IMAGE_HEIGHT
                                            + 0.5 * DEFAULT_TITLE_FONT_SIZE;
                                            //FONT ISN'T PRINTED FROM THE TOP LEFT CORNER

        renderer.fill_text(iced_core::text::Text {
            content: self.title.to_string(),
            bounds: layout.bounds().size(),
            size: self.title_font_size.into(),
            line_height: Default::default(),
            font: self.font,
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Center,
            shaping: Default::default(),
            wrapping: Default::default(),
        }, Point {
            x: layout.bounds().x + self.width / 2.0,
            y: title_start_location_y,
        }, bb_theme::color::TEXT_COLOR, *viewport);

        let description_start_location_y: f32 = title_start_location_y
                                                + DEFAULT_TITLE_FONT_SIZE + INDENT;

        let printable_exercise_index: usize = 3;

        let mut printable_exercises = Vec::<String>::new();
        for (i,exercise) in self.exercises.iter().enumerate() {
            if i < printable_exercise_index {
                printable_exercises.push((*exercise).clone());
            } else {
                break
            }
        }
        if self.exercises.len() == printable_exercise_index + 1 {
            printable_exercises.push(self.exercises[printable_exercise_index].clone());
        } else if self.exercises.len() > printable_exercise_index + 1 {
            printable_exercises.push(". . .".to_string())
        }

        let mut description_lines = 0;
        for description_exercise in printable_exercises {
            renderer.fill_text(iced_core::text::Text {
                content: description_exercise,
                bounds: layout.bounds().size(),
                size: self.description_font_size.into(),
                line_height: Default::default(),
                font: renderer.default_font(),
                horizontal_alignment: alignment::Horizontal::Left,
                vertical_alignment: alignment::Vertical::Center,
                shaping: Default::default(),
                wrapping: Default::default(),
            }, Point {
                x: layout.bounds().x + INDENT,
                y: description_start_location_y
                    + description_lines as f32 * self.description_font_size,
            }, bb_theme::color::DESCRIPTION_TEXT_COLOR, *viewport);
            description_lines += 1;
        }

    }
}
impl<'a, Renderer> From<WorkoutPresetWidget<Renderer>> for Element<'a, Message, Theme, Renderer>
    where Renderer: 'a + image::Renderer + text::Renderer
{
    fn from(value: WorkoutPresetWidget<Renderer>) -> Self {
        Self::new(value)
    }
}

