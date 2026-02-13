use crate::client::gui::app::App;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::container::ContainerStyle;
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, DEFAULT_BUTTON_RADIUS};
use crate::client::gui::bb_theme::{color, custom_button};
use crate::client::gui::bb_widget::activity_widget::date_utils::{
    DAYS_PER_WEEK, DateScope, Offset, get_date_by_offset, get_end_dates_of_offsets,
    get_start_dates_of_offsets, started_weeks_in_period,
};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use chrono::{Datelike, Duration, Local, NaiveDate};
use iced::widget::{Column, container, row};
use iced::{Element, Point, Task};
use iced_core::alignment::Vertical;
use iced_core::border::Radius;
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::{Quad, Style};
use iced_core::widget::Tree;
use iced_core::{
    Border, Color, Layout, Length, Rectangle, Size, Text, Theme, Widget, renderer, text,
};
use std::collections::HashMap;
use strum::IntoEnumIterator;

const DEFAULT_NAVIGATION_BUTTON_WIDTH: f32 = 130.0;
const DEFAULT_NAVIGATION_BUTTON_HEIGHT: f32 = 40.0;
pub const ACTIVITY_SQUARE_BORDER_RADIUS: f32 = 3.0;
const TIME_TEXT_HEIGHT: f32 = 12.0;
pub type ActivityData = HashMap<NaiveDate, AmountOfSets>;

#[derive(Debug, Clone)]
pub struct SquareDimensions {
    pub(crate) side_length: f32,
    pub(crate) spacing: f32,
    pub(crate) max_squares_per_col: u32,
}

pub type AmountOfSets = u32;
#[derive(Debug, Clone)]
pub struct ActivityWidget {
    width: f32,
    height: f32,
    current_scope: DateScope,
    current_offset: Offset,
    activity: ActivityData,
    active_mascot: Mascot,
    today: NaiveDate,
}

#[derive(Debug, Clone, Copy)]
pub enum ActivityMessage {
    TimeScope(DateScope),
    TimeOffset(Offset),
}

impl ActivityWidget {
    pub fn new(active_mascot: Mascot, activity: ActivityData) -> Self {
        let mut activity_widget = ActivityWidget {
            width: 0.0,
            height: 0.0,
            current_scope: DateScope::Month,
            current_offset: Offset::Current,
            activity,
            active_mascot,
            today: Local::now().date_naive(),
        };
        activity_widget.width = activity_widget.compute_widget_width();
        activity_widget.height = activity_widget.compute_widget_height();

        activity_widget
    }
    pub fn update_active_mascot(&mut self, mascot: Mascot) {
        self.active_mascot = mascot
    }
    pub fn update_data(&mut self, mascot: Mascot, activity_data: ActivityData) {
        self.active_mascot = mascot;
        self.activity = activity_data;
    }
    fn compute_widget_width(&self) -> f32 {
        let side_length = self.current_scope.dimensions().side_length;
        let spacing = self.current_scope.dimensions().spacing;

        if self.current_scope == DateScope::Week {
            return DAYS_PER_WEEK as f32 * side_length + (DAYS_PER_WEEK - 1) as f32 * spacing;
        }
        let started_weeks = started_weeks_in_period(self.start_date(), self.end_date()) as f32;

        started_weeks * side_length + (started_weeks - 1.0) * spacing
    }
    fn compute_widget_height(&self) -> f32 {
        let dimensions = self.current_scope.dimensions();

        let mut base = dimensions.max_squares_per_col as f32 * dimensions.side_length
            + (dimensions.max_squares_per_col - 1) as f32 * dimensions.spacing;

        if self.current_scope == DateScope::Year {
            base += TIME_TEXT_HEIGHT;
        }
        base
    }

    pub fn start_date(&self) -> NaiveDate {
        get_date_by_offset(
            get_start_dates_of_offsets(self.today, self.current_scope),
            self.current_offset,
        )
    }
    pub fn end_date(&self) -> NaiveDate {
        get_date_by_offset(
            get_end_dates_of_offsets(self.today, self.current_scope),
            self.current_offset,
        )
    }
    pub fn offset_title(&self, offset: Offset) -> String {
        let start_dates = get_start_dates_of_offsets(self.today, self.current_scope);
        let start_date = get_date_by_offset(start_dates, offset);
        match self.current_scope {
            DateScope::Year => start_date.year().to_string(),
            DateScope::Month => start_date.format("%B").to_string(),
            DateScope::Week => format!("Week {}", start_date.iso_week().week()),
        }
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
    pub fn view<'a>(&self, app: &'a App) -> Element<'a, Message> {
        let mut time_scope_buttons: Column<Message> = Column::new();

        let time_scope_border_radius = Radius {
            top_left: 0.0,
            top_right: DEFAULT_BUTTON_RADIUS,
            bottom_right: DEFAULT_BUTTON_RADIUS,
            bottom_left: 0.0,
        };

