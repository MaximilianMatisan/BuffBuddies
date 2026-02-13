use std::time::Duration;
use iced::alignment;
use iced::mouse;
use iced::widget::canvas::{Cache, Geometry, LineCap, Path, Stroke, stroke, LineJoin, LineDash, event, Frame};
use iced::widget::{canvas, container, text};
use iced::{
    Degrees, Element, Fill, Font, Radians, Rectangle, Renderer, Size, Subscription, Theme,
    Vector,
};
use iced::advanced::graphics::text::cosmic_text::Shaping;
use iced::widget::canvas::path::Arc;
use iced_anim::{Animated, Animation, Event, Motion};
use iced_core::{color, renderer, Point};
use iced_core::alignment::{Horizontal, Vertical};
use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::{CONTAINER_COLOR, HIGHLIGHTED_CONTAINER_COLOR};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::canvas_utils::generate_stroke;
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;

const CIRCLE_WIDGET_WIDTH: f32 = 300.0;
const CIRCLE_WIDGET_HEIGHT: f32 = 300.0;
const CIRCLE_RADIUS: f32  = 80.0;

const PADDING_BETWEEN_ARCS: f32 = 25.0;

// Arc angles are defined as clockwise rotations starting from the positive X-axis.
// For our use case, it is more intuitive to measure angles clockwise from the positive Y-axis
// This offset converts between the two coordinate systems.
const DEGREE_START_TRANSLATION: f32 = -90.0;


pub struct CircleWidget<'a> {
    active_mascot: Mascot,
    exercise_manager: &'a ExerciseManager,
    circle_widget_state: &'a CircleWidgetState,
    completed_exercises: u32,
    total_exercises: u32
}

#[derive(Clone, Debug)]
pub enum  CircleMessage{
    UpdateCirlceAnimation(Event<f32>)
}

impl<'a> CircleWidget<'a> {
    pub(crate) fn new(app: &'a App) -> Self {

        CircleWidget {
            active_mascot: app.mascot_manager.selected_mascot,
            exercise_manager: &app.exercise_manager,
            circle_widget_state: &app.circle_widget_state,
            completed_exercises: 3,
            total_exercises: app.user_manager.user_info.weekly_workout_goal  //CANNOT BE ZERO OR ELSE APP CRASHES
        }
    }
    pub(crate) fn view (self) -> Element<'a, Message> {

        let draw_percentage = &self.circle_widget_state.animation_progress;

        let canvas = canvas(self).width(CIRCLE_WIDGET_WIDTH).height(CIRCLE_WIDGET_HEIGHT);

        Animation::new(draw_percentage, canvas)
            .on_update(|event| Message::Circle(CircleMessage::UpdateCirlceAnimation(event)))
            .into()

    }
}
pub struct CircleWidgetState {
    width: f32,
    height: f32,
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
            width: CIRCLE_WIDGET_WIDTH,
            height:CIRCLE_WIDGET_HEIGHT,
            circle: Cache::default(),
            animation_progress: Animated::new(0.0, animation_motion)
        }
    }
}

impl canvas::Program<Message> for CircleWidget<'_> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: iced::widget::canvas::Event,
        _bounds: Rectangle,
        _cursor: iced_core::mouse::Cursor,
    ) -> (iced::event::Status, std::option::Option<Message>) {
        self.circle_widget_state.update_circle();

        match event {

            _ => (event::Status::Ignored,
                Some(crate::client::gui::user_interface::Message::Circle(CircleMessage::UpdateCirlceAnimation(
                    iced_anim::Event::Target(1.0)

            ))))
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        use chrono::Timelike;

        let circle_widget = self.circle_widget_state.circle.draw(renderer, bounds.size(), |frame| {

            let circle_center = frame.center();

            //DRAW BACKGORUND
            draw_background(frame);

            //DRAW ARC SHOWING COMPLETED EXERCISES
            draw_arc_completed_exercises(frame, circle_center, self);


            //DRAW ARC SHOWING UNCOMPLETED EXERCISES
            draw_arc_not_completed_exercises(frame,circle_center,self);


            //DRAW TEXT COMPLETED/TOTAL EXERCISES

            frame.fill_text(canvas::Text {
                content: format!("{}/{}",self.completed_exercises,self.total_exercises),
                size: 30.0.into(),
                position: circle_center,
                color: color!(255,255,255),
                font: FIRA_SANS_EXTRABOLD,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: Default::default(),
                shaping: iced_core::text::Shaping::Advanced,
            });


            frame.fill_text(canvas::Text {
                content: String::from("Workouts this week"),
                size: 20.0.into(),
                position:
                Point{
                    x: circle_center.x,
                    y: circle_center.y - CIRCLE_RADIUS - 32.0,
                },
                color: color!(255,255,255),
                font: FIRA_SANS_EXTRABOLD,
                horizontal_alignment: Horizontal::Center,
                vertical_alignment: Vertical::Center,
                line_height: Default::default(),
                shaping: iced_core::text::Shaping::Advanced,
            });

        });
        
        vec![circle_widget]
    }
}
fn draw_background(frame: &mut Frame) {

    let background_size = Path::rectangle(
        Point::ORIGIN, //START

        Size {
            width: frame.width(),
            height: frame.height(),
        },
    );

    //DRAW BACKGORUND
    frame.fill(&background_size, CONTAINER_COLOR);
}

fn draw_arc_completed_exercises(frame: &mut Frame, center_of_circle: Point, circle_widget: &CircleWidget ) {

    if circle_widget.completed_exercises > 0 {
        let arc_path = &Path::new(|builder| {
            builder.arc(Arc {
                center: center_of_circle,
                radius: CIRCLE_RADIUS,
                start_angle: Degrees(DEGREE_START_TRANSLATION + PADDING_BETWEEN_ARCS / 2.0 * circle_widget.circle_widget_state.animation_progress.value()).into(),
                end_angle: Degrees(DEGREE_START_TRANSLATION + (convert_done_exercises_in_degrees(circle_widget.completed_exercises, circle_widget.total_exercises) - PADDING_BETWEEN_ARCS / 2.0) * circle_widget.circle_widget_state.animation_progress.value()).into(),
            });
        });

        frame.stroke(arc_path, generate_stroke(20.0, circle_widget.active_mascot.get_primary_color()));
    }
}

fn draw_arc_not_completed_exercises(frame: &mut Frame, center_of_circle: Point, circle_widget: &CircleWidget ) {

    if circle_widget.total_exercises > circle_widget.completed_exercises {
        let arc_path = &Path::new(|builder| {
            builder.arc(Arc {
                center: center_of_circle,
                radius: CIRCLE_RADIUS,
                start_angle: Degrees(DEGREE_START_TRANSLATION + (convert_done_exercises_in_degrees(circle_widget.completed_exercises, circle_widget.total_exercises) + PADDING_BETWEEN_ARCS / 2.0) * circle_widget.circle_widget_state.animation_progress.value()
                ).into(),
                end_angle: Degrees(DEGREE_START_TRANSLATION + (360.0 - PADDING_BETWEEN_ARCS / 2.0) * circle_widget.circle_widget_state.animation_progress.value()).into(),
            });
        });

        frame.stroke(arc_path, generate_stroke(20.0, circle_widget.active_mascot.get_secondary_color()));
    }
}

//LOGIC

fn convert_done_exercises_in_degrees(completed_exercises: u32, total_exercises: u32 ) -> f32 {
    let ratio = completed_exercises as f32 /total_exercises as f32;

    match ratio {
        ..=1.0 => ratio * 360.0,
        _ =>  360.0
    }
}
