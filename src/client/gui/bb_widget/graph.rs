use crate::client::gui::bb_theme::color::{
    CONTAINER_COLOR, DASHED_LINES_COLOR, HIGHLIGHTED_CONTAINER_COLOR,
};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use chrono::NaiveDate;
use iced::Renderer;
use iced::advanced::graphics::Gradient;
use iced::advanced::graphics::geometry::Frame;
use iced::advanced::graphics::gradient::Linear;
use iced::mouse;
use iced::widget::canvas::{
    Cache, Event, Geometry, LineCap, LineDash, LineJoin, Path, Stroke, event, stroke,
};
use iced_core::{Color, color};
use std::time::Duration;

use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_text_button};
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::bb_widget::chart::{CHART_WIDGET_HEIGHT, CHART_WIDGET_WIDTH, ChartTypes};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use crate::common::exercise_mod::exercise::ExerciseDataPoints;
use crate::common::exercise_mod::weight::Kg;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::Element;
use iced::widget::{Space, container};
use iced::widget::{canvas, row, text};
use iced_anim::{Animated, Animation, Motion};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::gradient::ColorStop;
use iced_core::keyboard::Key;
use iced_core::mouse::Cursor;
use iced_core::{Length, Point, Rectangle, Size, Theme};

const GRAPH_PADDING: f32 = 50.0;
const AXIS_FONT_SIZE: f32 = 12.0;
const FREQUENCY_OF_Y_AXIS_LABELS: u32 = 12;
const AXIS_THICKNESS: f32 = 5.0;
pub const MAX_AMOUNT_POINTS: u8 = 40;
pub const MAX_X_LABELS: u8 = 11;

#[derive(Clone, Debug)]
pub enum GraphMessage {
    GraphCursorMoved(Point),
    GraphKeyPressed(Key),
    IncrementCounter,
    DecrementCounter,
    UpdateAnimatedSelection(iced_anim::Event<f32>),
}

#[derive(Default)]
pub struct GraphWidgetState {
    graph_cache: Cache,
    pub animation_progress: Animated<f32>,
    visible_points: bool,
    visible_cursor_information: bool,
    points_to_draw: u8,
    pub shown_chart_type: ChartTypes,
}

impl GraphWidgetState {
    pub fn new() -> Self {
        let animation_motion = Motion {
            response: Duration::from_millis(3000),
            damping: Motion::SMOOTH.damping(),
        };

        GraphWidgetState {
            graph_cache: Cache::default(),
            animation_progress: Animated::new(0.0, animation_motion),
            visible_points: true,
            visible_cursor_information: true,
            points_to_draw: 9,
            shown_chart_type: ChartTypes::default(),
        }
    }
    pub fn invert_visible_points(&mut self) {
        self.visible_points = !self.visible_points
    }
    pub fn invert_visible_cursor_information(&mut self) {
        self.visible_cursor_information = !self.visible_cursor_information
    }

    pub(crate) fn increment_counter(&mut self) {
        self.points_to_draw += 1;
    }

    pub(crate) fn decrement_counter(&mut self) {
        self.points_to_draw -= 1;
    }
    pub(crate) fn get_counter(&self) -> u8 {
        self.points_to_draw
    }
    pub(crate) fn update_graph(&self) {
        self.graph_cache.clear();
    }
}
pub struct GraphWidget<'a> {
    active_mascot: Mascot,
    exercise_manager: &'a ExerciseManager,
    graph_state: &'a GraphWidgetState,
}

impl<'a> GraphWidget<'a> {
    pub(crate) fn new(app: &'a App) -> Self {
        GraphWidget {
            active_mascot: app.mascot_manager.selected_mascot,
            exercise_manager: &app.exercise_manager,
            graph_state: &app.graph_widget_state,
        }
    }

    pub(crate) fn view(self) -> Element<'a, Message> {
        let draw_percentage = &self.graph_state.animation_progress;

        let canvas = canvas(self)
            .width(CHART_WIDGET_WIDTH)
            .height(CHART_WIDGET_HEIGHT);

        Animation::new(draw_percentage, canvas)
            .on_update(|event| Message::Graph(GraphMessage::UpdateAnimatedSelection(event)))
            .into()
    }
}

