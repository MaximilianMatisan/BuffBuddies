use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Frame, Geometry, Path, event};
use iced::{Element, Rectangle, Renderer, Size, Theme};
use iced_anim::{Animated, Animation, Event, Motion};
use iced_core::Point;
use std::time::Duration;

use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::CONTAINER_COLOR;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_widget::canvas_utils::{create_arc_path, draw_text, generate_stroke};
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;

const CIRCLE_WIDGET_WIDTH: f32 = 250.0;
const CIRCLE_WIDGET_HEIGHT: f32 = 250.0;
const CIRCLE_RADIUS: f32 = 90.0;
const PADDING_BETWEEN_ARCS: f32 = 25.0;
const FONT_SIZE_RATIO: f32 = 24.0;
const FONT_SIZE_DESCRIPTION: f32 = 17.0;
// Arc angles are defined as clockwise rotations starting from the positive X-axis.
// For our use case, it is more intuitive to measure angles clockwise from the positive Y-axis
// This offset converts between the two coordinate systems.
const DEGREE_START_TRANSLATION: f32 = -90.0;

pub struct CircleWidget<'a> {
    active_mascot: Mascot,
    circle_widget_state: &'a CircleWidgetState,
    completed_exercises: u32,
    total_exercises: u32,
}

#[derive(Clone, Debug)]
pub enum CircleMessage {
    UpdateCircleAnimation(Event<f32>),
}

impl<'a> CircleWidget<'a> {
    pub(crate) fn new(app: &'a App) -> Self {
        CircleWidget {
            active_mascot: app.mascot_manager.selected_mascot,
            circle_widget_state: &app.circle_widget_state,
            completed_exercises: 2,
            total_exercises: app.user_manager.user_info.weekly_workout_goal, //CANNOT BE ZERO OR ELSE APP CRASHES
        }
    }
    pub(crate) fn view(self) -> Element<'a, Message> {
        let draw_percentage = &self.circle_widget_state.animation_progress;

        let canvas = canvas(self)
            .width(CIRCLE_WIDGET_WIDTH)
            .height(CIRCLE_WIDGET_HEIGHT);

        Animation::new(draw_percentage, canvas)
            .on_update(|event| Message::Circle(CircleMessage::UpdateCircleAnimation(event)))
            .into()
    }
}
pub struct CircleWidgetState {
    circle: Cache,
    pub animation_progress: Animated<f32>,
}

impl CircleWidgetState {
    pub(crate) fn update_circle(&self) {
        self.circle.clear();
    }
}

impl CircleWidgetState {
    pub fn new() -> Self {
        let animation_motion = Motion {
            response: Duration::from_millis(3000),
            damping: Motion::SMOOTH.damping(),
        };

        Self {
            circle: Cache::default(),
            animation_progress: Animated::new(0.0, animation_motion),
        }
    }
}

impl Default for CircleWidgetState {
    fn default() -> Self {
        Self::new()
    }
}

impl canvas::Program<Message> for CircleWidget<'_> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: iced::widget::canvas::Event,
        _bounds: Rectangle,
        _cursor: iced_core::mouse::Cursor,
    ) -> (iced::event::Status, std::option::Option<Message>) {
        self.circle_widget_state.update_circle();

        (
            event::Status::Ignored,
            Some(crate::client::gui::user_interface::Message::Circle(
                CircleMessage::UpdateCircleAnimation(iced_anim::Event::Target(1.0)),
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
        let circle_widget =
            self.circle_widget_state
                .circle
                .draw(renderer, bounds.size(), |frame| {
                    let circle_center = frame.center();

                    //DRAW BACKGORUND
                    draw_background(frame);

                    //DRAW ARC SHOWING COMPLETED EXERCISES
                    draw_arc_completed_exercises(frame, circle_center, self);

                    //DRAW ARC SHOWING UNCOMPLETED EXERCISES
                    draw_arc_not_completed_exercises(frame, circle_center, self);

                    //DRAW TEXT COMPLETED/TOTAL EXERCISES
                    draw_circle_text(frame, self)
                });

        vec![circle_widget]
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

fn draw_arc_completed_exercises(
    frame: &mut Frame,
    center_of_circle: Point,
    circle_widget: &CircleWidget,
) {
    if circle_widget.completed_exercises > 0 {
        let start_angle = DEGREE_START_TRANSLATION
            + PADDING_BETWEEN_ARCS / 2.0
                * circle_widget.circle_widget_state.animation_progress.value();

        let end_angle = DEGREE_START_TRANSLATION
            + (convert_done_exercises_in_degrees(
                circle_widget.completed_exercises,
                circle_widget.total_exercises,
            ) - PADDING_BETWEEN_ARCS / 2.0)
                * circle_widget.circle_widget_state.animation_progress.value();

        let arc_path = &create_arc_path(center_of_circle, CIRCLE_RADIUS, start_angle, end_angle);

        frame.stroke(
            arc_path,
            generate_stroke(20.0, circle_widget.active_mascot.get_primary_color()),
        );
    }
}

fn draw_arc_not_completed_exercises(
    frame: &mut Frame,
    center_of_circle: Point,
    circle_widget: &CircleWidget,
) {
    if circle_widget.total_exercises > circle_widget.completed_exercises {
        let start_angle = DEGREE_START_TRANSLATION
            + (convert_done_exercises_in_degrees(
                circle_widget.completed_exercises,
                circle_widget.total_exercises,
            ) + PADDING_BETWEEN_ARCS / 2.0)
                * circle_widget.circle_widget_state.animation_progress.value();

        let end_angle = DEGREE_START_TRANSLATION
            + (360.0 - PADDING_BETWEEN_ARCS / 2.0)
                * circle_widget.circle_widget_state.animation_progress.value();

        let arc_path = &create_arc_path(center_of_circle, CIRCLE_RADIUS, start_angle, end_angle);

        frame.stroke(
            arc_path,
            generate_stroke(20.0, circle_widget.active_mascot.get_secondary_color()),
        );
    }
}

fn draw_circle_text(frame: &mut Frame, circle_widget: &CircleWidget) {
    let circle_center = frame.center();
    let text_padding = 5.0;
    let content_text = format!(
        "{} / {}",
        circle_widget.completed_exercises, circle_widget.total_exercises
    );

    draw_text(
        frame,
        content_text,
        FONT_SIZE_RATIO,
        Point {
            x: circle_center.x,
            y: circle_center.y - FONT_SIZE_RATIO - text_padding,
        },
    );

    draw_text(
        frame,
        String::from("workouts"),
        FONT_SIZE_DESCRIPTION + 4.0,
        circle_center,
    );

    draw_text(
        frame,
        String::from("this week"),
        FONT_SIZE_DESCRIPTION,
        Point {
            x: circle_center.x,
            y: circle_center.y + FONT_SIZE_RATIO + text_padding,
        },
    );
}

//LOGIC

fn convert_done_exercises_in_degrees(completed_exercises: u32, total_exercises: u32) -> f32 {
    let ratio = completed_exercises as f32 / total_exercises as f32;

    match ratio {
        ..=1.0 => ratio * 360.0,
        _ => 360.0, //NEVER BECOMES A FULL CIRCLE THANKS TO THIS LIMIT AND THE PADDING_BETWEEN_ARCS
    }
}
