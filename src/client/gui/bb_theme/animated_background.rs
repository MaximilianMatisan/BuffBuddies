use crate::client::backend::widget_state::widget_state_manager::WidgetMessage;
use crate::client::backend::widget_state::widget_state_manager::WidgetMessage::Circle;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_widget::circle_widget::CircleMessage;
use crate::client::gui::user_interface::Message;
use crate::client::gui::user_interface::Message::Widget;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::widget::canvas::{Cache, Geometry, Path, Stroke, Style};
use iced::widget::{Action, canvas, container};
use iced::{Element, Renderer};
use iced_anim::{Animated, Animation, Event, Motion};
use iced_core::mouse::Cursor;
use iced_core::{Length, Rectangle, Theme};
use std::time::Duration;

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
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Option<Action<Message>> {
        self.state.cache.clear();

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
            let path = Path::new(|builder| {
                builder.move_to((0.0, 0.0).into());
                builder.line_to(
                    (
                        frame.size().width * self.state.animation_progress.value(),
                        frame.size().height * self.state.animation_progress.value(),
                    )
                        .into(),
                );
            });
            let line_stroke = Stroke {
                style: Style::Solid(self.mascot.get_secondary_color()),
                width: 10.0,
                line_cap: Default::default(),
                line_join: Default::default(),
                line_dash: Default::default(),
            };

            frame.stroke(&path, line_stroke)
        });

        vec![line]
    }
}
#[derive(Clone, Debug)]
pub enum BackgroundAnimationMessage {
    UpdateAnimation(Event<f32>),
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
