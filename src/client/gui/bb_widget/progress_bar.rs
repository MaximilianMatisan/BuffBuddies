use iced::widget::{canvas, ProgressBar};
use iced::widget::canvas::{Cache, Frame, Geometry, Path, event};
use iced::{Element, Rectangle, Renderer, Size, Theme};
use iced::{Task, mouse};
use iced_anim::{Animated, Animation, Event, Motion};
use iced_core::{Color, Point, color};
use std::time::Duration;

use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::{CONTAINER_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_widget::canvas_utils::{create_arc_path, draw_text, generate_stroke};
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::epic_mascot::EpicMascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::mascot_mod::rare_mascot::RareMascot;
use crate::common::user_mod::user::UserInformation;
use iced::advanced::text::{Renderer as TextRenderer, Text};
use iced_core::alignment::{Horizontal, Vertical};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;

const PROGRESS_BAR_WIDGET_WIDTH: f32 = 700.0;
const PROGRESS_BAR_WIDGET_HEIGHT: f32 = 83.0;
const PADDING_X: f32 = 31.0;
const PADDING_Y: f32 = 15.0;
const BAR_THICKNESS: f32 = 15.0;
const PADDING_BETWEEN_BARS: f32 = 40.0;
const PROGRESS_BAR_FONT_SIZE_STATS: f32 = 26.0;
const PROGRESS_BAR_TITLE_FONT_SIZE: f32 = 24.0;

//-------Variables used my multiple methods
static Y_POSITION_BAR: f32 = PADDING_Y + PROGRESS_BAR_TITLE_FONT_SIZE + (PROGRESS_BAR_WIDGET_HEIGHT - PADDING_Y - PROGRESS_BAR_TITLE_FONT_SIZE) / 2.0;

pub struct ProgressBarWidget<'a> {
    progress_bar_state : &'a ProgressBarState,
    title: String,
    current_value: f32,
    goal_value: f32,
    unit: String

}

#[derive(Clone, Debug)]
pub enum ProgressBarMessage {
    UpdateProgressBarAnimation(Event<f32>),
}

impl ProgressBarMessage {
    pub fn update_progress_bar_message(self, app: &mut App) -> Task<Message> {
        match self {
            ProgressBarMessage::UpdateProgressBarAnimation(event) => {
                app.widget_manager
                    .progress_bar_state
                    .animation_progress
                    .update(event);
                app.widget_manager.progress_bar_state.update_circle();
                Task::none()
            }
        }
    }
}

impl<'a> ProgressBarWidget<'a> {
    pub(crate) fn new(app: &'a App) -> Self {
        ProgressBarWidget {
            progress_bar_state: &app.widget_manager.progress_bar_state,
            title: "Water".to_string(),
            current_value: 1.0,
            goal_value: 3.5,
            unit: "L".to_string(),
        }
    }
    pub(crate) fn view(self) -> Element<'a, Message> {
        let draw_percentage = &self.progress_bar_state.animation_progress;

        let canvas = canvas(self)
            .width(PROGRESS_BAR_WIDGET_WIDTH)
            .height(PROGRESS_BAR_WIDGET_HEIGHT);

        Animation::new(draw_percentage, canvas)
            .on_update(|event| Message::ProgressBar(ProgressBarMessage::UpdateProgressBarAnimation(event)))
            .into()
    }
}

pub struct ProgressBarState {
    progress_bar: Cache,
    pub animation_progress: Animated<f32>,
}

impl ProgressBarState {
    pub(crate) fn update_circle(&self) {
        self.progress_bar.clear();
    }
}

impl ProgressBarState {
    pub fn new() -> Self {
        let animation_motion = Motion {
            response: Duration::from_millis(3000),
            damping: Motion::SMOOTH.damping(),
        };

        Self {
            progress_bar: Cache::default(),
            animation_progress: Animated::new(0.0, animation_motion),
        }
    }
}

impl Default for ProgressBarState {
    fn default() -> Self {
        Self::new()
    }
}

impl canvas::Program<Message> for ProgressBarWidget<'_> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: iced::widget::canvas::Event,
        _bounds: Rectangle,
        _cursor: iced_core::mouse::Cursor,
    ) -> (iced::event::Status, std::option::Option<Message>) {
        self.progress_bar_state.update_circle();

        (
            event::Status::Ignored,
            Some(crate::client::gui::user_interface::Message::ProgressBar(
                ProgressBarMessage::UpdateProgressBarAnimation(iced_anim::Event::Target(1.0)),
            )),
        )
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let progress_bar_geometry = self
            .progress_bar_state
            .progress_bar
            .draw(renderer, bounds.size(), |frame| {

                //DRAW BACKGORUND
                draw_background(frame);

                //DRAW TEXT BMI VALUE
                draw_progress_bar_title(frame,self);

                //DRAW BAR REPRESENTING COMPLETED PERCENTAGE
                draw_bar_completion(frame, self);

                //DRAW BAR REPRESENTING UNCOMPLETED PERCENTAGE
                draw_bar_remaining(frame, self);

                //DRAW TEXT BMI VALUE
                draw_progress_bar_values_text(frame, self)
            });

        vec![progress_bar_geometry]
    }
}
fn draw_background(frame: &mut Frame) {
    let background_size = Path::rounded_rectangle(
        Point::ORIGIN, //START
        Size {
            width: frame.width(),
            height: frame.height(),
        },
        DEFAULT_CONTAINER_RADIUS.into(),
    );

    frame.fill(&background_size, CONTAINER_COLOR);
}