fn draw_dashed_lines(animation_percentage: f32, frame: &mut Frame<Renderer>) {
    let gradient_padding = GRAPH_PADDING + 6.0;

    let gradient = Gradient::Linear(
        Linear::new(
            Point {
                x: gradient_padding,
                y: frame.height() / 2.0,
            },
            Point {
                x: frame.width(),
                y: frame.height() / 2.0,
            },
        )
        .add_stops([
            // FADE IN
            ColorStop {
                offset: 0.0,
                color: Color {
                    a: 0.0,
                    ..DASHED_LINES_COLOR
                },
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
                color: Color {
                    a: 0.0,
                    ..DASHED_LINES_COLOR
                },
            },
        ]),
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

    //PADDING UP AND DOWN:  UP FOR Y-AXIS-ARROW SPACE,DOWN FOR X-LABELS
    let block_height =
        (CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0) / FREQUENCY_OF_Y_AXIS_LABELS as f32;

    let current_x = CHART_WIDGET_WIDTH - GRAPH_PADDING; //START OF GRAPH
    let mut current_y = Point::ORIGIN.y + GRAPH_PADDING;

    let mut draw_stroke = |start: Point, end: Point, stroke: Stroke| {
        frame.stroke(&Path::line(start, end), stroke);
    };

    //VERTICAL LINE
    let point_up = Point {
        x: Point::ORIGIN.x + current_x,
        y: Point::ORIGIN.y + GRAPH_PADDING,
    };

    let point_bottom = Point {
        x: Point::ORIGIN.x + current_x,
        y: (CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0) * animation_percentage + GRAPH_PADDING,
    };

    draw_stroke(point_up, point_bottom, dashed_stroke);

    //HORIZONTAL LINES

    for _line in 1..=FREQUENCY_OF_Y_AXIS_LABELS {
        let point_left = Point {
            x: Point::ORIGIN.x + GRAPH_PADDING,
            y: Point::ORIGIN.y + current_y,
        };

        let point_right = Point {
            x: CHART_WIDGET_WIDTH - GRAPH_PADDING,
            y: Point::ORIGIN.y + current_y,
        };

        draw_stroke(point_left, point_right, dashed_stroke);
        current_y += block_height;
    }
}

fn draw_axis(animation_progress: f32, active_mascot: &Mascot, frame: &mut Frame<Renderer>) {
    //LATER CREATE PARAMETER VIRTUAL GRAPH BOUNDS
    let axis_color = active_mascot.get_primary_color();

    let solid_mascot_colored_stroke = Stroke {
        width: AXIS_THICKNESS,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Solid(axis_color),
        line_dash: Default::default(),
    };

    //Y-AXIS
    frame.stroke(
        &Path::line(
            //TOP
            Point {
                x: Point::ORIGIN.x + GRAPH_PADDING,
                y: Point::ORIGIN.y + GRAPH_PADDING,
            },
            //BOTTOM
            Point {
                x: Point::ORIGIN.x + GRAPH_PADDING,
                y: (frame.height() - GRAPH_PADDING * 2.0) * animation_progress + GRAPH_PADDING,
            },
        ),
        solid_mascot_colored_stroke,
    );

    //X-AXIS
    frame.stroke(
        &Path::line(
            //LEFT
            Point {
                x: Point::ORIGIN.x
                    + (frame.width() - GRAPH_PADDING * 2.0) * (1.0 - animation_progress)
                    + GRAPH_PADDING,
                y: frame.height() - GRAPH_PADDING,
            },
            //RIGHT
            Point {
                x: frame.width() - GRAPH_PADDING,
                y: frame.height() - GRAPH_PADDING,
            },
        ),
        solid_mascot_colored_stroke,
    );

    //ARROW - Y
    let distance_from_tip = 10.0;

    let y_tip = Point {
        x: Point::ORIGIN.x + GRAPH_PADDING,
        y: Point::ORIGIN.y + GRAPH_PADDING,
    };
    let y_left = Point {
        x: y_tip.x - distance_from_tip,
        y: y_tip.y + distance_from_tip,
    };
    let y_right = Point {
        x: y_tip.x + distance_from_tip,
        y: y_tip.y + distance_from_tip,
    };

    //ARROW - X

    let x_tip = Point {
        x: frame.width() - GRAPH_PADDING,
        y: frame.height() - GRAPH_PADDING,
    };
    let x_up = Point {
        x: x_tip.x - distance_from_tip,
        y: x_tip.y - distance_from_tip,
    };
    let x_bottom = Point {
        x: x_tip.x - distance_from_tip,
        y: x_tip.y + distance_from_tip,
    };

    let mut draw_arrow = |point_1: Point, tip: Point, point_2: Point| {
        frame.stroke(
            &Path::new(|builder| {
                builder.move_to(point_1);
                builder.line_to(tip);
                builder.line_to(point_2);
            }),
            solid_mascot_colored_stroke,
        );
    };

    draw_arrow(y_left, y_tip, y_right);
    draw_arrow(x_up, x_tip, x_bottom);
}

fn draw_points(
    graph_widget_state: &GraphWidgetState,
    frame: &mut Frame<Renderer>,
    y_values: Vec<Kg>,
    mascot: &Mascot,
) {
    //!You should pass a list with the y-values,which are things such as weight lifted,kms run,etc.

    let points = calculate_points(graph_widget_state, y_values);
    let base_size_point = 3.0;
    let max_size_point = 7.0;
    let range_size_point = max_size_point - base_size_point;
    let percentage = points.len() as f32 / MAX_AMOUNT_POINTS as f32;

    //CALCULATE POINTS RADIUS
    let point_size = max_size_point - percentage * range_size_point;

    //DRAW POINTS
    for point in points.iter() {
        frame.fill(
            &Path::circle(
                *point,
                point_size * graph_widget_state.animation_progress.value(),
            ),
            mascot.get_primary_color(),
        );
    }
}

fn draw_connections(
    graph_widget_state: &GraphWidgetState,
    frame: &mut Frame<Renderer>,
    y_values: Vec<f32>,
    mascot: &Mascot,
) {
    //TODO: Make connections thicker when less points_to_draw

    let points = calculate_points(graph_widget_state, y_values);
    let base_size_stroke = 1.5;
    let max_size_stroke = 4.2;
    let range_size_stroke = max_size_stroke - base_size_stroke;
    let percentage = points.len() as f32 / MAX_AMOUNT_POINTS as f32;

    //CALCULATE POINTS RADIUS
    let stroke_size = max_size_stroke - percentage * range_size_stroke;

    let connection_stroke = Stroke {
        width: stroke_size * graph_widget_state.animation_progress.value(),
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        style: stroke::Style::Solid(mascot.get_secondary_color()),
        line_dash: Default::default(),
    };

    let point_tuples = points
        .iter()
        .enumerate()
        .map(|(index, y)| {
            let last_element = points.len() - 1;
            let path_start = y;
            let mut path_end = path_start;

            if index != last_element {
                path_end = &points[index + 1];
            }

            (*path_start, *path_end)
        })
        .collect::<Vec<(Point, Point)>>();

    //draw start line (line should be at the same height of first point)
    let point_on_y_axis = Point {
        x: Point::ORIGIN.x + GRAPH_PADDING,
        y: points[0].y, //it is sure that there is at least one point since the function doesn't get called if length() < 1
    };

    frame.stroke(
        &Path::line(point_on_y_axis, points[0]), //it is sure that there is at least one point since the function doesn't get called if length() < 1
        connection_stroke,
    );

    for (start, end) in point_tuples {
        frame.stroke(&Path::line(start, end), connection_stroke);
    }
}

fn draw_cursor_information(
    y_values: Vec<Kg>,
    graph_widget_state: &GraphWidgetState,
    bounds: Rectangle,
    cursor: Cursor,
    frame: &mut Frame<Renderer>,
) {
    //max_point is needed for scaling

    //SETUP GRAPH BOUNDS
    let mut graph_bounds = bounds;
    graph_bounds.x += GRAPH_PADDING;
    graph_bounds.y += GRAPH_PADDING;
    graph_bounds.height -= GRAPH_PADDING * 2.0;
    graph_bounds.width -= GRAPH_PADDING * 2.0;

    let graph_origin = Point {
        x: graph_bounds.x,
        y: graph_bounds.y + graph_bounds.height,
    };

    //PADDING UP AND DOWN:  UP FOR Y-AXIS-ARROW SPACE,DOWN FOR X-LABELS
    let block_height =
        (CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0) / FREQUENCY_OF_Y_AXIS_LABELS as f32;
    let chopped_y_values = chop_weights(graph_widget_state, y_values);

    let min_y: Kg = (chopped_y_values
        .iter()
        .map(|value| (*value * 10.0) as usize)
        .min()
        .unwrap_or(0)) as f32
        / 10.0; //TODO: USE FUNCTION IN weight.rs when connecting to ExerciseManager
    let max_y: Kg = (chopped_y_values
        .iter()
        .map(|value| (*value * 10.0) as usize)
        .max()
        .unwrap_or(0)) as f32
        / 10.0; //TODO: USE FUNCTION IN weight.rs when connecting to ExerciseManager
    let min_to_max_distance = max_y - min_y;

    let height_padding_for_arrow = block_height;
    let x_axis_padding = block_height;

    let height_graph_from_min_to_max: f32 =
        CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0 - height_padding_for_arrow - x_axis_padding;

    if graph_bounds.contains(cursor.position().unwrap_or_default()) {
        let cursor_position_in_graph =
            if let Some(mut position) = cursor.position_from(graph_origin) {
                position.y = -position.y; //invert y-coordinate since everything above position_from(graph_origin) is negative
                position.y -= x_axis_padding; //shifting everything one block above x-axis,first point starts after first block
                let position_percentage = position.y / height_graph_from_min_to_max; //percentage of the current position divided by the max_position possible
                let kg_position_y = min_y + position_percentage * min_to_max_distance; //weight_calculation: min_weight + percentage * delta(max_weight,min_weight)
                Point {
                    x: position.x,
                    y: kg_position_y,
                }
            } else {
                Point { x: 0.0, y: 0.0 }
            };

        let information_offset_from_cursor_x = -70.0;
        let information_offset_from_cursor_y = 50.0;

        let cursor_information_position =
            if let Some(mut position) = cursor.position_from(graph_origin) {
                position.y = position.y
                    + (CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0)
                    + information_offset_from_cursor_y;
                position.x += information_offset_from_cursor_x;
                position
            } else {
                Point { x: 0.0, y: 0.0 }
            };

        let cursor_information_shadow_position = Point {
            x: cursor_information_position.x + 10.0,
            y: cursor_information_position.y + 10.0,
        };

        let cursor_information_box_size = Size {
            width: 120.0,
            height: 60.0,
        };

        let shadow_color = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.5,
        };

        //SHADOW
        frame.fill(
            &Path::rounded_rectangle(
                cursor_information_shadow_position,
                cursor_information_box_size,
                10.0.into(),
            ),
            shadow_color,
        );

        //CONTAINER
        frame.fill(
            &Path::rounded_rectangle(
                cursor_information_position,
                cursor_information_box_size,
                10.0.into(),
            ),
            HIGHLIGHTED_CONTAINER_COLOR,
        );

        let mut cursor_information_text = cursor_information_position;
        //ADD PADDING
        cursor_information_text.x += cursor_information_box_size.width / 10.0;
        cursor_information_text.y += cursor_information_box_size.width / 10.0;

        let format_value: fn(f32) -> f32 = |value| (value * 10.0).round() / 10.0;

        frame.fill_text(canvas::Text {
            content: format!("{} Kg", format_value(cursor_position_in_graph.y)),
            size: 25.0.into(),
            position: cursor_information_text,
            color: bb_theme::color::TEXT_COLOR,
            font: FIRA_SANS_EXTRABOLD,
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Top,
            line_height: Default::default(),
            shaping: iced_core::text::Shaping::Advanced,
        });
    }
}

