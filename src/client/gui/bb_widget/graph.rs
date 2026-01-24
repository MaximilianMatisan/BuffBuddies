use chrono::NaiveDate;
use crate::client::gui::bb_theme::color::{CONTAINER_COLOR, DASHED_LINES_COLOR, HIGHLIGHTED_CONTAINER_COLOR};
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
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::container::{create_container_style, ContainerStyle};
use crate::client::gui::bb_theme::custom_button::{create_text_button, ButtonStyle};
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::bb_theme::text_format;
use crate::client::gui::bb_widget::stats::exercise_stat_column;
use crate::client::gui::user_interface::Message;
use iced::widget::{canvas, combo_box, row, text};
use iced::widget::{container, Column, Row, Space};
use iced::Element;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::gradient::ColorStop;
use iced_core::keyboard::Key;
use iced_core::mouse::Cursor;
use iced_core::{
    Length, Padding, Point, Rectangle, Size, Theme, Widget,
};
use crate::client::backend::exercise::weight::Kg;

const GRAPH_WIDGET_WIDTH: f32 = 700.0;
const GRAPH_WIDGET_HEIGHT: f32 = 500.0;
const GRAPH_PADDING: f32 = 50.0;
const LINE_THICKNESS: f32 = 3.0;
const MOUSE_INFORMATION_SIZE: f32 = 3.0;
const AXIS_FONT_SIZE: f32 = 12.0;
const FREQUENCY_OF_Y_AXIS_LABELS: u32 = 9;
pub const MAX_AMOUNT_POINTS: u8 = 16;

#[derive(Clone, Debug)]
pub enum GraphMessage{
    GraphCursorMoved(Point),
    GraphKeyPressed(Key),
    IncrementCounter,
    DecrementCounter
}

#[derive(Default,Clone)]
pub struct GraphWidgetState {
    visible_points: bool,
    visible_cursor_information: bool,
    points_to_draw: u8,
}


impl GraphWidgetState {
    pub fn new() -> Self {

        GraphWidgetState {
            visible_points: true,
            visible_cursor_information: true,
            points_to_draw: 9,
        }

    }
    pub fn invert_visible_points(&mut self) {
        self.visible_points = !self.visible_points
    }
    pub fn invert_visible_cursor_information(&mut self) {
        self.visible_cursor_information = !self.visible_cursor_information
    }

    pub(crate) fn increment_counter(&mut self)  {
        self.points_to_draw += 1 ;
    }

    pub(crate) fn decrement_counter(&mut self)  {
        self.points_to_draw -= 1 ;
    }
    pub (crate) fn get_counter(&self) -> u8 {self.points_to_draw}
}
pub struct GraphWidget<'a>
{
    width: f32,
    height: f32,
    active_mascot: Mascot,
    exercise_manager: &'a ExerciseManager,
    graph_state: GraphWidgetState,
    graph: Cache
}

impl<'a> GraphWidget<'a> {
    pub(crate) fn new(app:&'a App) -> Self {
        GraphWidget {
            width: GRAPH_WIDGET_WIDTH,
            height: GRAPH_WIDGET_HEIGHT,
            active_mascot: app.mascot_manager.selected_mascot,
            exercise_manager: &app.exercise_manager,
            graph_state: app.graph_widget_state.clone(),
            graph: Cache::default()
        }
    }

    pub(crate) fn get_width(&self) -> f32 {
        self.width
    }

    pub(crate) fn view(self) -> Element<'a, Message> {
        let canvas = canvas(self).width(GRAPH_WIDGET_WIDTH).height(GRAPH_WIDGET_HEIGHT);

        container(canvas).into()
    }
}

