use crate::client::backend::widget_state::widget_state_manager::WidgetMessage;
use crate::client::gui::bb_theme::color::interpolate_colors;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_widget::animated_background_mod::line::{
    AnimatedLine, get_random_start_and_end_point_of_line,
};
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
use std::time::Duration;

const ANIMATION_VALUE_TO_SPAWN_NEW_LINE: f32 = 0.5;
pub const BASE_LINE_WIDTH: f32 = 200.0;

pub struct BackgroundAnimation<'a> {
    state: &'a BackgroundAnimationState,
}
impl<'a> BackgroundAnimation<'a> {
    pub fn new(background_animation_state: &'a BackgroundAnimationState) -> Self {
        Self {
            state: background_animation_state,
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
    pub current_mascot: Mascot,
    pub next_mascot: Mascot,
    pub color_animation: Animated<f32>,

    // Animation State
    pub cache: Cache,
    pub overall_animation: Animated<f32>,
}

impl Default for BackgroundAnimationState {
    fn default() -> Self {
        let color_animation_motion = Motion {
            response: Duration::from_secs(60),
            damping: Motion::SMOOTH.damping(),
        };

        let animation_motion = Motion {
            response: Duration::from_mins(10),
            damping: Motion::SMOOTH.damping(),
        };

        Self {
            lines: vec![],
            frame_size: None,
            current_mascot: Mascot::default(),
            next_mascot: Mascot::get_random_mascot(),
            color_animation: Animated::new(0.0, color_animation_motion),
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
                let path = line.bezier_curve();

                let line_stroke = Stroke {
                    style: Style::Gradient(Gradient::Linear(Linear {
                        start: Point::new(0.0, 0.0),
                        end: Point::new(bounds.size().width, bounds.size().height),
                        stops: [
                            Some(ColorStop {
                                offset: 0.0,
                                color: interpolate_colors(
                                    self.state.current_mascot.get_primary_color(),
                                    self.state.next_mascot.get_primary_color(),
                                    *self.state.color_animation.value(),
                                )
                                .scale_alpha(1.0 - line.animation_progress.value()),
                            }),
                            Some(ColorStop {
                                offset: 1.0,
                                color: interpolate_colors(
                                    self.state.current_mascot.get_secondary_color(),
                                    self.state.next_mascot.get_secondary_color(),
                                    *self.state.color_animation.value(),
                                )
                                .scale_alpha(1.0 - line.animation_progress.value()),
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
                if *state.overall_animation.value() > 0.99 {
                    *state = BackgroundAnimationState::default()
                }
                if *state.color_animation.value() > 0.99 {
                    state.current_mascot = state.next_mascot;
                    state.next_mascot = Mascot::get_random_mascot();
                    state.color_animation.settle_at(0.0);
                }
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
                state.color_animation.update(event);
                state.update_lines(event);
                state.overall_animation.update(event);
            }
        }
        Task::none()
    }
}

pub fn animated_line_background(
    background_animation_state: &BackgroundAnimationState,
) -> Element<'_, Message> {
    let animated_line: Element<Message> =
        BackgroundAnimation::new(background_animation_state).view();

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