fn draw_axis_labels(
    frame: &mut Frame<Renderer>,
    graph_widget_state: &GraphWidgetState,
    exercise_data_points: &ExerciseDataPoints,
    mascot: &Mascot,
) {
    //X-AXIS
    let number_labels = match graph_widget_state.points_to_draw {
        ..=MAX_X_LABELS => graph_widget_state.points_to_draw,
        _ => MAX_X_LABELS, //MAX X-AXIS LABLES
    };

    let block_width = (CHART_WIDGET_WIDTH - GRAPH_PADDING * 2.0) / number_labels as f32; //division with 0 is not possible since limit is 1
    let mut current_x = Point::ORIGIN.x + GRAPH_PADDING + block_width;
    let height_text = CHART_WIDGET_HEIGHT - GRAPH_PADDING / 2.0;

    let chopped_dates = chop_dates(graph_widget_state, extract_dates(exercise_data_points));
    let amount_points = chopped_dates.len();

    let mut dates_for_axis = Vec::new();

    if amount_points <= number_labels as usize {
        for date in chopped_dates.iter() {
            dates_for_axis.push(date);
        }
    } else {
        let percentage = (amount_points - 1) as f32 / (number_labels - 1) as f32;

        for i in 0..number_labels {
            let index = (i as f32 * percentage).round() as usize;
            dates_for_axis.push(&chopped_dates[index]);
        }
    }

    for date in dates_for_axis {
        let formatted_date = date.format("%d.%m.%y").to_string();
        let position_text_x = Point {
            x: current_x,
            y: height_text,
        };

        frame.fill_text(canvas::Text {
            content: formatted_date,
            position: position_text_x,
            color: bb_theme::color::TEXT_COLOR,
            size: AXIS_FONT_SIZE.into(),
            line_height: Default::default(),
            font: FIRA_SANS_EXTRABOLD,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            shaping: Default::default(),
        });

        current_x += block_width;
    }

    // Y-AXIS
    let chopped_weights = chop_weights(graph_widget_state, extract_weights(exercise_data_points));
    let min_y: Kg = (chopped_weights
        .iter()
        .map(|value| (*value * 10.0) as usize)
        .min()
        .unwrap_or(0)) as f32
        / 10.0; //TODO: USE FUNCTION IN weight.rs when connecting to ExerciseManager
    let max_y: Kg = (chopped_weights
        .iter()
        .map(|value| (*value * 10.0) as usize)
        .max()
        .unwrap_or(0)) as f32
        / 10.0; //TODO: USE FUNCTION IN weight.rs when connecting to ExerciseManager
    let delta = max_y - min_y;
    let block_height =
        (CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0) / FREQUENCY_OF_Y_AXIS_LABELS as f32;
    let x_axis_padding = block_height;
    let height_padding_for_arrow = block_height;
    let start_point_labels = CHART_WIDGET_HEIGHT - GRAPH_PADDING - x_axis_padding;
    let height_graph_from_min_to_max: f32 =
        CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0 - height_padding_for_arrow - x_axis_padding;

    let label_amount = FREQUENCY_OF_Y_AXIS_LABELS - 1; //since on label is used for the "kg" label at the top of the graph, the amount has to be decreased by 1

    let format_value: fn(f32) -> f32 = |value| (value * 10.0).round() / 10.0;

    for i in 0..=(label_amount - 1) {
        //since i is used  for the percentage and it starts at 0, I have to decrease label_amount by 1 or otherwise one extra label would be drawn
        let percentage = i as f32 / (label_amount - 1) as f32; // Since the maximum value of i is (label_amount - 1), the denominator must also be (label_amount - 1) so that the percentage can reach 1.0
        let label_value = min_y + percentage * delta;

        let position_text_y = start_point_labels - percentage * height_graph_from_min_to_max;

        let position_text = Point {
            x: GRAPH_PADDING / 2.0,
            y: position_text_y,
        };

        frame.fill_text(canvas::Text {
            content: format!("{}", format_value(label_value)),
            position: position_text,
            color: color!(255, 255, 255),
            size: AXIS_FONT_SIZE.into(),
            font: FIRA_SANS_EXTRABOLD,
            horizontal_alignment: Horizontal::Center,
            vertical_alignment: Vertical::Center,
            line_height: Default::default(),
            shaping: Default::default(),
        });
    }

    let kg_label_position_y =
        start_point_labels - height_graph_from_min_to_max - height_padding_for_arrow;
    frame.fill_text(canvas::Text {
        content: "kg".to_string(),
        position: Point {
            x: GRAPH_PADDING / 2.0,
            y: kg_label_position_y,
        },
        color: mascot.get_secondary_color(),
        size: AXIS_FONT_SIZE.into(),
        font: FIRA_SANS_EXTRABOLD,
        horizontal_alignment: Horizontal::Center,
        vertical_alignment: Vertical::Center,
        line_height: Default::default(),
        shaping: Default::default(),
    });
}

