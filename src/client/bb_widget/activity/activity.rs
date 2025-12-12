use std::collections::HashMap;
use chrono::NaiveDate;
use iced::{Element, Task};
use iced::widget::Column;
use iced_core::{renderer, Layout, Length, Rectangle, Size, Theme, Widget};
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::Style;
use iced_core::widget::Tree;
use crate::client::app::App;
use crate::client::bb_widget::activity::date_utils::{DateScope, Offset};
use crate::Message;

//TODO adjust dynamically
const TEMP_WIDTH: f32 = 200.0;
const TEMP_HEIGHT: f32 = 150.0;

type AmountOfWorkouts = usize;
#[derive(Debug, Clone)]
pub struct ActivityWidget {
    width: f32,
    height: f32,
    current_scope: DateScope,
    current_offset: Offset,
    activity: HashMap<NaiveDate, AmountOfWorkouts>
    //TODO zukünftig ergänzbar um map auf Vec<Exercise> / handle user-input
}

#[derive(Debug, Clone, Copy)]
pub enum ActivityMessage {
    TimeScope(DateScope),
    TimeOffset(Offset)
}

impl Default for ActivityWidget {
    fn default() -> Self {
        ActivityWidget {
            width: TEMP_WIDTH,
            height: TEMP_HEIGHT,
            current_scope: DateScope::Month,
            current_offset: Offset::Current,
            activity: HashMap::new(),
        }
    }
}

impl ActivityWidget {
    pub fn update(&mut self, message: ActivityMessage) -> Task<Message> {
        match message {
            ActivityMessage::TimeScope(scope) => {
                self.current_scope = scope;
                self.current_offset = Offset::Current;
            }
            ActivityMessage::TimeOffset(offset) => {
                self.current_offset = offset;
            }
        }
        Task::none()
    }
    pub fn view<'a>(&self, _app: &'a App) -> Element<'a, Message> {
        Column::new().into()
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for ActivityWidget
where
    Renderer: renderer::Renderer,
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size {width: Length::Fixed(self.width), height: Length::Fixed(self.height)}
    }

    fn layout(&self,
              _tree: &mut Tree,
              _renderer: &Renderer,
              _limits: &Limits) -> Node
    {
        Node::new(Size { width: self.width, height: self.height })
    }

    fn draw(&self,
            _tree: &Tree,
            _renderer: &mut Renderer,
            _theme: &Theme,
            _style: &Style,
            _layout: Layout<'_>,
            _cursor: Cursor,
            _viewport: &Rectangle)
    {
        //TODO
    }
}

impl<'a, Message: 'a, Renderer> From<ActivityWidget> for Element<'a, Message, Theme, Renderer>
where Message: Clone,
      Renderer: 'a + renderer::Renderer
{
    fn from(activity_widget: ActivityWidget) -> Self {
        Self::new(activity_widget)
    }
}