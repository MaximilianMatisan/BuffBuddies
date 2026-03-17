use crate::client::backend::widget_state::widget_state_manager::WidgetMessage;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::user_interface::Message;
use crate::client::gui::user_interface::Message::Widget;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::widget::canvas::{Cache, Frame, Geometry, LineCap, LineJoin, Path, Stroke, Style};
use iced::widget::{Action, canvas, container};
use iced::{Element, Renderer, Task};
use iced_anim::{Animated, Animation, Event, Motion};
use iced_core::mouse::Cursor;
use iced_core::{Length, Point, Rectangle, Size, Theme};
use std::time::Duration;
use iced::advanced::graphics::Gradient;
use iced::advanced::graphics::gradient::Linear;
use iced_core::gradient::ColorStop;
use rand::RngExt;
use crate::client::gui::bb_theme::color::create_color_stops;

pub struct BackgroundAnimation<'a> {
    state: &'a BackgroundAnimationState,
    mascot: Mascot,
}
impl<'a> BackgroundAnimation<'a> {
    pub fn new(app: &'a App) -> Self {
        Self {
            mascot: app.mascot_manager.selected_mascot,
            state: &app.widget_manager.background_animation_state,
        }
    }

    pub fn view(self) -> Element<'a, Message> {
        let draw_percentage = &self.state.animation_progress;

        let canvas = canvas(self).width(Length::Fill).height(Length::Fill);

        Animation::new(draw_percentage, canvas)
            .on_update(|event| {
                Widget(WidgetMessage::BackgroundAnimation(
                    BackgroundAnimationMessage::UpdateAnimation(event),
                ))
            })
            .into()
    }
}
pub struct BackgroundAnimationState {
    start_point: Option<Point>,

    // Animation State
    pub cache: Cache,
    pub animation_progress: Animated<f32>,
}

impl Default for BackgroundAnimationState {
    fn default() -> Self {
        let animation_motion = Motion {
            response: Duration::from_millis(5000),
            damping: Motion::BOUNCY.damping(),
        };

        Self {
            start_point: None,
            cache: Cache::default(),
            animation_progress: Animated::new(0.0, animation_motion),
        }
    }
}

impl<'a> canvas::Program<Message> for BackgroundAnimation<'a> {
    type State = BackgroundAnimationState;

    fn update(
        &self,
        _state: &mut Self::State,
        _event: &canvas::Event,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Option<Action<Message>> {
        self.state.cache.clear();

        if self.state.start_point.is_none() {
            return Some(Action::publish(
                Widget(WidgetMessage::BackgroundAnimation(
                    BackgroundAnimationMessage::Init(bounds.size()),
                ))
            ));
        }

        Some(Action::publish(Widget(WidgetMessage::BackgroundAnimation(
            BackgroundAnimationMessage::UpdateAnimation(Event::Target(1.0)),
        ))))
    }
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let line = self.state.cache.draw(renderer, bounds.size(), |frame| {
            let start = self.state.start_point.unwrap_or(Point::ORIGIN);
            let end = frame.center();
            let progress = self.state.animation_progress.value();

            let current = Point::new(
                start.x + (end.x - start.x) * progress,
                start.y + (end.y - start.y) * progress,
            );
            let path = Path::new(|builder| {
                builder.move_to(self.state.start_point.unwrap_or(Point::ORIGIN));
                builder.line_to(
                    current
                );
            });
            let line_stroke = Stroke {
                style: Style::Gradient(Gradient::Linear(Linear {
                    start: Point::new(0.0, 0.0),
                    end: Point::new(bounds.size().width, bounds.size().height),
                    stops: [
                        Some(ColorStop { offset: 0.0, color: self.mascot.get_primary_color()}),
                        Some(ColorStop { offset: 1.0, color: self.mascot.get_secondary_color()}),
                        None, None, None, None, None, None
                    ],
                })),
                width: 65.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                line_dash: Default::default(),
            };

            frame.stroke(&path, line_stroke)
        });

        vec![line]
    }
}
#[derive(Clone, Debug)]
pub enum BackgroundAnimationMessage {
    Init(Size),
    UpdateAnimation(Event<f32>),
}
impl BackgroundAnimationMessage {
    pub fn update(self, state: &mut BackgroundAnimationState) -> Task<Message> {
        match self {
            BackgroundAnimationMessage::Init(size) => {
                state.start_point = Some(get_random_start_point_of_line(size));
            }
            BackgroundAnimationMessage::UpdateAnimation(event) => {
                state
                    .animation_progress
                    .update(event);
                state.cache.clear();
            }
        }
        Task::none()
    }
}

pub fn animated_line_background(app: &App) -> Element<'_, Message> {
    let animated_line: Element<Message> = BackgroundAnimation::new(app).view();

    container(animated_line)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(create_container_style(
            ContainerStyle::Background,
            Some(0.0.into()),
            None,
        ))
        .into()
}

/// Returns a random point on an edge of the frame
fn get_random_start_point_of_line(size: Size) -> Point {
    let mut rng = rand::rng();

    // Whether the Point should start on a horizontal or vertical line
    let horizontal_start = rng.random_bool(0.5);

    // Whether the Point should start on the
    //  * top(first) or bottom(second)
    //  * left(first) or right(second)
    // edge, interpretation depends on `horizontal_start`
    let first_edge = rng.random_bool(0.5);

    let offset_on_edge: f32 = rng.random_range(0.0..=1.0);

    match (horizontal_start, first_edge) {
        (true, true) => Point::new(size.width * offset_on_edge, 0.0), // Top
        (true, false) => Point::new(size.width * offset_on_edge, size.height), // Bottom
        (false, true) => Point::new(0.0, size.height * offset_on_edge), // Left
        (false, false) => Point::new(size.width, size.height * offset_on_edge), // Right
    }
}