//LOGIC --------------------

fn chop_weights(graph_widget_state: &GraphWidgetState, y_values: Vec<Kg>) -> Vec<Kg> {
    let chop_start_index: isize =
        y_values.len() as isize - graph_widget_state.points_to_draw as isize;

    let safe_chop_start_index = match chop_start_index {
        ..=0 => 0,
        _ => chop_start_index,
    };

    y_values[(safe_chop_start_index) as usize..].to_vec()
}

fn chop_dates(graph_widget_state: &GraphWidgetState, x_values: Vec<NaiveDate>) -> Vec<NaiveDate> {
    let chop_start_index: isize =
        x_values.len() as isize - graph_widget_state.points_to_draw as isize;

    let safe_chop_start_index = match chop_start_index {
        ..=0 => 0,
        _ => chop_start_index,
    };

    x_values[(safe_chop_start_index) as usize..].to_vec()
}

fn calculate_points(graph_widget_state: &GraphWidgetState, y_values: Vec<Kg>) -> Vec<Point> {
    let chopped_y_values = &chop_weights(graph_widget_state, y_values.clone());

    //PADDING LEFT AND RIGHT: LEFT FOR Y_LABELS, RIGHT FOR FREE SPACE
    let block_width =
        (CHART_WIDGET_WIDTH - GRAPH_PADDING * 2.0) / graph_widget_state.points_to_draw as f32; //division with 0 is not possible since limit is 1
    //PADDING UP AND DOWN:  UP FOR Y-AXIS-ARROW SPACE,DOWN FOR X-LABELS
    let block_height =
        (CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0) / FREQUENCY_OF_Y_AXIS_LABELS as f32;
    let mut current_x = CHART_WIDGET_WIDTH - GRAPH_PADDING;

    //I can unwrap() since the function is not going to get called if exercises.len() = 0
    let min_y: Kg = (chopped_y_values
        .iter()
        .map(|value| (*value * 10.0) as usize)
        .min()
        .unwrap_or(0)) as f32
        / 10.0; //TODO: USE FUNCTION IN weight.rs when connecting to ExerciseManager
    let max_y: Kg = (chopped_y_values
        .iter()
        .map(|value| (*value * 10.0) as usize)
        .max()
        .unwrap_or(0)) as f32
        / 10.0; //TODO: USE FUNCTION IN weight.rs when connecting to ExerciseManager
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
    let height_graph: f32 =
        CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0 - height_padding_for_arrow - x_axis_padding;
    let lowest_point_graph: f32 = CHART_WIDGET_HEIGHT - GRAPH_PADDING - x_axis_padding;

    //FORMULA: lowest_point-(y_min - current_y)/(max_y - min_y) * height_graph
    let calculate_graph_value =
        |y: &Kg| -> f32 { lowest_point_graph - (percentage(*y) * height_graph) };

    let mut new_y_values = vec![];

    let mut x_values = vec![];

    let mut points = vec![];

    //CALCULATE Y-VALUES
    for y_value in chopped_y_values {
        new_y_values.push(calculate_graph_value(y_value));
    }

    //CALCULATE X-VALUES
    for _x_value in 1..=graph_widget_state.points_to_draw {
        x_values.push(current_x);
        current_x -= block_width
    }
    x_values.reverse();

    //ZIP X- AND Y-VALUES
    for i in 0..chopped_y_values.len() {
        points.push(Point {
            x: x_values[i],
            y: new_y_values[i],
        })
    }

    points
}