        for time in DateScope::iter() {
            let style_of_button = if self.current_scope == time {
                ButtonStyle::Active
            } else {
                ButtonStyle::InactiveSolid
            };
            let width_of_button = if self.current_scope == time {
                DEFAULT_NAVIGATION_BUTTON_WIDTH
            } else {
                DEFAULT_NAVIGATION_BUTTON_WIDTH - 20.0
            };

            time_scope_buttons = time_scope_buttons.push(
                custom_button::create_text_button(
                    &self.active_mascot,
                    time.to_string(),
                    style_of_button,
                    Some(time_scope_border_radius),
                )
                .width(width_of_button)
                .height(DEFAULT_NAVIGATION_BUTTON_HEIGHT)
                .on_press(Message::Activity(ActivityMessage::TimeScope(time))),
            );
        }

        let mut time_offset_buttons: Column<Message> = Column::new();

        let offset_button_width = match self.current_scope {
            DateScope::Year => Length::Shrink,
            DateScope::Week | DateScope::Month => Length::Fixed(DEFAULT_NAVIGATION_BUTTON_WIDTH),
        };

        for offset in Offset::iter() {
            let style_of_button = if self.current_offset == offset {
                ButtonStyle::Active
            } else {
                ButtonStyle::InactiveTransparent
            };

            time_offset_buttons = time_offset_buttons.push(
                custom_button::create_text_button(
                    &self.active_mascot,
                    self.offset_title(offset),
                    style_of_button,
                    None,
                )
                .width(offset_button_width)
                .height(DEFAULT_NAVIGATION_BUTTON_HEIGHT)
                .on_press(Message::Activity(ActivityMessage::TimeOffset(offset))),
            );
        }
        time_offset_buttons = time_offset_buttons.spacing(INDENT);

        let widget_offset_container = container(
            row![
                app.widget_manager.activity_widget.clone(), //TODO ohne clone?
                time_offset_buttons
            ]
            .spacing(10)
            .align_y(Vertical::Center),
        )
        .style(bb_theme::container::create_container_style(
            ContainerStyle::Default,
            None,
            None,
        ))
        .padding(INDENT);

        row![widget_offset_container, time_scope_buttons.spacing(10)]
            .align_y(Vertical::Center)
            .into()
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for ActivityWidget
where
    Renderer: renderer::Renderer + text::Renderer,
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(self.width),
            height: Length::Fixed(self.height),
        }
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
        let activity_square_dim: SquareDimensions = self.current_scope.dimensions();

        let first_weekday = self.start_date().weekday().num_days_from_monday();

        let mut date_iterator: NaiveDate = self.start_date();

        while date_iterator <= self.end_date() {
            let activity_color = match self.activity.get(&date_iterator) {
                None | Some(0) => Color::TRANSPARENT,
                Some(_) => self.active_mascot.get_primary_color(),
            };

            let activity_border = match self.activity.get(&date_iterator) {
                _ if date_iterator == self.today => Border {
                    color: Color::WHITE,
                    width: 2.0,
                    radius: ACTIVITY_SQUARE_BORDER_RADIUS.into(),
                },
                None | Some(0) => Border {
                    color: color::HIGHLIGHTED_CONTAINER_COLOR,
                    width: 1.0,
                    radius: ACTIVITY_SQUARE_BORDER_RADIUS.into(),
                },
                Some(_) => Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: ACTIVITY_SQUARE_BORDER_RADIUS.into(),
                },
            };
            let days_from_start = (date_iterator - self.start_date()).num_days() as u32;
            let index = days_from_start + first_weekday;

            let column = (index / activity_square_dim.max_squares_per_col) as f32;
            let row = (index % activity_square_dim.max_squares_per_col) as f32;
            let cur_x = layout.bounds().x
                + column * (activity_square_dim.side_length + activity_square_dim.spacing);
            let cur_y = layout.bounds().y
                + row * (activity_square_dim.side_length + activity_square_dim.spacing);
            renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: cur_x,
                        y: cur_y,
                        width: activity_square_dim.side_length,
                        height: activity_square_dim.side_length,
                    },
                    border: activity_border,
                    shadow: Default::default(),
                },
                activity_color,
            );
            if self.current_scope == DateScope::Year
                && (date_iterator - Duration::days(1)).month() != date_iterator.month()
            {
                renderer.fill_text(
                    Text {
                        content: date_iterator.format("%b").to_string(),
                        bounds: layout.bounds().size(),
                        size: TIME_TEXT_HEIGHT.into(),
                        line_height: Default::default(),
                        font: renderer.default_font(),
                        horizontal_alignment: iced::alignment::Horizontal::Left,
                        vertical_alignment: Vertical::Top,
                        shaping: Default::default(),
                        wrapping: Default::default(),
                    },
                    Point {
                        x: cur_x,
                        y: layout.bounds().y + self.height - TIME_TEXT_HEIGHT,
                    },
                    color::DESCRIPTION_TEXT_COLOR,
                    *viewport,
                );
            }
            date_iterator += Duration::days(1);
        }
    }
}

impl<'a, Message: 'a, Renderer> From<ActivityWidget> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: 'a + renderer::Renderer + text::Renderer,
{
    fn from(activity_widget: ActivityWidget) -> Self {
        Self::new(activity_widget)
    }
}

pub fn calculate_activity_data(exercise_data: &Vec<Exercise>) -> ActivityData {
    let mut map: ActivityData = HashMap::new();

    for exercise in exercise_data {
        for (date, set) in &exercise.sets {
            map.entry(*date)
                .and_modify(|entry| *entry += set.len() as AmountOfSets)
                .or_insert(set.len() as AmountOfSets);
        }
    }
    map
}