fn draw_dashed_lines (graph_widget_state: &GraphWidgetState ,frame: &mut Frame<Renderer>) {

    let dashed_lines_gradient_padding = GRAPH_PADDING + 10.0;

    let gradient = Gradient::Linear(
        Linear::new(
            Point {
                x: frame.width() / 2.0,
                y: dashed_lines_gradient_padding
            },
            Point {
                x: frame.width() / 2.0 ,
                y: frame.height() - dashed_lines_gradient_padding
            },
        )
            .add_stops([
                // FADE IN
                ColorStop {
                    offset: 0.0,
                    color: Color { a: 0.0, ..DASHED_LINES_COLOR},
                },

                //ACTUAL COLOR
                ColorStop {
                    offset: 0.1,
                    color: DASHED_LINES_COLOR,
                },


                ColorStop {
                    offset: 0.9,
                    color: DASHED_LINES_COLOR,
                },

                // FADE OUT
                ColorStop {
                    offset: 1.0,
                    color: Color { a: 0.0, ..DASHED_LINES_COLOR },
                },
            ])
    );

    let dashed_stroke = Stroke {
        width: 2.0,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Gradient(gradient),
        line_dash: LineDash {
            segments: &[2.0, 6.0], // LINE LENGTH , GAP LENGTH
            offset: 0,
        },
    };
    //PADDING LEFT AND RIGHT: LEFT FOR Y_LABELS, RIGHT FOR FREE SPACE
    let block_width = (GRAPH_WIDGET_WIDTH - GRAPH_PADDING * 2.0) / graph_widget_state.points_to_draw as f32 ;//division with 0 is not possible since limit is 1
    //PADDING UP AND DOWN:  UP FOR Y-AXIS-ARROW SPACE,DOWN FOR X-LABELS
    let block_height = (GRAPH_WIDGET_HEIGHT - GRAPH_PADDING * 2.0)  / FREQUENCY_OF_Y_AXIS_LABELS as f32;

    let mut current_x =  GRAPH_WIDGET_WIDTH - GRAPH_PADDING; //START OF GRAPH
    let mut current_y = Point::ORIGIN.y + GRAPH_PADDING;

    let mut draw_stroke = |start: Point, end: Point, stroke: Stroke| {
        frame.stroke(
                &Path::line(start,end)
                ,stroke
            );
        };

    //VERTICAL LINES
    for line in  1..= graph_widget_state.points_to_draw {

        let point_up = Point{
            x:  Point::ORIGIN.x + current_x,
            y:  Point::ORIGIN.y + GRAPH_PADDING
        };

        let point_bottom = Point{
            x:  Point::ORIGIN.x + current_x,
            y:  GRAPH_WIDGET_HEIGHT - GRAPH_PADDING
        };

        draw_stroke(point_up,point_bottom,dashed_stroke);
        current_x -= block_width;
    }

    //HORIZONTAL LINES

    for line in  1..=FREQUENCY_OF_Y_AXIS_LABELS {

        let point_left = Point{
            x:  Point::ORIGIN.x + GRAPH_PADDING,
            y:  Point::ORIGIN.y + current_y
        };

        let point_right = Point{
            x:  GRAPH_WIDGET_WIDTH - GRAPH_PADDING,
            y:  Point::ORIGIN.y + current_y
        };

        draw_stroke(point_left,point_right,dashed_stroke);
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

    //Y-AXIS
    frame.stroke(
        &Path::line(
            //TOP
            Point {
                x: Point::ORIGIN.x + GRAPH_PADDING,
                y: Point::ORIGIN.y + GRAPH_PADDING
            },
            //BOTTOM
            Point {
                x: Point::ORIGIN.x + GRAPH_PADDING,
                y: frame.height()  - GRAPH_PADDING,
            }
        )
        , solid_mascot_colored_stroke);

    //X-AXIS
    frame.stroke(
        &Path::line(
            //LEFT
            Point {
                x: Point::ORIGIN.x + GRAPH_PADDING,
                y: frame.height() - GRAPH_PADDING
            },
            //RIGHT
            Point {
                x: frame.width() - GRAPH_PADDING,
                y: frame.height() - GRAPH_PADDING
            }
        )
        , solid_mascot_colored_stroke);

    //ARROW - Y
    let distance_from_tip = 10.0;

    let y_tip = Point {x: Point::ORIGIN.x + GRAPH_PADDING ,y: Point::ORIGIN.y + GRAPH_PADDING };
    let y_left = Point { x: y_tip.x - distance_from_tip, y: y_tip.y + distance_from_tip };
    let y_right = Point { x: y_tip.x + distance_from_tip, y: y_tip.y + distance_from_tip };

    //ARROW - X

    let x_tip = Point{x: frame.width() - GRAPH_PADDING, y: frame.height() - GRAPH_PADDING};
    let x_up = Point { x:x_tip.x - distance_from_tip, y: x_tip.y - distance_from_tip };
    let x_bottom = Point { x: x_tip.x - distance_from_tip, y:  x_tip.y + distance_from_tip};

    let mut draw_arrow = |point_1: Point, tip: Point, point_2: Point| {
        frame.stroke(
            &Path::new(|builder| {
                builder.move_to(point_1);
                builder.line_to(tip);
                builder.line_to(point_2);
            }), solid_mascot_colored_stroke);
    };

    draw_arrow(y_left,y_tip,y_right);
    draw_arrow(x_up, x_tip, x_bottom);

}

fn draw_points (graph_widget_state: &GraphWidgetState, frame: &mut Frame<Renderer>, y_values: Vec<Kg>, mascot: &Mascot,) {

    //!You should pass a list with the y-values,which are things such as weight lifted,kms run,etc.
    //! The list has to be sorted!

    let points = calculate_points(graph_widget_state,y_values);
    let base_size_point = 4.0;
    let max_size_point = 8.7;
    let range_size_point = max_size_point -  base_size_point;
    let percentage = points.len() as f32 / MAX_AMOUNT_POINTS as f32;

    //CALCULATE POINTS RADIUS
    let point_size = max_size_point - percentage * range_size_point;

    //DRAW POINTS
    for point in points.iter() {
        frame.fill(&Path::circle(*point, point_size), mascot.get_primary_color());
    }
}

fn draw_connections (graph_widget_state: &GraphWidgetState, frame: &mut Frame<Renderer>, y_values: Vec<f32>, mascot: &Mascot,) {

    let points = calculate_points(graph_widget_state,y_values);

    let connection_stroke = Stroke {
        width: 1.5,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Solid(mascot.get_secondary_color()),
        line_dash: Default::default()
    };


    let point_tuples =
        points.iter().enumerate().map(|(index,y)| {

            let last_element = points.len() - 1;
            let path_start = y;
            let mut path_end = path_start ;


            if index != last_element {
                path_end = &points[index + 1];
            }

            (*path_start,*path_end)

    }).collect::<Vec<(Point,Point)>>();

    let graph_origin = Point {
        x: Point::ORIGIN.x + GRAPH_PADDING,
        y: frame.height()  - GRAPH_PADDING,
    };

    //draw start line from origin
    frame.stroke(
        &Path::line(graph_origin,points[0]), //it is sure that there is at least one point since the function doesn't get called if length() < 1
        connection_stroke
    );

    for (start,end) in point_tuples
    {
        frame.stroke(&Path::line(start,end),connection_stroke);
    }
}

fn draw_cursor_information(bounds: Rectangle,cursor: Cursor, frame: &mut Frame<Renderer>) {
    if bounds.contains(cursor.position().unwrap_or_default()) {

        let cursor_information_position =
            if let Some(mut position) = cursor.position_from(Point::ORIGIN) {
                position.x -= 361.0 - 70.0;
                position.y -= 1114.0 + 30.0;
                position
            } else {
                Point{ x: 0.0, y: 0.0 }
            };


        frame.fill(
            &Path::rounded_rectangle(
                Point{
                    x: cursor_information_position.x - 62.0,
                    y: cursor_information_position.y - 30.0
                }, Size { width: 120.0, height: 60.0 },10.0.into()),HIGHLIGHTED_CONTAINER_COLOR);
        frame.fill_text(canvas::Text {
            content: format!("Date: {}\nKg: {}", cursor_information_position.x.round(), cursor_information_position.y.round()), //TODO:HANDLE UNWRAP
            size: 15.0.into(),
            position: cursor_information_position,
            color: color!(255,255,255),
            font: FIRA_SANS_EXTRABOLD,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: Default::default(),
            shaping: iced_core::text::Shaping::Advanced,
        });
    }
}

//LOGIC

fn calculate_points(graph_widget_state: &GraphWidgetState, y_values: Vec<Kg>) -> Vec<Point> {
    let chopped_y_values = &y_values[y_values.len() - graph_widget_state.points_to_draw as usize..];

    //PADDING LEFT AND RIGHT: LEFT FOR Y_LABELS, RIGHT FOR FREE SPACE
    let block_width = (GRAPH_WIDGET_WIDTH - GRAPH_PADDING * 2.0) / graph_widget_state.points_to_draw as f32 ; //division with 0 is not possible since limit is 1
    //PADDING UP AND DOWN:  UP FOR Y-AXIS-ARROW SPACE,DOWN FOR X-LABELS
    let block_height = (GRAPH_WIDGET_HEIGHT - GRAPH_PADDING * 2.0)  / FREQUENCY_OF_Y_AXIS_LABELS as f32;
    let mut current_x = GRAPH_WIDGET_WIDTH - GRAPH_PADDING;

    //I can unwrap() since the function is not going to get called if exercises.len() = 0
    let min_y: Kg = (chopped_y_values.iter().map(|value| (*value * 10.0) as usize).min().unwrap_or(0)) as f32 / 10.0; //TODO: USE FUNCTION IN weight.rs when connecting to ExerciseManager
    let max_y: Kg = (chopped_y_values.iter().map(|value| (*value * 10.0) as usize).max().unwrap_or(0)) as f32 / 10.0; //TODO: USE FUNCTION IN weight.rs when connecting to ExerciseManager
    let delta = max_y - min_y;
    let percentage = |current_y: Kg| {
        if delta == 0.0 {
            1.0
        } else {
            (current_y - min_y) / delta
        }
    };

    let height_padding_for_arrow = block_height;
    let x_axis_padding = block_height;
    let height_graph: f32 = GRAPH_WIDGET_HEIGHT - GRAPH_PADDING * 2.0 - height_padding_for_arrow - x_axis_padding;
    let lowest_point_graph: f32 = GRAPH_WIDGET_HEIGHT - GRAPH_PADDING - x_axis_padding;

    let calculate_graph_value=  |y: &Kg| -> f32 {
        (lowest_point_graph - (percentage(*y) * height_graph))
    };

    let mut new_y_values = vec![];

    let mut x_values = vec![];

    let mut points = vec![];

    //CALCULATE Y-VALUES
    for y_value in chopped_y_values
    {
        new_y_values.push(calculate_graph_value(y_value));
    }

    //CALCULATE X-VALUES
    for x_value in (1..=graph_widget_state.points_to_draw) {
        x_values.push(current_x);
        current_x -= block_width
    }
    x_values.reverse();

    //ZIP X- AND Y-VALUES
    for i  in 0.. graph_widget_state.points_to_draw {
        points.push(Point{x: x_values[i as usize],y: new_y_values[i as usize]})
    }

    points

}



impl<'a> canvas::Program<Message> for GraphWidget<'a> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        _bounds: Rectangle,
        cursor: iced_core::mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                (iced::widget::canvas::event::Status::Captured, Some(Message::Graph(GraphMessage::GraphCursorMoved(position))))
            },
            canvas::Event::Keyboard(iced::keyboard::Event::KeyPressed {key, .. }) => {
                (iced::widget::canvas::event::Status::Captured, Some(Message::Graph(GraphMessage::GraphKeyPressed(key))))},

            _ => (iced::widget::canvas::event::Status::Ignored, None)
        }
    }


    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let graph_widget = self.graph.draw(renderer, bounds.size(), |frame| {

            println!("ORIGIN: {}", Point::ORIGIN);

            //FRAME CENTER
            let mut center = frame.center();

            //DRAW BACKGROUND
            let background = Path::rectangle(Point::ORIGIN, Size {
                width: frame.width(),
                height: frame.height(),
            });

            println!("Cursor Position from {}: {:?}",Point::ORIGIN,cursor.position_from(Point::ORIGIN));

            match self.exercise_manager.data_points.len() {
                0 => {
                    frame.fill_text(canvas::Text{
                        content: "NO DATA".to_string(),
                        position: center,
                        color: self.active_mascot.get_primary_color(),
                        size: 40.into(),
                        line_height: Default::default(),
                        font: FIRA_SANS_EXTRABOLD,
                        horizontal_alignment: Horizontal::Left,
                        vertical_alignment: Vertical::Top,
                        shaping: Default::default(),
                    }
                    )

                }
                data_points_amount => {

                    let mut y_values = vec![
                        30.0,
                        35.0,
                        40.0,
                        45.0,
                        50.0,
                        55.0,
                        60.0,
                        65.0,
                        70.0,
                        70.0,
                        75.0,
                        80.0,
                        80.0,
                        85.0,
                        90.0,
                        100.0,
                    ];

                    frame.fill(&background, CONTAINER_COLOR);

                    //DASHED LINES
                    draw_dashed_lines(&self.graph_state , frame);

                    //CONNECTIONS BETWEEN POINTS
                    draw_connections(&self.graph_state, frame, y_values.clone(), &self.active_mascot);

                    //AXIS
                    draw_axis(&self.active_mascot, frame);

                    //POINTS
                    if self.graph_state.visible_points {
                        draw_points(&self.graph_state,frame, y_values, &self.active_mascot);
                    }

                    //CURSOR
                    if self.graph_state.visible_cursor_information {
                        draw_cursor_information(bounds, cursor, frame);
                    }
                }
            }

        });

        vec![graph_widget]
    }
}