fn extract_weights(exercise_data_points: &ExerciseDataPoints) -> Vec<Kg> {
    let mut weights: Vec<Kg> = vec![];
    for (_date, weight) in exercise_data_points {
        weights.push(*weight)
    }
    weights
}

fn extract_dates(exercise_data_points: &ExerciseDataPoints) -> Vec<NaiveDate> {
    let mut dates: Vec<NaiveDate> = vec![];
    for (date, _weight) in exercise_data_points {
        dates.push(*date)
    }

    dates
}

impl canvas::Program<Message> for GraphWidget<'_> {
    type State = GraphWidgetState; //not used yet

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        _bounds: Rectangle,
        _cursor: iced_core::mouse::Cursor,
    ) -> (event::Status, Option<Message>) {
        self.graph_state.update_graph();

        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => (
                iced::widget::canvas::event::Status::Captured,
                Some(Message::Graph(GraphMessage::GraphCursorMoved(position))),
            ),
            canvas::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => (
                iced::widget::canvas::event::Status::Captured,
                Some(Message::Graph(GraphMessage::GraphKeyPressed(key))),
            ),

            _ => (
                event::Status::Ignored,
                Some(Message::Graph(GraphMessage::UpdateAnimatedSelection(
                    iced_anim::Event::Target(1.0),
                ))),
            ),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let graph_widget = self
            .graph_state
            .graph_cache
            .draw(renderer, bounds.size(), |frame| {
                //TODO: Find right frame center to centrate "NO DATA" text
                //FRAME CENTER
                let center_x = CHART_WIDGET_WIDTH / 2.0 - INDENT;
                let center_y = CHART_WIDGET_HEIGHT / 2.0;
                let center = Point {
                    x: center_x,
                    y: center_y,
                };

                //DRAW BACKGROUND
                let background = Path::rectangle(
                    Point {
                        x: Point::ORIGIN.x + frame.width() - GRAPH_PADDING, //setting top right, so that the animation goes along the x-axis animation
                        y: Point::ORIGIN.y + GRAPH_PADDING,
                    },
                    Size {
                        width: -((frame.width() - GRAPH_PADDING * 2.0)
                            * self.graph_state.animation_progress.value()), //negative width, so that the animation goes along the x-axis animation
                        height: frame.height() - GRAPH_PADDING * 2.0,
                    },
                );

                match self.exercise_manager.data_points.len() {
                    0 => frame.fill_text(canvas::Text {
                        content: "NO DATA".to_string(),
                        position: center,
                        color: color!(142, 142, 147),
                        size: 40.into(),
                        line_height: Default::default(),
                        font: FIRA_SANS_EXTRABOLD,
                        horizontal_alignment: Horizontal::Left,
                        vertical_alignment: Vertical::Top,
                        shaping: Default::default(),
                    }),
                    _data_points_amount => {
                        let weights = extract_weights(&self.exercise_manager.data_points);

                        let background_gradient = Gradient::Linear(
                            Linear::new(
                                Point {
                                    x: frame.width() / 2.0,
                                    y: Point::ORIGIN.y + GRAPH_PADDING,
                                },
                                Point {
                                    x: frame.width() / 2.0,
                                    y: frame.height() - GRAPH_PADDING,
                                },
                            )
                            .add_stops([
                                // FADE IN
                                ColorStop {
                                    offset: 0.80
                                        + (0.20
                                            * (1.0 - self.graph_state.animation_progress.value())),
                                    color: Color {
                                        a: 0.0,
                                        ..CONTAINER_COLOR
                                    },
                                },
                                //ACTUAL COLOR
                                ColorStop {
                                    offset: 1.0,
                                    color: Color {
                                        a: 0.3,
                                        ..self.active_mascot.get_primary_color()
                                    },
                                },
                            ]),
                        );

                        frame.fill(&background, background_gradient);

                        //LABELS
                        draw_axis_labels(
                            frame,
                            self.graph_state,
                            &self.exercise_manager.data_points,
                            &self.active_mascot,
                        );

                        //DASHED LINES
                        draw_dashed_lines(*self.graph_state.animation_progress.value(), frame);

                        //CONNECTIONS BETWEEN POINTS
                        draw_connections(
                            self.graph_state,
                            frame,
                            weights.clone(),
                            &self.active_mascot,
                        );

                        let draw_percentage = self.graph_state.animation_progress.value();
                        //AXIS
                        draw_axis(*draw_percentage, &self.active_mascot, frame);

                        //POINTS
                        if self.graph_state.visible_points {
                            draw_points(
                                self.graph_state,
                                frame,
                                weights.clone(),
                                &self.active_mascot,
                            );
                        }

                        //println!("{:?}", weights);

                        //CURSOR
                        if self.graph_state.visible_cursor_information {
                            draw_cursor_information(
                                weights,
                                self.graph_state,
                                bounds,
                                cursor,
                                frame,
                            ); //unwrap() can't fail since the list can't be empty
                        }
                    }
                }
            });

        vec![graph_widget]
    }
}

pub fn view_graph_widget_settings<'a>(app: &App) -> Element<'a, Message> {
    let mut counter = format_button_text(text!("{}", app.graph_widget_state.points_to_draw));
    counter = counter.size(19);

    let increment_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        "+".to_string(),
        ButtonStyle::Active,
        Some(10.0.into()),
    )
    .on_press(Message::Graph(GraphMessage::IncrementCounter));

    let decrement_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        "-".to_string(),
        ButtonStyle::Active,
        Some(10.0.into()),
    )
    .on_press(Message::Graph(GraphMessage::DecrementCounter));

    let row_counter_with_buttons = row![
        decrement_button,
        Space::with_width(Length::FillPortion(1)),
        counter,
        Space::with_width(Length::FillPortion(1)),
        increment_button,
    ]
    .align_y(Vertical::Center);

    let counter_with_buttons = container(row_counter_with_buttons)
        .style(create_container_style(
            ContainerStyle::Highlighted,
            Some(10.into()),
            None,
        ))
        .width(Length::Fixed(100.0));

    counter_with_buttons.into()
}
