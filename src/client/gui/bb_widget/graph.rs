use crate::client::gui::bb_theme::color::{CONTAINER_COLOR, HIGHLIGHTED_CONTAINER_COLOR};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use iced::advanced::graphics::geometry::Frame;
use iced::advanced::graphics::gradient::Linear;
use iced::advanced::graphics::Gradient;
use iced::mouse;
use iced::widget::canvas::{event, stroke, Cache, Event, Geometry, LineCap, LineDash, LineJoin, Path, Stroke};
use iced::Renderer;
use iced_core::{color, Color};

use crate::client::backend::exercise::exercise_manager::ExerciseManager;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::mascot_trait::MascotTrait;
use crate::client::gui::app::App;
use crate::client::gui::user_interface::Message;
use iced::widget::canvas;
use iced::widget::container;
use iced::Element;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::gradient::ColorStop;
use iced_core::mouse::Cursor;
use iced_core::{
    Point, Rectangle, Size, Theme, Widget,
};

const GRAPH_WIDGET_WIDTH: f32 = 700.0;
const GRAPH_WIDGET_HEIGHT: f32 = 500.0;
const LINE_THICKNESS: f32 = 3.0;
const MOUSE_HIGHLIGHT_LINE_THICKNESS: f32 = 3.0;
const AXIS_FONT_SIZE: f32 = 12.0;
const PERCENTAGE_PLACEHOLDER: f32 = 0.05;
const PERCENTAGE_SPACING_WIDGET_AXIS: f32 = 0.1;
const BASE_SPACING_BETWEEN_COLUMNS: f32 = 150.0;
const FREQUENCY_OF_X_AXIS_LABELS: usize = 6;
const FREQUENCY_OF_Y_AXIS_LABELS: u32 = 10;
const AMOUNT_DASHED_LINES: u32 = 12;

pub struct GraphWidget<'a>
{
    width: f32,
    height: f32,
    active_mascot: Mascot,
    exercise_manager: &'a ExerciseManager,
    graph: Cache
}

impl<'a> GraphWidget<'a> {
    pub(crate) fn new(app:&'a App) -> Self {
        GraphWidget {
            width: GRAPH_WIDGET_WIDTH,
            height: GRAPH_WIDGET_HEIGHT,
            active_mascot: app.mascot_manager.selected_mascot,
            exercise_manager: &app.exercise_manager,
            graph: Cache::default()
        }
    }

    pub(crate) fn get_width(&self) -> f32 {
        self.width
    }

    pub(crate) fn view(self) -> Element<'a, Message> {
        let canvas = canvas(self).width(GRAPH_WIDGET_WIDTH).height(GRAPH_WIDGET_HEIGHT);

        container(canvas).padding(20).into()
    }
}

fn draw_dashed_lines (frame: &mut Frame<Renderer>) {

    let line_color = color!(120,120,122);

    let gradient = Gradient::Linear(
        Linear::new(
            Point {
                x: frame.width() / 2.0,
                y: 10.0,
            },
            Point {
                x: frame.width() / 2.0,
                y: frame.height() - 10.0,
            },
        )
            .add_stops([
                // fade in
                ColorStop {
                    offset: 0.0,
                    color: Color { a: 0.0, ..line_color},
                },
                ColorStop {
                    offset: 0.1,
                    color: line_color,
                },

                // solid middle
                ColorStop {
                    offset: 0.9,
                    color: line_color,
                },

                // fade out
                ColorStop {
                    offset: 1.0,
                    color: Color { a: 0.0, ..line_color },
                },
            ])
    );

    let dashed_stroke = Stroke {
        width: 2.0,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Gradient(gradient),
        line_dash: LineDash {
            segments: &[2.0, 6.0], // line, gap
            offset: 0,
        },
    };

    let block_width = GRAPH_WIDGET_WIDTH / 10.0;
    let block_height = GRAPH_WIDGET_WIDTH / 12.0;
    let mut current_x =  Point::ORIGIN.x;
    let mut current_y = Point::ORIGIN.y;

    //VERTICAL LINES
    for line in  1..AMOUNT_DASHED_LINES {
        frame.stroke(
            &Path::line(
                Point{
                    x:  Point::ORIGIN.x + current_x,
                    y:  Point::ORIGIN.y
                },
                Point{
                    x:  Point::ORIGIN.x + current_x,
                    y:  frame.height()
                }
            )
            ,dashed_stroke);
        current_x += block_width;
    }

    //HORIZONTAL LINES

    for line in  1..AMOUNT_DASHED_LINES {
        frame.stroke(
            &Path::line(
                Point{
                    x:  Point::ORIGIN.x ,
                    y:  Point::ORIGIN.y + current_y
                },
                Point{
                    x:  frame.width(),
                    y:  Point::ORIGIN.y + current_y
                }
            )
            ,dashed_stroke);
        current_y += block_height;
    }

}

