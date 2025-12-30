use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use crate::client::gui::{bb_theme, bb_widget};
use iced::{Element, event};
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::Style;
use iced_core::widget::Tree;
use iced_core::{
    Clipboard, Event, Image, Layout, Length, Point, Rectangle, Shell, Size, Theme, Widget,
    alignment, image, mouse, text,
};

const SCALE: f32 = 1.0;
pub const DEFAULT_WORKOUT_WIDGET_WIDTH: f32 = 208.0 * SCALE;
pub const DEFAULT_RECENT_WORKOUT_WIDGET_HEIGHT: f32 = 158.0 * SCALE;
pub const DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT: f32 = 252.0 * SCALE;
const IMAGE_HEIGHT: f32 = 125.0 * SCALE;
//FONT
const DEFAULT_TITLE_FONT_SIZE: f32 = 17.0 * SCALE;
const DEFAULT_DESCRIPTION_FONT_SIZE: f32 = 15.0 * SCALE;

pub struct WorkoutWidget<Renderer>
where
    Renderer: image::Renderer + text::Renderer,
{
    image: Option<Image<<Renderer as image::Renderer>::Handle>>,
    title: String,
    title_font_size: f32,
    exercises: Vec<String>,
    description_font_size: f32,
    width: f32,
    height: f32,
    font: <Renderer as iced_core::text::Renderer>::Font,
    on_press: Option<Message>,
}
impl<Renderer> WorkoutWidget<Renderer>
where
    Renderer:
        image::Renderer<Handle = iced_core::image::Handle> + text::Renderer<Font = iced::Font>,
{
    pub fn default_workout_preset_widget() -> Self {
        WorkoutWidget {
            width: DEFAULT_WORKOUT_WIDGET_WIDTH,
            height: DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT,
            image: Some(Image::new(image::Handle::from_path(
                "assets/images/default_preset.png",
            ))),
            title: "Default preset".to_string(),
            title_font_size: DEFAULT_TITLE_FONT_SIZE,
            //TODO FETCH FROM DATABASE
            exercises: vec![
                "Preacher Curl".to_string(),
                "Bench Press".to_string(),
                "Lateral Raises".to_string(),
                "Chest Supported Row".to_string(),
                "Triceps Extension".to_string(),
            ],
            description_font_size: DEFAULT_DESCRIPTION_FONT_SIZE,
            font: bb_theme::text_format::FIRA_SANS_EXTRABOLD,
            on_press: None,
        }
    }
    pub fn default_recent_workout_widget() -> Self {
        WorkoutWidget {
            width: DEFAULT_WORKOUT_WIDGET_WIDTH,
            height: DEFAULT_RECENT_WORKOUT_WIDGET_HEIGHT,
            image: None,
            title: "Today".to_string(),
            title_font_size: DEFAULT_TITLE_FONT_SIZE,
            //TODO FETCH FROM DATABASE
            exercises: vec![
                "Preacher Curl".to_string(),
                "Bench Press".to_string(),
                "Lateral Raises".to_string(),
                "Chest Supported Row".to_string(),
                "Triceps Extension".to_string(),
                "Leg Press".to_string(),
                //"Leg Extension".to_string(),
                // "Hamstring Curls".to_string(),
            ],
            description_font_size: DEFAULT_DESCRIPTION_FONT_SIZE,
            font: bb_theme::text_format::FIRA_SANS_EXTRABOLD,
            on_press: None,
        }
    }
    pub fn set_image(
        mut self,
        img: Option<Image<<Renderer as iced_core::image::Renderer>::Handle>>,
    ) -> Self {
        self.image = img;
        self
    }
}
impl<Renderer> Widget<Message, Theme, Renderer> for WorkoutWidget<Renderer>
where
    Renderer: image::Renderer + text::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, _limits: &Limits) -> Node {
        Node::new(Size {
            width: self.width,
            height: self.height,
        })
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            bb_widget::widget_utils::background_quad(layout),
            bb_theme::color::CONTAINER_COLOR,
        );
        if let Some(img) = &self.image {
            renderer.draw_image(
                img.clone(),
                Rectangle {
                    x: layout.bounds().x + INDENT,
                    y: layout.bounds().y + INDENT,
                    width: self.width - 2.0 * INDENT, //Jeweils indent links und rechts
                    height: IMAGE_HEIGHT,
                },
            );
        }
        let title_start_location_y: f32;
        let description_start_location_y: f32;

        if self.image.is_none() {
            title_start_location_y = layout.bounds().y + INDENT + 0.5 * self.title_font_size;
            description_start_location_y =
                layout.bounds().y + 2.0 * INDENT + 1.5 * self.title_font_size;
        } else {
            title_start_location_y = layout.bounds().y + 3.0 * INDENT + IMAGE_HEIGHT;
            description_start_location_y =
                layout.bounds().y + 3.0 * INDENT + IMAGE_HEIGHT + 1.5 * self.title_font_size
        };

        renderer.fill_text(
            text::Text {
                content: self.title.to_string(),
                bounds: layout.bounds().size(),
                size: iced_core::Pixels(self.title_font_size),
                line_height: Default::default(),
                font: self.font,
                horizontal_alignment: alignment::Horizontal::Center,
                vertical_alignment: alignment::Vertical::Center,
                shaping: Default::default(),
                wrapping: Default::default(),
            },
            Point {
                x: layout.bounds().x + self.width / 2.0,
                y: title_start_location_y,
            },
            bb_theme::color::TEXT_COLOR,
            *viewport,
        );

        let printable_index: usize = if self.image.is_none() { 5 } else { 3 };

        let mut printable_exercises = Vec::<String>::new();
        for (i, exercise) in self.exercises.iter().enumerate() {
            if i < printable_index {
                printable_exercises.push((*exercise).clone());
            } else {
                break;
            }
        }

        match self.exercises.len().cmp(&(printable_index + 1)) {
            std::cmp::Ordering::Equal => {
                printable_exercises.push(self.exercises[printable_index].clone());
            }
            std::cmp::Ordering::Greater => printable_exercises.push(". . .".to_string()),
            std::cmp::Ordering::Less => {}
        }

        for (description_lines, description_exercise) in printable_exercises.into_iter().enumerate()
        {
            renderer.fill_text(
                text::Text {
                    content: description_exercise,
                    bounds: layout.bounds().size(),
                    size: iced_core::Pixels(self.description_font_size),
                    line_height: Default::default(),
                    font: renderer.default_font(),
                    horizontal_alignment: alignment::Horizontal::Left,
                    vertical_alignment: alignment::Vertical::Center,
                    shaping: Default::default(),
                    wrapping: Default::default(),
                },
                Point {
                    x: layout.bounds().x + INDENT,
                    y: description_start_location_y
                        + description_lines as f32 * self.description_font_size,
                },
                bb_theme::color::DESCRIPTION_TEXT_COLOR,
                *viewport,
            );
        }
    }
    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        if cursor.is_over(layout.bounds()) {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(_)) => match &self.on_press {
                    Some(msg) => {
                        shell.publish(msg.clone());
                        event::Status::Captured
                    }
                    None => event::Status::Ignored,
                },
                _ => event::Status::Ignored,
            }
        } else {
            event::Status::Ignored
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());
        if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}
impl<'a, Renderer> From<WorkoutWidget<Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + image::Renderer + text::Renderer,
{
    fn from(value: WorkoutWidget<Renderer>) -> Self {
        Self::new(value)
    }
}