pub fn graph_environment_widget<'a>(app: &'a App) -> Element<'a, Message> {
    let default_padding = 30.0;
    let title: Element<'a, Message> =
        format_button_text(iced::widget::text("PRs").size(40)).into();
    let search_bar: Element<Message> = combo_box(
        &app.exercise_manager.owned_exercise_state,
        "Search Exercise...",
        Some(&app.exercise_manager.selected_exercise_name),
        Message::SelectExercise,
    )
        .menu_style(bb_theme::combo_box::create_menu_style(
            &app.mascot_manager.selected_mascot,
        ))
        .input_style(bb_theme::combo_box::create_text_input_style(
            &app.mascot_manager.selected_mascot,
        ))
        .font(text_format::FIRA_SANS_EXTRABOLD)
        .width(Length::Fixed(250.0))
        .padding([8, 16])
        .into();

    let mut counter = format_button_text(text!("{}", app.graph_widget_state.points_to_draw));
        counter = counter.size(19);
    let increment_button =
        create_text_button(&app.mascot_manager.selected_mascot,
                           "+".to_string(),
                           ButtonStyle::Active,
                           Some(100.0.into())
        ).on_press(Message::Graph(GraphMessage::IncrementCounter));
    let decrement_button =
        create_text_button(&app.mascot_manager.selected_mascot,
                           "-".to_string(),
                           ButtonStyle::Active,
                           Some(100.0.into())
        ).on_press(Message::Graph(GraphMessage::DecrementCounter)) ;

    let row_counter_with_buttons = row![
        decrement_button,
        Space::with_width(Length::FillPortion(1)),
        counter,
        Space::with_width(Length::FillPortion(1)),
        increment_button,
    ].align_y(Vertical::Center);

    let counter_with_buttons =
        container(row_counter_with_buttons)
            .style(create_container_style(ContainerStyle::Highlighted,Some(10.into()),None))
            .width(Length::Fixed(100.0));

    let graph_widget =
        GraphWidget::new(app);

    let exercise_stats = exercise_stat_column(app)
        .width(Length::Fixed(graph_widget.get_width()))
        .padding(Padding {
            top: 0.0,
            right: default_padding,
            bottom: default_padding,
            left: default_padding,
        });


    let header_row = Row::new()
        .width(Length::Fixed(graph_widget.get_width()))
        .push(Space::with_width(Length::FillPortion(1)))
        .push(title)
        .push(Space::with_width(Length::FillPortion(1)))
        .push(counter_with_buttons)
        .push(Space::with_width(Length::FillPortion(3)))
        .push(search_bar)
        .push(Space::with_width(Length::FillPortion(1)))
        .align_y(Vertical::Center);

    let contents = Column::new()
        .width(Length::Shrink)
        .push(header_row)
        .push(graph_widget.view())
        .push(exercise_stats)
        .padding(Padding {
            top: default_padding / 2.0,
            ..Default::default()
        })
        .align_x(Horizontal::Center);

    container(contents)
        .width(Length::Shrink)
        .style(bb_theme::container::create_container_style(
            ContainerStyle::Default,
            None,
            None,
        ))
        .into()
}