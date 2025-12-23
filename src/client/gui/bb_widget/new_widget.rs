use crate::bb_theme;
use crate::client::gui::bb_widget;
use iced::advanced::{
    layout::Layout,
    widget::Widget,
    {renderer, Clipboard, Shell}
};
use iced::{event, Event};
use iced::{mouse, Element};
use iced::{Length, Rectangle, Size};
use iced_core::layout::{Limits, Node};
use iced_core::widget::Tree;
use iced_core::{alignment, Border, Image, Point};

const DEFAULT_NEW_WIDGETS_WIDTH: f32 = 218.0 * SCALE;
const DEFAULT_NEW_WORKOUT_HEIGHT: f32 = 176.0 * SCALE;
const DEFAULT_NEW_PRESET_HEIGHT: f32 = 274.0 * SCALE;
const DEFAULT_TITLE_FONT_SIZE: f32 = 24.0 * SCALE;
const CONTAINER_BORDER_SIZE: f32 = 1.2;
const ADD_SYMBOL_SIZE: f32 = 86.0 * SCALE;

const SCALE: f32 = 0.9;


pub struct NewWidget<Message, Renderer>
where Renderer: iced_core::image::Renderer + iced_core::text::Renderer
{
    width: f32,
    height: f32,
    title: String,
    title_font_size: f32,
    add_symbol_size: f32,
    on_pressed: Option<Message>,
    font: Option<<Renderer as iced_core::text::Renderer>::Font>
}

impl<Message, Renderer> NewWidget<Message, Renderer>
where Renderer: iced_core::image::Renderer<Handle = iced_core::image::Handle> + iced_core::text::Renderer<Font = iced::Font>
{
    pub fn default_new_workout_widget() -> Self {
        NewWidget {
            width: DEFAULT_NEW_WIDGETS_WIDTH,
            height: DEFAULT_NEW_WORKOUT_HEIGHT,
            title: "New workout".to_string(),
            title_font_size: DEFAULT_TITLE_FONT_SIZE,
            add_symbol_size: ADD_SYMBOL_SIZE,
            on_pressed: None,
            font: Some(bb_theme::text_format::FIRA_SANS_EXTRABOLD)
        }
    }
    pub fn default_new_preset_widget() -> Self {
        NewWidget {
            width: DEFAULT_NEW_WIDGETS_WIDTH,
            height: DEFAULT_NEW_PRESET_HEIGHT,
            title: "New preset".to_string(),
            title_font_size: DEFAULT_TITLE_FONT_SIZE,
            add_symbol_size: ADD_SYMBOL_SIZE,
            on_pressed: None,
            font: Some(bb_theme::text_format::FIRA_SANS_EXTRABOLD)
        }
    }

    pub fn on_press(mut self, message: Message) -> Self{
        self.on_pressed = Some(message);
        self
    }
    pub fn set_title(mut self, title: String) -> Self{
        self.title = title;
        self
    }
}

impl< Message, Theme, Renderer> Widget<Message, Theme, Renderer> for NewWidget <Message, Renderer>
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

        let mut background = bb_widget::widget_utils::background_quad(layout);
        background.border = Border {
            color: iced::color!(146,142,142),
            width: CONTAINER_BORDER_SIZE,
            radius: 10.0.into()
        };

        renderer.fill_quad(
            background,
            bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR,
        );

        let indent_title: f32;
        if let DEFAULT_NEW_WORKOUT_HEIGHT = self.height {
            indent_title = self.height * 0.14;
        } else {
            indent_title = self.height * 0.25;
        }

        let title_start_location_y: f32 =  layout.bounds().y + indent_title + self.title_font_size;
        let add_symbol_start_location_y: f32 = layout.bounds().y + self.height * 0.5 + self.title_font_size;

        renderer.fill_text(iced_core::text::Text {  //title
            content: self.title.to_string(),
            bounds: layout.bounds().size(),
            size: iced_core::Pixels(self.title_font_size),
            line_height: Default::default(),
            font: self.font.unwrap_or(renderer.default_font()),
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Center,
            shaping: Default::default(),
            wrapping: Default::default(),
        }, Point {
            x: layout.bounds().x + self.width / 2.0,
            y: title_start_location_y,
        }, bb_theme::color::TEXT_COLOR, *viewport);


        renderer.fill_text(iced_core::text::Text {    //add symbol
            content: "+".to_string(),
            bounds: layout.bounds().size(),
            size: iced_core::Pixels(self.add_symbol_size),
            line_height: Default::default(),
            font: self.font.unwrap_or(renderer.default_font()),
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Center,
            shaping: Default::default(),
            wrapping: Default::default(),
        }, Point {
            x: layout.bounds().x + DEFAULT_NEW_WIDGETS_WIDTH * 0.5,
            y: add_symbol_start_location_y
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

impl<'a, Message: 'a, Theme, Renderer> From<NewWidget<Message, Renderer>> for Element<'a, Message, Theme, Renderer>
where Message: Clone,
      Renderer: 'a +
      iced_core::image::Renderer
      + iced_core::text::Renderer,
{
    fn from(workout: NewWidget <Message, Renderer>) -> Self {
        Self::new(workout)
    }
}