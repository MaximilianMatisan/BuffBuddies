use std::collections::HashMap;
use chrono::{Datelike, Duration, Local, NaiveDate};
use iced::{Element, Task};
use iced::widget::Column;
use iced_core::{renderer, Border, Color, Layout, Length, Rectangle, Size, Theme, Widget};
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::{Quad, Style};
use iced_core::widget::Tree;
use crate::client::app::App;
use crate::client::bb_theme;
use crate::client::bb_widget::activity::date_utils::{get_date_by_offset, get_end_dates_of_offsets, get_start_dates_of_offsets, started_weeks_in_period, DateScope, Offset, DAYS_PER_WEEK};
use crate::client::mascots::Mascot;
use crate::Message;

const ACTIVITY_SQUARE_BORDER_RADIUS: f32 = 3.0;

#[derive(Debug, Clone)]
pub struct SquareDimensions {
    pub(crate) side_length: f32,
    pub(crate) spacing: f32,
    pub(crate) max_squares_per_col: u32
}

type AmountOfWorkouts = usize;
#[derive(Debug, Clone)]
pub struct ActivityWidget {
    width: f32,
    height: f32,
    current_scope: DateScope,
    current_offset: Offset,
    activity: HashMap<NaiveDate, AmountOfWorkouts>,
    //TODO zukünftig ergänzbar um map auf Vec<Exercise> / handle user-input
    active_mascot: Mascot, //TODO Update nach Message::SelectMascot(...)
    today: NaiveDate
}

#[derive(Debug, Clone, Copy)]
pub enum ActivityMessage {
    TimeScope(DateScope),
    TimeOffset(Offset)
}

impl ActivityWidget {
    pub fn new(active_mascot: Mascot) -> Self {
        let mut activity_widget = ActivityWidget {
            width: 0.0,
            height: 0.0,
            current_scope: DateScope::Year,
            current_offset: Offset::Current,
            activity: Default::default(),
            active_mascot,
            today: Local::now().date_naive()
        };
        activity_widget.width = activity_widget.compute_widget_width();
        activity_widget.height = activity_widget.compute_widget_height();

        activity_widget
    }
    pub fn update_current_mascot(&mut self, mascot: Mascot) {
        self.active_mascot = mascot
    }
    fn compute_widget_width(&self) -> f32 {
        let side_length = self.current_scope.dimensions().side_length;
        let spacing = self.current_scope.dimensions().spacing;

        if self.current_scope == DateScope::Week {
            return DAYS_PER_WEEK as f32 * side_length
                + (DAYS_PER_WEEK - 1) as f32 * spacing;
        }
        let started_weeks = started_weeks_in_period(self.start_date(), self.end_date()) as f32;

        started_weeks * side_length + (started_weeks - 1.0) * spacing
    }
    fn compute_widget_height(&self) -> f32 {
        let dimensions = self.current_scope.dimensions();

        dimensions.max_squares_per_col as f32 * dimensions.side_length
            + (dimensions.max_squares_per_col - 1) as f32 * dimensions.spacing
    }

    pub fn start_date(&self) -> NaiveDate {
        get_date_by_offset(get_start_dates_of_offsets(self.today, self.current_scope), self.current_offset)
    }
    pub fn end_date(&self) -> NaiveDate {
        get_date_by_offset(get_end_dates_of_offsets(self.today, self.current_scope), self.current_offset)
    }

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
        self.width = self.compute_widget_width();
        self.height = self.compute_widget_height();
        Task::none()
    }
    pub fn view<'a>(&self, _app: &'a App) -> Element<'a, Message> {
        //TODO
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
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &Style,
            layout: Layout<'_>,
            _cursor: Cursor,
            _viewport: &Rectangle)
    {
        let activity_square_dim: SquareDimensions = self.current_scope.dimensions();

        let first_weekday = self.start_date().weekday().num_days_from_monday();

        let mut date_iterator: NaiveDate = self.start_date();

        while date_iterator <= self.end_date() {

            let activity_color = match self.activity.get(&date_iterator){
                None | Some(0) => Color::TRANSPARENT,
                Some(_) => self.active_mascot.get_primary_color()
            };
            let activity_border = match self.activity.get(&date_iterator){
                _ if date_iterator == self.today => Border{
                    color: Color::WHITE,
                    width: 2.0,
                    radius: ACTIVITY_SQUARE_BORDER_RADIUS.into(),
                },
                None | Some(0) => Border{
                    color: bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR,
                    width: 1.0,
                    radius: ACTIVITY_SQUARE_BORDER_RADIUS.into()
                },
                Some(_) => Border{
                    color: bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR,
                    width: 0.0,
                    radius: ACTIVITY_SQUARE_BORDER_RADIUS.into()
                },
            };
            let days_from_start = (date_iterator - self.start_date()).num_days() as u32;
            let index = days_from_start + first_weekday;

            let column = (index / activity_square_dim.max_squares_per_col) as f32;
            let row = (index % activity_square_dim.max_squares_per_col) as f32;
            renderer.fill_quad(Quad {
                bounds: Rectangle {
                    x: layout.bounds().x + column *
                        (activity_square_dim.side_length
                            + activity_square_dim.spacing),
                    y: layout.bounds().y + row *
                        (activity_square_dim.side_length
                            + activity_square_dim.spacing),
                    width: activity_square_dim.side_length,
                    height: activity_square_dim.side_length
                },
                border: activity_border,
                shadow: Default::default(),
            }, activity_color);
            date_iterator += Duration::days(1);
        }
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