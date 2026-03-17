use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::widget::canvas::{Cache, Geometry, Path, Stroke, Style};
use iced::widget::{canvas, container};
use iced::{Element, Renderer};
use iced_anim::Animated;
use iced_core::mouse::Cursor;
use iced_core::{Length, Rectangle, Theme};

#[allow(dead_code)]
#[derive(Default)]
pub struct BackgroundLine {
    cache: Cache,
    progress: Animated<f32>,
    mascot: Mascot,
}
impl BackgroundLine {
    pub fn new(mascot: Mascot) -> Self {
        Self {
            mascot,
            ..Default::default()
        }
    }
}

impl canvas::Program<Message> for BackgroundLine {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let line = self.cache.draw(renderer, bounds.size(), |frame| {
            let path = Path::new(|builder| {
                builder.move_to((0.0, 0.0).into());
                builder.line_to((frame.size().width, frame.size().height).into());
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

pub fn animated_line_background(mascot: &Mascot) -> Element<'_, Message> {
    let line: Element<'_, Message> = canvas(BackgroundLine::new(*mascot))
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

    container(line)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(create_container_style(
            ContainerStyle::Background,
            Some(0.0.into()),
            None,
        ))
        .into()
}
