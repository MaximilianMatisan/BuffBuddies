use chrono::NaiveDate;
use crate::client::gui::bb_theme::color;
use crate::client::gui::mascots::Mascot;
use iced::{Element};
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::{Quad, Style};
use iced_core::widget::Tree;
use iced_core::{renderer, Layout, Length, Rectangle, Size, Theme, Widget};
use crate::client::backend::exercise::weight::Kg;

const PROGRESS_WIDGET_WIDTH: f32 = 500.0;
const PROGRESS_WIDGET_HEIGHT: f32 = 500.0;
pub struct ProgressWidget {
    width: f32,
    height: f32,
    active_mascot: Mascot,
    data_points: Option<Vec<(NaiveDate, Kg)>>
}
impl ProgressWidget {
    pub fn new(active_mascot: Mascot, data_points: Option<Vec<(NaiveDate, Kg)>>) -> Self {
        ProgressWidget {
            width: PROGRESS_WIDGET_WIDTH,
            height: PROGRESS_WIDGET_HEIGHT,
            active_mascot,
            data_points
        }
    }
    pub fn update_data_points(&mut self, data_points: Option<Vec<(NaiveDate, Kg)>>) {
        self.data_points = data_points;
    }
    pub fn update_active_mascot(&mut self, mascot: Mascot) {
        self.active_mascot = mascot;
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for ProgressWidget
where
    Renderer: renderer::Renderer,
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
            _viewport: &Rectangle)
    {
       renderer.fill_quad(Quad { //TODO TEMP
           bounds: layout.bounds(),
           border: Default::default(),
           shadow: Default::default(),
       }, color::CONTAINER_COLOR);
    }
}
impl<'a, Message: 'a, Renderer> From<ProgressWidget> for Element<'a, Message, Theme, Renderer>
where Message: Clone,
      Renderer: 'a + renderer::Renderer
{
    fn from(value: ProgressWidget) -> Self {
        Self::new(value)
    }
}
