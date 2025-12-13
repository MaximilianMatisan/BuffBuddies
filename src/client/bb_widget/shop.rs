use crate::client::bb_theme;
use iced::advanced::{
    layout::Layout,
    widget::Widget,
    {renderer, Clipboard, Shell}
};
use iced::{event, Event};
use iced::{mouse, Element};
use iced::{Length, Rectangle, Size};
use iced_core::layout::{Limits, Node};
use iced_core::renderer::Quad;
use iced_core::widget::Tree;
use iced_core::{alignment, Point};
use iced_core::{image, Border, Shadow};

const INDENT: f32 = DEFAULT_PRESET_HEIGHT/13.0 * SCALE;
const TITLE_FONT_SIZE: f32 = 27.5 * SCALE;
const DEFAULT_PRESET_WIDTH: f32 = 389.0 * SCALE;
const DEFAULT_PRESET_HEIGHT: f32 = 377.0 * SCALE;
const IMAGE_WIDTH: f32 = 184.0 * SCALE;
const IMAGE_HEIGHT: f32 = 256.0 * SCALE;
const SCALE: f32 = 1.0;

pub struct ShopWidget <Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer
{
    image: Option<image::Image<<Renderer as iced_core::image::Renderer>::Handle>>,
    title: String,
    width: f32,
    height: f32,
    on_pressed: Option<Message>,
    font: Option<<Renderer as iced_core::text::Renderer>::Font>
}

impl<Message, Renderer> ShopWidget< Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer
{

    pub fn on_press(mut self, message: Message) -> Self{
        self.on_pressed = Some(message);
        self
    }

    pub fn set_image(mut self, img: image::Image<<Renderer as iced_core::image::Renderer>::Handle>) -> Self {
        self.image = Some(img);
        self
    }
    pub fn set_font(mut self, font: <Renderer as iced_core::text::Renderer>::Font) -> Self{
        self.font = Some(font);
        self
    }

    pub fn set_title(mut self, title: String) -> Self{
        self.title = title;
        self
    }
}

impl<Message, Renderer> Default for ShopWidget <Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer
{
    fn default() -> Self {
        ShopWidget {
            image: None,
            title: "Random epic pet-egg".to_string(),
            width: DEFAULT_PRESET_WIDTH,
            height: DEFAULT_PRESET_HEIGHT,
            on_pressed: None,
            font: None
        }
    }
}

impl< Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ShopWidget<Message, Renderer>
where
    Renderer: renderer::Renderer + iced_core::text::Renderer + iced_core::image::Renderer,
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, _limits: &Limits) -> Node {
        Node::new(Size { width: self.width, height: self.height})
    }

    fn draw(&self, _tree: &Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: mouse::Cursor,
            viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: bb_theme::color::DARKER_CONTAINER_COLOR,
                    width: 1.0,
                    radius: 10.0.into(),
                },
                shadow: Shadow::default(),
            },
            bb_theme::color::CONTAINER_COLOR,
        );
        if let Some(img) = &self.image {
            renderer.draw_image(img.clone(), Rectangle{
                x: layout.bounds().x + DEFAULT_PRESET_WIDTH/2.0 - IMAGE_WIDTH/2.0,
                y: layout.bounds().y + INDENT,
                width: IMAGE_WIDTH,
                height: IMAGE_HEIGHT,
            });
        }

        renderer.fill_text(iced_core::text::Text {
            content: self.title.to_string(),
            bounds: layout.bounds().size(),
            size: iced_core::Pixels(TITLE_FONT_SIZE),
            line_height: Default::default(),
            font: self.font.unwrap_or(renderer.default_font()),
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Center,
            shaping: Default::default(),
            wrapping: Default::default(),
        }, Point {
            x: layout.bounds().x + DEFAULT_PRESET_WIDTH / 2.0,
            y: layout.bounds().y + 3.0 * INDENT + IMAGE_HEIGHT,
        }, bb_theme::color::TEXT_COLOR, *viewport);
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        if cursor.is_over(layout.bounds()) {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                    match &self.on_pressed {
                        Some(msg) => {
                            shell.publish(msg.clone());
                            event::Status::Captured
                        },
                        None => event::Status::Ignored,
                    }
                }
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
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {

        let is_mouse_over = cursor.is_over(layout.bounds());
        if is_mouse_over  {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message: 'a, Theme, Renderer> From<ShopWidget<Message, Renderer>> for Element<'a, Message, Theme, Renderer>
where Message: Clone,
      Renderer: 'a +
      iced_core::image::Renderer
      + iced_core::text::Renderer,
{
    fn from(gacha: ShopWidget<Message, Renderer>) -> Self {
        Self::new(gacha)
    }
}