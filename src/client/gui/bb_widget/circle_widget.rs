use std::time::Duration;
use iced::alignment;
use iced::mouse;
use iced::widget::canvas::{Cache, Geometry, LineCap, Path, Stroke, stroke, LineJoin, LineDash, event};
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
use crate::client::gui::bb_theme::color::CONTAINER_COLOR;
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;

const CIRCLE_WIDGET_WIDTH: f32 = 300.0;
const CIRCLE_WIDGET_HEIGHT: f32 = 300.0;


pub struct CircleWidget<'a> {
    active_mascot: Mascot,
    exercise_manager: &'a ExerciseManager,
    circle_widget_state: &'a CircleWidgetState,
}

#[derive(Clone, Debug)]
pub enum  CircleMessage{
    UpdateAnimation(Event<f32>)
}

impl<'a> CircleWidget<'a> {
    pub(crate) fn new(app: &'a App) -> Self {
        CircleWidget {
            active_mascot: app.mascot_manager.selected_mascot,
            exercise_manager: &app.exercise_manager,
            circle_widget_state: &app.circle_widget_state,
        }
    }
    pub(crate) fn view (self) -> Element<'a, Message> {

        let draw_percentage = &self.circle_widget_state.animation_progress;

        let canvas = canvas(self).width(CIRCLE_WIDGET_WIDTH).height(CIRCLE_WIDGET_HEIGHT);

        Animation::new(draw_percentage, canvas)
            .on_update(|event| Message::Circle(CircleMessage::UpdateAnimation(event)))
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
                Some(crate::client::gui::user_interface::Message::Circle(CircleMessage::UpdateAnimation(
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


            let mut center = frame.center();
            center.y += 10.0;

            let radius = 80.0;

            let background = Path::rectangle(Point::ORIGIN,
            Size {
                width: frame.width(),
                height: frame.height(),
            }
            );

            frame.fill(&background,CONTAINER_COLOR);


            //DONE EXERCISES
            frame.stroke(&Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle: Degrees(0.0 - 90.0).into(),
                    end_angle: Degrees(80.0 * self.circle_widget_state.animation_progress.value() - 90.0).into(),
                });
            }), Stroke {
                width: 20.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                style: stroke::Style::Solid(self.active_mascot.get_primary_color()),
                line_dash: Default::default(),
            });



            //REMAINING EXERCISES

            frame.stroke(&Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle: Degrees(110.0 * self.circle_widget_state.animation_progress.value() - 90.0 ).into(),
                    end_angle: Degrees(330.0  - 90.0).into(),
                });
            }), Stroke {
                width: 20.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                style: stroke::Style::Solid(color!(128, 128, 128)),
                line_dash: Default::default()
            });

            frame.fill_text(canvas::Text {
                content: String::from("1/4"),
                size: 30.0.into(),
                position: center,
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
                    x: center.x,
                    y: center.y - radius - 32.0,
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
