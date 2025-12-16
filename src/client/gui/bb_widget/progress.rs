use chrono::NaiveDate;
use crate::client::gui::bb_theme::color;
use crate::client::gui::mascots::Mascot;
use iced::{Element};
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::{Quad, Style};
use iced_core::widget::Tree;
use iced_core::{renderer, text, Border, Layout, Length, Point, Rectangle, Size, Text, Theme, Widget};
use iced_core::alignment::{Horizontal, Vertical};
use crate::client::backend::exercise::weight::Kg;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;

const PROGRESS_WIDGET_WIDTH: f32 = 500.0;
const PROGRESS_WIDGET_HEIGHT: f32 = 500.0;
pub struct ProgressWidget<Renderer>
    where Renderer: text::Renderer
{
    width: f32,
    height: f32,
    active_mascot: Mascot,
    data_points: Vec<(NaiveDate, Kg)>,
    font: <Renderer>::Font
}
impl<Renderer> ProgressWidget<Renderer>
    where Renderer: text::Renderer<Font = iced::Font>
{
    pub fn new(active_mascot: Mascot, data_points: Vec<(NaiveDate, Kg)>) -> Self {
        ProgressWidget {
            width: PROGRESS_WIDGET_WIDTH,
            height: PROGRESS_WIDGET_HEIGHT,
            active_mascot,
            data_points,
            font: bb_theme::text_format::FIRA_SANS_EXTRABOLD,
        }
    }
    pub fn update_data_points(&mut self, data_points: Vec<(NaiveDate, Kg)>) {
        self.data_points = data_points;
    }
    pub fn update_active_mascot(&mut self, mascot: Mascot) {
        self.active_mascot = mascot;
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for ProgressWidget<Renderer>
where
    Renderer: renderer::Renderer + text::Renderer,
    Message: Clone
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(&self,
              _tree: &mut Tree,
              _renderer: &Renderer,
              _limits: &Limits) -> Node
    {
        Node::new(Size::new(self.width, self.height))
    }

    fn draw(&self,
            _tree: &Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &Style,
            layout: Layout<'_>,
            _cursor: Cursor,
            viewport: &Rectangle)
    {
       renderer.fill_quad(Quad { //TODO TEMP
           bounds: layout.bounds(),
           border: Border {
               color: Default::default(),
               width: 0.0,
               radius: DEFAULT_CONTAINER_RADIUS.into()
           },
           shadow: Default::default(),
       }, color::CONTAINER_COLOR);

        println!("{:?}", self.data_points);
        match &self.data_points.len() {
            0  => {
                renderer.fill_text(Text{
                    content: "NO DATA".to_string(),
                    bounds: layout.bounds().size(),
                    size: 40.into(),
                    line_height: Default::default(),
                    font: self.font,
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    shaping: Default::default(),
                    wrapping: Default::default(),
                }, Point {
                    x: layout.bounds().center_x(),
                    y: layout.bounds().center_y(),
                }, color::DESCRIPTION_TEXT_COLOR, *viewport)
            },
            _ => {}
        }
    }
}
impl<'a, Message: 'a, Renderer> From<ProgressWidget<Renderer>> for Element<'a, Message, Theme, Renderer>
where Message: Clone,
      Renderer: 'a + renderer::Renderer + text::Renderer
{
    fn from(value: ProgressWidget<Renderer>) -> Self {
        Self::new(value)
    }
}