fn draw_progress_bar_title(frame: &mut Frame,  progress_bar_widget: &ProgressBarWidget) {
    let position_title = Point{
        x: Point::ORIGIN.x + PADDING_X,
        y: Point::ORIGIN.y + PADDING_Y,
    };

    //draw_text(frame, progress_bar_widget.title.clone(), PROGRESS_BAR_TITLE_FONT_SIZE, position_title)

    frame.fill_text(canvas::Text {
        content: progress_bar_widget.title.clone(),
        size: PROGRESS_BAR_TITLE_FONT_SIZE.into(),
        position: position_title,
        color: TEXT_COLOR,
        font: FIRA_SANS_EXTRABOLD,
        horizontal_alignment: Horizontal::Left,
        vertical_alignment: Vertical::Top,
        line_height: Default::default(),
        shaping: iced_core::text::Shaping::Advanced,
    });
}

fn draw_bar_completion(frame: &mut Frame, progress_bar_widget: &ProgressBarWidget) {
    let start_point =
        Point{
            x: Point::ORIGIN.x + PADDING_X,
            y: Point::ORIGIN.y + Y_POSITION_BAR,
        };

    let end_point =
        Point{
            x: Point::ORIGIN.x + PADDING_X + calculate_length_completion_bar(progress_bar_widget),
            y: Point::ORIGIN.y + Y_POSITION_BAR,
        };

    frame.stroke(&Path::line(start_point,end_point),generate_stroke(BAR_THICKNESS, RareMascot::Whale.get_primary_color()))

}
fn draw_bar_remaining(frame: &mut Frame, progress_bar_widget: &ProgressBarWidget) {
    if progress_bar_widget.current_value < progress_bar_widget.goal_value {
        let start_point =
            Point {
                x: Point::ORIGIN.x + PADDING_X + calculate_length_completion_bar(progress_bar_widget) + PADDING_BETWEEN_BARS / 2.0,
                y: Point::ORIGIN.y + Y_POSITION_BAR,
            };

        let end_point =
            Point {
                x: PROGRESS_BAR_WIDGET_WIDTH - PADDING_X,
                y: Point::ORIGIN.y + Y_POSITION_BAR,
            };

        frame.stroke(&Path::line(start_point, end_point), generate_stroke(BAR_THICKNESS, RareMascot::Whale.get_secondary_color()))
    }
}

fn draw_progress_bar_values_text(frame: &mut Frame, progress_bar_widget: &ProgressBarWidget) {

    let values_text = format!("{}/{} {}", progress_bar_widget.current_value, progress_bar_widget.goal_value, progress_bar_widget.unit);

    let position_values = Point{
        x: Point::ORIGIN.x + PROGRESS_BAR_WIDGET_WIDTH - PADDING_X,
        y: Point::ORIGIN.y + PADDING_Y + PROGRESS_BAR_TITLE_FONT_SIZE / 2.0,
    };

    frame.fill_text(canvas::Text {
        content: values_text,
        size: PROGRESS_BAR_FONT_SIZE_STATS.into(),
        position: position_values,
        color: TEXT_COLOR,
        font: FIRA_SANS_EXTRABOLD,
        horizontal_alignment: Horizontal::Right,
        vertical_alignment: Vertical::Center,
        line_height: Default::default(),
        shaping: iced_core::text::Shaping::Advanced,
    });
    //draw_text(frame, values_text, PROGRESS_BAR_TITLE_FONT_SIZE, position_values)
}

//LOGIC

fn calculate_length_completion_bar(progress_bar_widget: &ProgressBarWidget) -> f32 {

    let total_possible_bar_length = PROGRESS_BAR_WIDGET_WIDTH - PADDING_X * 2.0;
    let percentage = progress_bar_widget.current_value / progress_bar_widget.goal_value;
    let padding_to_other_bar = match percentage {
        1.0 => 0.0,
        0.0 => 0.0,
        _ => PADDING_BETWEEN_BARS / 2.0
    };

    total_possible_bar_length * percentage - padding_to_other_bar
}
