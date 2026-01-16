use iced::alignment;
use iced::mouse;
use iced::widget::canvas::{Cache, Geometry, LineCap, Path, Stroke, stroke, LineJoin, LineDash};
use iced::widget::{canvas, container, text};
use iced::{
    Degrees, Element, Fill, Font, Radians, Rectangle, Renderer, Size, Subscription, Theme,
    Vector,
};
use iced::advanced::graphics::text::cosmic_text::Shaping;
use iced::widget::canvas::path::Arc;
use iced_core::{color, renderer, Point};
use iced_core::alignment::{Horizontal, Vertical};
use crate::client::gui::bb_theme::color::CONTAINER_COLOR;
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::progress::ProgressWidget;
use crate::client::gui::user_interface::Message;

const CIRCLE_WIDGET_WIDTH: f32 = 300.0;
const CIRCLE_WIDGET_HEIGHT: f32 = 300.0;

pub struct CircleWidget {
    width: f32,
    height: f32,
    circle: Cache
}

impl CircleWidget {
    pub fn new() -> Self {
        Self {
            width: CIRCLE_WIDGET_WIDTH,
            height:CIRCLE_WIDGET_HEIGHT,
            circle: Cache::default()
        }
    }


    pub(crate) fn view(self) -> Element<'static, Message> {
        let canvas = canvas(self).width(CIRCLE_WIDGET_WIDTH).height(CIRCLE_WIDGET_HEIGHT);

        container(canvas).padding(20).into()
    }
}

impl<Message> canvas::Program<Message> for CircleWidget {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        use chrono::Timelike;

        let circle_widget = self.circle.draw(renderer, bounds.size(), |frame| {


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
                    end_angle: Degrees(80.0 - 90.0).into(),
                });
            }), Stroke {
                width: 20.0,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                style: stroke::Style::Solid(color!(255, 165, 0)),
                line_dash: Default::default(),
            });



            //REMAINING EXERCISES

            frame.stroke(&Path::new(|builder| {
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle: Degrees(110.0 - 90.0 ).into(),
                    end_angle: Degrees(330.0 - 90.0 ).into(),
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
