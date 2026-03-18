use crate::client::backend::widget_state::widget_state_manager::WidgetMessage;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_widget::animated_background_mod::line::AnimatedLine;
use crate::client::gui::user_interface::Message;
use crate::client::gui::user_interface::Message::Widget;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::advanced::graphics::Gradient;
use iced::advanced::graphics::gradient::Linear;
use iced::widget::canvas::{Cache, Geometry, LineCap, LineJoin, Stroke, Style};
use iced::widget::{Action, canvas, container};
use iced::{Element, Renderer, Task};
use iced_anim::{Animated, Animation, Event, Motion};
use iced_core::gradient::ColorStop;
use iced_core::mouse::Cursor;
use iced_core::{Length, Point, Rectangle, Size, Theme};
use rand::RngExt;
use std::time::Duration;

const ANIMATION_VALUE_TO_SPAWN_NEW_LINE: f32 = 0.5;
const BASE_LINE_WIDTH: f32 = 200.0;
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
        let draw_percentage = &self.state.overall_animation;

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
    lines: Vec<AnimatedLine>,
    frame_size: Option<Size>,

    // Animation State
    pub cache: Cache,
    pub overall_animation: Animated<f32>,
}

impl Default for BackgroundAnimationState {
    fn default() -> Self {
        let animation_motion = Motion {
            response: Duration::from_millis(500000),
            damping: Motion::BOUNCY.damping(),
        };

        Self {
            lines: vec![],
            frame_size: None,
            cache: Cache::default(),
            overall_animation: Animated::new(0.0, animation_motion),
        }
    }
}
impl BackgroundAnimationState {
    /// Spawns a line at the end of lines
    fn spawn_line(&mut self) {
        if let Some(frame_size) = self.frame_size {
            let (start, end) = get_random_start_and_end_point_of_line(frame_size);
            let center = Point::new(frame_size.width / 2.0, frame_size.height / 2.0);
            self.lines.push(AnimatedLine::new(start, center, end));
        }
    }

    /// Updates all line animations with given Event
    fn update_lines(&mut self, event: Event<f32>) {
        for line in &mut self.lines {
            line.animation_progress.update(event);
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

        if self.state.frame_size.is_none() {
            return Some(Action::publish(Widget(WidgetMessage::BackgroundAnimation(
                BackgroundAnimationMessage::GetFrameSize(bounds.size()),
            ))));
        }
        if let Some(size) = self.state.frame_size
            && size != bounds.size()
        {
            return Some(Action::publish(Widget(WidgetMessage::BackgroundAnimation(
                BackgroundAnimationMessage::Init(bounds.size()),
            ))));
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
        let drawing = self.state.cache.draw(renderer, bounds.size(), |frame| {
            for line in &self.state.lines {
                let progress = line.animation_progress.value();
                let path = line.bezier_curve();

                let line_stroke = Stroke {
                    style: Style::Gradient(Gradient::Linear(Linear {
                        start: Point::new(0.0, 0.0),
                        end: Point::new(bounds.size().width, bounds.size().height),
                        stops: [
                            Some(ColorStop {
                                offset: 0.0,
                                color: self.mascot.get_primary_color().scale_alpha(1.0 - progress),
                            }),
                            Some(ColorStop {
                                offset: 1.0,
                                color: self
                                    .mascot
                                    .get_secondary_color()
                                    .scale_alpha(1.0 - progress),
                            }),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                        ],
                    })),
                    width: BASE_LINE_WIDTH,
                    line_cap: LineCap::Round,
                    line_join: LineJoin::Round,
                    line_dash: Default::default(),
                };

                frame.stroke(&path, line_stroke)
            }
        });

        vec![drawing]
    }
}
#[derive(Clone, Debug)]
pub enum BackgroundAnimationMessage {
    Init(Size),
    GetFrameSize(Size),
    UpdateAnimation(Event<f32>),
}
impl BackgroundAnimationMessage {
    pub fn update(self, state: &mut BackgroundAnimationState) -> Task<Message> {
        match self {
            BackgroundAnimationMessage::Init(size) => {
                *state = BackgroundAnimationState::default();
                state.frame_size = Some(size)
            }
            BackgroundAnimationMessage::GetFrameSize(size) => {
                state.frame_size = Some(size);
            }
            BackgroundAnimationMessage::UpdateAnimation(event) => {
                // Spawn lines if necessary
                let mut should_spawn_line = false;
                for line in &mut state.lines {
                    if *line.animation_progress.value() > ANIMATION_VALUE_TO_SPAWN_NEW_LINE
                        && !line.has_spawned_line
                    {
                        line.has_spawned_line = true;
                        should_spawn_line = true;
                    }
                }
                if should_spawn_line || state.lines.is_empty() {
                    state.spawn_line();
                }

                // Delete lines which finished their animation
                state
                    .lines
                    .retain(|line| *line.animation_progress.value() < 0.99);

                // Update animations
                state.update_lines(event);
                state.overall_animation.update(event);
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

/// Returns a random start and end points on an edge of the frame
fn get_random_start_and_end_point_of_line(size: Size) -> (Point, Point) {
    let mut rng = rand::rng();
    let start_point_horizontal_start = rng.random_bool(0.5);
    let start_point_first_edge = rng.random_bool(0.5);
    let start_point =
        random_point_on_edge(size, start_point_horizontal_start, start_point_first_edge);

    let end_point_horizontal_start = rng.random_bool(0.5);
    let end_point_first_edge = if start_point_horizontal_start == end_point_horizontal_start {
        // End Point shouldn't be on the same edge as start!
        !start_point_first_edge
    } else {
        rng.random_bool(0.5)
    };
    let end_point = random_point_on_edge(size, end_point_horizontal_start, end_point_first_edge);

    (start_point, end_point)
}

/// Returns a random point on a specified edge of the given Size
/// ## Arguments
///
/// ### horizontal_start
/// Whether the Point should start on a horizontal or vertical line
///
/// ### first_edge
/// Whether the Point should start on the
///
///  * top(first) or bottom(second)
///  * left(first) or right(second)
///
/// edge, interpretation depends on `horizontal_start`
fn random_point_on_edge(size: Size, horizontal_start: bool, first_edge: bool) -> Point {
    let mut rng = rand::rng();
    let offset_on_edge: f32 = rng.random_range(0.0..=1.0);
    match (horizontal_start, first_edge) {
        (true, true) => Point::new(size.width * offset_on_edge, -BASE_LINE_WIDTH), // Top
        (true, false) => Point::new(size.width * offset_on_edge, size.height + BASE_LINE_WIDTH), // Bottom
        (false, true) => Point::new(-BASE_LINE_WIDTH, size.height * offset_on_edge), // Left
        (false, false) => Point::new(size.width + BASE_LINE_WIDTH, size.height * offset_on_edge), // Right
    }
}