fn draw_axis (active_mascot: &Mascot, frame: &mut Frame<Renderer>) { //LATER CREATE PARAMETER VIRTUAL GRAPH BOUNDS
    let axis_color = active_mascot.get_primary_color();

    let solid_mascot_colored_stroke = Stroke {
        width: 10.0,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Solid(axis_color),
        line_dash: Default::default()
    };

    //X-AXIS
    frame.stroke(
        &Path::line(
            //TOP
            Point {
                x: Point::ORIGIN.x,
                y: Point::ORIGIN.y
            },
            //BOTTOM
            Point {
                x: Point::ORIGIN.x,
                y: frame.height()
            }
        )
        , solid_mascot_colored_stroke);

    frame.stroke(
        &Path::line(
            //TOP
            Point {
                x: Point::ORIGIN.x,
                y: frame.height()
            },
            //BOTTOM
            Point {
                x: frame.width(),
                y: frame.height()
            }
        )
        , solid_mascot_colored_stroke);

    //ARROW - X

    let x_tip = Point::ORIGIN;
    let x_left = Point { x: Point::ORIGIN.x - 10.0, y: Point::ORIGIN.y };
    let x_right = Point { x: Point::ORIGIN.x + 10.0, y: Point::ORIGIN.y };

    frame.stroke(
        &Path::new(|builder| {
            builder.move_to(x_left);
            builder.line_to(x_tip);
            builder.line_to(x_right);
        }), solid_mascot_colored_stroke);


    //ARROW - Y

    let y_tip = Point{x: frame.width(), y: frame.height()};
    let y_up = Point { x: frame.width() - 10.0, y: frame.height() - 10.0 };
    let y_bottom = Point { x: frame.width() - 10.0, y: frame.height() + 10.0};

    frame.stroke(
        &Path::new(|builder| {
            builder.move_to(y_up);
            builder.line_to(y_tip);
            builder.line_to(y_bottom);
        }), solid_mascot_colored_stroke);
}

fn draw_points (frame: &mut Frame<Renderer>, points: Vec<Point>, mascot: &Mascot,) {

    let radius = 7.0;

    let block_width = GRAPH_WIDGET_WIDTH / 10.0;
    let mut y_values = vec![
        frame.height() - 20.0,   // STEIGT
        frame.height() - 80.0,
        frame.height() - 120.0,
        frame.height() - 180.0,
        frame.height() - 150.0,  // SENKT
        frame.height() - 220.0,  // STEIGT
        frame.height() - 250.0,
        frame.height() - 210.0,  // SENKT
        frame.height() - 270.0,  // STEIGT
        frame.height() - 330.0,  // STEIGT
    ];

    let mut current_x = block_width;
    y_values = y_values.into_iter().map(|y| y - 30.0).collect::<Vec<f32>>();;

    for y_value in y_values //CHOPPED THE START OF THE LIST (WILL CHANGE LOGIC LATER)
    {
        frame.fill(&Path::circle(Point{x: current_x , y: y_value}, radius),mascot.get_primary_color());
        current_x += block_width;
    }
}

fn draw_connections (frame: &mut Frame<Renderer>, points: Vec<Point>, mascot: &Mascot,) {

    let mut y_values = vec![
        frame.height() - 20.0,   // STEIGT
        frame.height() - 80.0,
        frame.height() - 120.0,
        frame.height() - 180.0,
        frame.height() - 150.0,  // SENKT
        frame.height() - 220.0,  // STEIGT
        frame.height() - 250.0,
        frame.height() - 210.0,  // SENKT
        frame.height() - 270.0,  // STEIGT
        frame.height() - 330.0,  // STEIGT
    ];

    let connection_stroke = Stroke {
        width: 1.5,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Solid(mascot.get_secondary_color()),
        line_dash: Default::default()
    };


    let block_width = GRAPH_WIDGET_WIDTH / 10.0;
    let mut current_x = block_width;

    let point_tuples = y_values.iter().enumerate().map(|(index,y)| {
        let start = Point{
            x: current_x,
            y: y - 30.0,
        };

        let mut end_y = y - 30.0;
        let mut end_x = current_x;

        if index < y_values.len() - 1 {
            end_y = y_values[index + 1];
            end_x = current_x + block_width;
        } else {                //letztes Element
            end_y = *y
        }

        let end =
            Point{
                x: end_x,
                y: end_y - 30.0,
            };

        current_x += block_width;

        (start,end)

    }).collect::<Vec<(Point,Point)>>();

    //draw start line from origin
    frame.stroke(
        &Path::line(Point{x:Point::ORIGIN.x , y:frame.height()},Point{x: block_width,y: y_values[0]- 30.0}),
        connection_stroke
    );

    for (start,end) in point_tuples //CHOPPED THE START OF THE LIST (WILL CHANGE LOGIC LATER)
    {
        frame.stroke(&Path::line(start,end),connection_stroke);
    }
}


//LOGIC

fn calculate_points() -> Vec<Point> {
    let mut vec = vec![
        Point { x: 40.0, y: 40.0 },
        Point { x: 100.0, y: 60.0 },
        Point { x: 160.0, y: 80.0 },
        Point { x: 220.0, y: 100.0 },
        Point { x: 280.0, y: 120.0 },
        Point { x: 340.0, y: 140.0 },
        Point { x: 400.0, y: 160.0 },
        Point { x: 460.0, y: 180.0 },
        Point { x: 520.0, y: 200.0 },
        Point { x: 580.0, y: 220.0 },
        Point { x: 640.0, y: 240.0 },
        Point { x: 700.0, y: 260.0 },
    ];

    vec
}

impl<'a> canvas::Program<Message> for GraphWidget<'a> {
    type State = ();
    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let graph_widget = self.graph.draw(renderer, bounds.size(), |frame| {

            //FRAME CENTER
            let mut center = frame.center();

            //DRAW BACKGROUND
            let background = Path::rectangle(Point::ORIGIN, Size {
                width: frame.width(),
                height: frame.height(),
            });

            frame.fill(&background, CONTAINER_COLOR);

            //DASHED LINES
            draw_dashed_lines(frame);

            //CONNECTIONS BETWEEN POINTS
            draw_connections(frame, calculate_points(), &self.active_mascot);

            //AXIS
            draw_axis(&self.active_mascot, frame);

            //POINTS
            draw_points(frame, calculate_points(), &self.active_mascot)

        });
        vec![graph_widget]
    }
}