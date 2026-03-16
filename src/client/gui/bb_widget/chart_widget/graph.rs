use crate::client::backend::pop_up_manager::PopUpType;
use crate::client::backend::widget_state::widget_state_manager::WidgetMessage::Chart;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::color::{
    CONTAINER_COLOR, DARK_SHADOW, DASHED_LINES_COLOR, DESCRIPTION_TEXT_COLOR,
    HIGHLIGHTED_CONTAINER_COLOR, TEXT_COLOR, create_canvas_gradient, create_color_stops,
    create_gradient_stroke_style, create_solid_stroke_style, transform_alpha,
};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::canvas_utils::{
    draw_line, draw_text, generate_dashed_stroke, generate_stroke, translate_point,
};
use crate::client::gui::bb_widget::chart_widget::chart::{
    CHART_WIDGET_HEIGHT, CHART_WIDGET_WIDTH, ChartMessage, ChartTypes, DataPointsType,
};
use crate::client::gui::bb_widget::chart_widget::graph_logic::{
    calculate_points, chop_dates, chop_weights, extract_dates, extract_weights, get_f32_max,
    get_f32_min,
};
use crate::client::gui::size::FRAME_WIDTH;
use crate::client::gui::user_interface::Message;
use crate::client::gui::user_interface::Message::Widget;
use crate::common::exercise_mod::exercise::DateWeightPoints;
use crate::common::exercise_mod::weight::Kg;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::user_mod::user_goals::GoalType;
use chrono::NaiveDate;
use iced::Element;
use iced::Renderer;
use iced::advanced::graphics::geometry::Frame;
use iced::widget::canvas::{Cache, Event, Geometry, Path};
use iced::widget::{Action, canvas};
use iced::{Task, mouse};
use iced_anim::{Animated, Animation, Motion};
use iced_core::Color;
use iced_core::alignment::Vertical;
use iced_core::keyboard::Key;
use iced_core::mouse::Cursor;
use iced_core::text::Alignment;
use iced_core::{Point, Rectangle, Size, Theme};
use std::time::Duration;

pub(crate) const GRAPH_PADDING: f32 = 50.0;
const AXIS_FONT_SIZE: f32 = 12.0;
pub(crate) const FREQUENCY_OF_Y_AXIS_LABELS: u32 = 12;
const AXIS_THICKNESS: f32 = 5.0;
pub const MAX_AMOUNT_POINTS: u8 = 40;
pub const MAX_X_LABELS: u8 = 11;
pub const MAX_VERTICAL_LINES: u8 = 14;
pub const SHADOW_OFFSET: f32 = 5.0;
pub const CURSOR_BOX_WIDTH: f32 = 120.0;
pub const CURSOR_BOX_HEIGHT: f32 = 60.0;

//PADDING TOP AND BOTTOM: TOP FOR Y-AXIS-ARROW SPACE, BOTTOM FOR X-LABELS
pub static BLOCK_HEIGHT: f32 =
    (CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0) / FREQUENCY_OF_Y_AXIS_LABELS as f32;
pub static GRAPH_HEIGHT: f32 = CHART_WIDGET_HEIGHT - GRAPH_PADDING * 2.0;
pub static GRAPH_WIDTH: f32 = CHART_WIDGET_WIDTH - GRAPH_PADDING * 2.0;
pub static GRAPH_START_X: f32 = GRAPH_PADDING;
pub static GRAPH_END_X: f32 = CHART_WIDGET_WIDTH - GRAPH_PADDING;
pub static GRAPH_START_Y: f32 = GRAPH_PADDING; // Y-coordinate where the graph starts (top + padding)
pub static GRAPH_END_Y: f32 = CHART_WIDGET_HEIGHT - GRAPH_PADDING; // Y-coordinate where the graph ends (bottom - padding)
#[derive(Clone, Debug)]
pub enum GraphMessage {
    GraphCursorMoved(Point),
    GraphKeyPressed(Key),
    IncrementCounter,
    DecrementCounter,
    AnimateGraph(iced_anim::Event<f32>),
    ToggleDots,
    ToggleCursor,
    ToggleVerticalLines,
}
impl GraphMessage {
    pub fn update_graph(
        data_points_type: DataPointsType,
        graph_message: GraphMessage,
        app: &mut App,
    ) -> Task<Message> {
        let graph_state = match data_points_type {
            DataPointsType::Exercise(_) => &mut app.widget_manager.exercise_graph_widget_state,
            DataPointsType::Health(_, _) => &mut app.widget_manager.health_graph_widget_state,
        };

        match graph_message {
            GraphMessage::GraphCursorMoved(_point) => {}

            GraphMessage::GraphKeyPressed(Key::Character(char)) => match char.as_str() {
                "d" => graph_state.invert_visible_points(),
                "c" => graph_state.invert_visible_cursor_information(),
                "v" => graph_state.invert_visible_vertical_lines(),
                "b" => graph_state.data_points_type = DataPointsType::Exercise(ChartTypes::Bar),
                _ => {}
            },
            GraphMessage::IncrementCounter => {
                if graph_state.get_counter() < MAX_AMOUNT_POINTS {
                    graph_state.increment_counter();
                } else {
                    app.pop_up_manager.new_pop_up(
                        PopUpType::Minor,
                        "Limit reached ".to_string(),
                        format!("The graph can’t display more than {MAX_AMOUNT_POINTS} points"),
                    );
                }
            }
            GraphMessage::DecrementCounter => {
                if graph_state.get_counter() > 1 {
                    graph_state.decrement_counter();
                }
            }
            GraphMessage::AnimateGraph(event) => {
                graph_state.animation_progress.update(event);
                graph_state.update_graph();
            }

            GraphMessage::ToggleDots => graph_state.invert_visible_points(),

            GraphMessage::ToggleCursor => graph_state.invert_visible_cursor_information(),

            GraphMessage::ToggleVerticalLines => graph_state.invert_visible_vertical_lines(),

            GraphMessage::GraphKeyPressed(_) => {} //other key_enums
        };
        Task::none()
    }
}

#[derive(Default)]
pub struct GraphWidgetState {
    graph_cache: Cache,
    pub animation_progress: Animated<f32>,
    pub(crate) visible_points: bool,
    pub(crate) visible_cursor_information: bool,
    pub(crate) visible_vertical_lines: bool,
    pub(crate) points_to_draw: u8,
    pub data_points_type: DataPointsType,
}

impl GraphWidgetState {
    pub fn new(data_points_type: DataPointsType) -> Self {
        let animation_motion = Motion {
            response: Duration::from_millis(3000),
            damping: Motion::SMOOTH.damping(),
        };

        GraphWidgetState {
            graph_cache: Cache::default(),
            animation_progress: Animated::new(0.0, animation_motion),
            visible_points: true,
            visible_cursor_information: true,
            visible_vertical_lines: true,
            points_to_draw: 9,
            data_points_type,
        }
    }
    pub fn invert_visible_points(&mut self) {
        self.visible_points = !self.visible_points
    }
    pub fn invert_visible_cursor_information(&mut self) {
        self.visible_cursor_information = !self.visible_cursor_information
    }

    pub fn invert_visible_vertical_lines(&mut self) {
        self.visible_vertical_lines = !self.visible_vertical_lines
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
    data_points: &'a Vec<(NaiveDate, f32)>,
    graph_state: &'a GraphWidgetState,
}

impl<'a> GraphWidget<'a> {
    pub(crate) fn new(
        graph_widget_state: &'a GraphWidgetState,
        data_points: &'a Vec<(chrono::NaiveDate, f32)>,
        mascot: Mascot,
    ) -> Self {
        GraphWidget {
            active_mascot: mascot,
            data_points,
            graph_state: graph_widget_state,
        }
    }

    pub(crate) fn view(self) -> Element<'a, Message> {
        let draw_percentage = &self.graph_state.animation_progress;
        let data_points_type = self.graph_state.data_points_type;

        let canvas = canvas(self)
            .width(CHART_WIDGET_WIDTH)
            .height(CHART_WIDGET_HEIGHT);

        Animation::new(draw_percentage, canvas)
            .on_update(move |event| {
                Widget(Chart(ChartMessage::Graph(
                    data_points_type,
                    GraphMessage::AnimateGraph(event),
                )))
            })
            .into()
    }
}

fn draw_background(frame: &mut Frame<Renderer>, graph_widget: &GraphWidget) {
    let center_x = GRAPH_WIDTH / 2.0;

    let background = Path::rectangle(
        Point {
            x: GRAPH_END_X, //setting top right, so that the animation goes along the x-axis animation
            y: GRAPH_START_Y,
        },
        Size {
            width: -GRAPH_WIDTH * graph_widget.graph_state.animation_progress.value(), //negative width, so that the animation goes along the x-axis animation
            height: GRAPH_HEIGHT,
        },
    );

    let background_gradient_start = Point {
        x: center_x,
        y: GRAPH_START_Y,
    };
    let background_gradient_end = Point {
        x: center_x,
        y: GRAPH_END_Y,
    };

    let background_color_start = Color {
        a: 0.0,
        ..CONTAINER_COLOR
    };
    let background_color_end = Color {
        a: 0.3,
        ..graph_widget.active_mascot.get_primary_color()
    };

    let background_color_stops = create_color_stops(vec![
        (
            background_color_start,
            0.80 + (0.20 * (1.0 - graph_widget.graph_state.animation_progress.value())),
        ),
        (background_color_end, 1.0),
    ]);

    let background_gradient = create_canvas_gradient(
        background_gradient_start,
        background_gradient_end,
        background_color_stops,
    );

    frame.fill(&background, background_gradient);
}

///Draws the dashed lines in the background of the graph which adapt themselves to the amount of `points_to_draw`
fn draw_dashed_lines(graph_widget_state: &GraphWidgetState, frame: &mut Frame<Renderer>) {
    let frame_center_y = frame.height() / 2.0;
    let frame_center_x = frame.width() / 2.0;

    let horizontal_gradient_padding = GRAPH_PADDING + 6.0;
    let vertical_gradient_padding = GRAPH_PADDING + 10.0;

    let fade_in_out_color = Color {
        a: 0.0,
        ..DASHED_LINES_COLOR
    };

    let color_stops = create_color_stops(vec![
        (fade_in_out_color, 0.0),
        (DASHED_LINES_COLOR, 0.1),
        (DASHED_LINES_COLOR, 0.9),
        (fade_in_out_color, 1.0),
    ]);

    //HORIZONTAL GRADIENT
    let horizontal_gradient_start = Point::new(horizontal_gradient_padding, frame_center_y);
    let horizontal_gradient_end = Point::new(frame.width(), frame_center_y);

    let horizontal_gradient = create_canvas_gradient(
        horizontal_gradient_start,
        horizontal_gradient_end,
        color_stops.clone(),
    );

    //VERTICAL GRADIENT
    let vertical_gradient_start = Point::new(frame_center_x, vertical_gradient_padding);
    let vertical_gradient_end =
        Point::new(frame_center_x, frame.height() - vertical_gradient_padding);

    let vertical_gradient =
        create_canvas_gradient(vertical_gradient_start, vertical_gradient_end, color_stops);

    let horizontal_dashed_stroke =
        generate_dashed_stroke(2.0, create_gradient_stroke_style(horizontal_gradient));
    let vertical_dashed_stroke =
        generate_dashed_stroke(2.0, create_gradient_stroke_style(vertical_gradient));

    let y_axis_arrow_padding = GRAPH_HEIGHT / FREQUENCY_OF_Y_AXIS_LABELS as f32;

    let range = match graph_widget_state.points_to_draw {
        ..=MAX_VERTICAL_LINES => graph_widget_state.points_to_draw,
        _ => MAX_VERTICAL_LINES,
    };

    //PADDING LEFT AND RIGHT: LEFT FOR Y_LABELS, RIGHT FOR FREE SPACE
    let block_width = (CHART_WIDGET_WIDTH - GRAPH_PADDING * 2.0) / range as f32; //division with 0 is not possible since limit is 1
    let block_height = BLOCK_HEIGHT;

    let mut current_x = CHART_WIDGET_WIDTH - GRAPH_PADDING; //START OF GRAPH
    let mut current_y = GRAPH_START_Y + y_axis_arrow_padding;

    //VERTICAL LINES
    if graph_widget_state.visible_vertical_lines {
        for _line in 1..=range {
            let point_top = Point {
                x: current_x,
                y: GRAPH_START_Y,
            };

            let point_bottom = Point {
                x: current_x,
                y: CHART_WIDGET_WIDTH - GRAPH_PADDING,
            };

            draw_line(frame, point_top, point_bottom, vertical_dashed_stroke);
            current_x -= block_width;
        }
    }
    //HORIZONTAL LINES

    for _line in 1..=FREQUENCY_OF_Y_AXIS_LABELS - 1 {
        // -1 since we the dashed line at the height of the y-axis arrow should not be drawn
        let point_left = Point {
            x: GRAPH_START_X,
            y: current_y,
        };

        let point_right = Point {
            x: CHART_WIDGET_WIDTH - GRAPH_PADDING,
            y: current_y,
        };

        draw_line(frame, point_left, point_right, horizontal_dashed_stroke);
        current_y += block_height;
    }
}

/// Draws the x- and y-axis in the given `frame`.
///
/// The `animation_progress` parameter controls the animation of the axes.
/// It is expected to be a value between `0.0` and `1.0`, and is used to
/// multiply the axis lengths so that they appear to grow progressively
/// as the animation progresses.
fn draw_axis(animation_progress: f32, active_mascot: &Mascot, frame: &mut Frame<Renderer>) {
    let axis_color = active_mascot.get_primary_color();
    let solid_mascot_colored_stroke =
        generate_stroke(AXIS_THICKNESS, create_solid_stroke_style(axis_color));

    //Y-AXIS
    let start_y_axis = Point {
        x: GRAPH_START_X,
        y: GRAPH_START_Y,
    };

    let end_y_axis = Point {
        x: GRAPH_START_X,
        y: GRAPH_HEIGHT * animation_progress + GRAPH_PADDING,
    };

    draw_line(frame, start_y_axis, end_y_axis, solid_mascot_colored_stroke);

    //X-AXIS
    let start_x_axis = Point {
        x: GRAPH_PADDING + GRAPH_WIDTH * (1.0 - animation_progress),

        y: GRAPH_END_Y,
    };

    let end_x_axis = Point {
        x: CHART_WIDGET_WIDTH - GRAPH_PADDING,
        y: GRAPH_END_Y,
    };

    draw_line(frame, start_x_axis, end_x_axis, solid_mascot_colored_stroke);

    //ARROW Y-AXIS
    let distance_from_tip = 10.0 * animation_progress;

    let y_tip = Point {
        x: GRAPH_START_X,
        y: GRAPH_START_Y,
    };
    let y_left = Point {
        x: y_tip.x - distance_from_tip,
        y: y_tip.y + distance_from_tip,
    };
    let y_right = Point {
        x: y_tip.x + distance_from_tip,
        y: y_tip.y + distance_from_tip,
    };

    //ARROW X-AXIS

    let x_tip = Point {
        x: GRAPH_END_X,
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

// Draws the data points of a graph onto the given `frame`
fn draw_points(
    graph_widget_state: &GraphWidgetState,
    frame: &mut Frame<Renderer>,
    y_values: Vec<Kg>,
    mascot: &Mascot,
) {
    let points = calculate_points(graph_widget_state, y_values);
    let min_size_point = 3.0;
    let max_size_point = 7.0;
    let range_size_point = max_size_point - min_size_point;
    let ratio = points.len() as f32 / MAX_AMOUNT_POINTS as f32;

    //CALCULATE POINTS RADIUS
    let point_size = max_size_point - ratio * range_size_point;

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

///Draws the connections between the points in `y_values`
fn draw_connections(
    graph_widget_state: &GraphWidgetState,
    frame: &mut Frame<Renderer>,
    y_values: Vec<f32>,
    mascot: &Mascot,
) {
    let mut points = calculate_points(graph_widget_state, y_values);
    let min_size_stroke = 1.5;
    let max_size_stroke = 4.2;
    let range_size_stroke = max_size_stroke - min_size_stroke;
    let ratio = points.len() as f32 / MAX_AMOUNT_POINTS as f32;

    let stroke_width = max_size_stroke - ratio * range_size_stroke;

    let connection_stroke = generate_stroke(
        stroke_width * graph_widget_state.animation_progress.value(),
        create_solid_stroke_style(mascot.get_secondary_color()),
    );

    let shadow_color = transform_alpha(0.25, Color::BLACK);
    let shadow_stroke = generate_stroke(
        (stroke_width + 2.0) * graph_widget_state.animation_progress.value(),
        create_solid_stroke_style(shadow_color),
    );

    // Draw the first connection from the first point in `points` to the y-axis at the same height
    let point_on_y_axis = Point {
        x: GRAPH_START_X,
        y: points[0].y, //it is sure that there is at least one point since the function doesn't get called if length() < 1
    };

    points.insert(0, point_on_y_axis);

    //a list of tuples of the point n in y_values in a tuple with n + 1 in points
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

    for (start, end) in point_tuples {
        frame.stroke(
            &Path::line(
                translate_point(start, SHADOW_OFFSET),
                translate_point(end, SHADOW_OFFSET),
            ),
            shadow_stroke,
        );
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

    //PADDING UP AND DOWN: UP FOR Y-AXIS-ARROW SPACE, DOWN FOR X-LABELS
    let block_height = GRAPH_HEIGHT / FREQUENCY_OF_Y_AXIS_LABELS as f32;
    let chopped_y_values = chop_weights(graph_widget_state, y_values);

    let min_y: Kg = get_f32_min(&chopped_y_values);

    //max_point is needed for scaling
    let max_y: Kg = get_f32_max(&chopped_y_values);
    let min_to_max_distance = max_y - min_y;

    let height_padding_for_arrow = block_height;
    let x_axis_padding = block_height;

    let height_graph_from_min_to_max: f32 =
        GRAPH_HEIGHT - height_padding_for_arrow - x_axis_padding;

    let mut information_offset_from_cursor_x = -70.0;
    let mut information_offset_from_cursor_y = 50.0;
    let cursor_adjust_x = 15.0; //the bigger, the more left
    let cursor_adjust_y = 38.5; //the bigger, the lower

    if graph_bounds.contains(cursor.position().unwrap_or_default()) {
        let cursor_position_in_graph = if let Some(mut position) =
            cursor.position_from(graph_origin)
        {
            position.y = -position.y; //invert y-coordinate since everything above position_from(graph_origin) is negative
            position.y -= x_axis_padding; //shifting everything one block above x-axis,first point starts after first block

            //cursor box position handling
            if position.x < CURSOR_BOX_WIDTH || position.y + x_axis_padding < CURSOR_BOX_HEIGHT {
                information_offset_from_cursor_x =
                    -(information_offset_from_cursor_x + cursor_adjust_x);
                information_offset_from_cursor_y =
                    -(information_offset_from_cursor_y - cursor_adjust_y);
            }

            //cursor text value
            let position_percentage = position.y / height_graph_from_min_to_max; //percentage of the current position divided by the max_position possible
            let kg_position_y = min_y + position_percentage * min_to_max_distance; //weight_calculation: min_weight + percentage * delta(max_weight,min_weight)
            Point {
                x: position.x,
                y: kg_position_y,
            }
        } else {
            Point::ORIGIN
        };

        let cursor_information_position =
            if let Some(mut position) = cursor.position_from(graph_origin) {
                position.y = position.y + (GRAPH_HEIGHT) + information_offset_from_cursor_y;
                position.x += information_offset_from_cursor_x;
                position
            } else {
                Point::ORIGIN
            };

        let cursor_information_shadow_position = Point {
            x: cursor_information_position.x + 10.0,
            y: cursor_information_position.y + 10.0,
        };

        let cursor_information_box_size = Size {
            width: CURSOR_BOX_WIDTH,
            height: CURSOR_BOX_HEIGHT,
        };
        let shadow_color = Color {
            a: 0.5,
            ..DARK_SHADOW
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
        let content = format!("{} Kg", format_value(cursor_position_in_graph.y));

        frame.fill_text(canvas::Text {
            content,
            size: 25.0.into(),
            position: cursor_information_text,
            max_width: FRAME_WIDTH,
            color: bb_theme::color::TEXT_COLOR,
            font: FIRA_SANS_EXTRABOLD,
            align_x: Alignment::Left,
            line_height: Default::default(),
            shaping: iced_core::text::Shaping::Advanced,
            align_y: Vertical::Top,
        });
    }
}

fn draw_axis_labels(
    frame: &mut Frame<Renderer>,
    graph_widget_state: &GraphWidgetState,
    exercise_data_points: &DateWeightPoints,
    mascot: &Mascot,
) {
    //X-AXIS
    let number_labels = match graph_widget_state.points_to_draw {
        ..=MAX_X_LABELS => graph_widget_state.points_to_draw,
        _ => MAX_X_LABELS, //MAX X-AXIS LABELS
    };

    let block_width = GRAPH_WIDTH / number_labels as f32; //division with 0 is not possible since limit is 1
    let mut current_x = GRAPH_START_X + block_width;
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

        draw_text(
            frame,
            formatted_date,
            AXIS_FONT_SIZE,
            position_text_x,
            TEXT_COLOR,
        );

        current_x += block_width;
    }

    // Y-AXIS
    let chopped_weights = chop_weights(graph_widget_state, extract_weights(exercise_data_points));
    let min_y: Kg = get_f32_min(&chopped_weights);
    let max_y: Kg = get_f32_max(&chopped_weights);

    let delta = max_y - min_y;
    let block_height = BLOCK_HEIGHT;
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

        draw_text(
            frame,
            format!("{}", format_value(label_value)),
            AXIS_FONT_SIZE,
            position_text,
            TEXT_COLOR,
        );
    }

    let kg_label_position_y =
        start_point_labels - height_graph_from_min_to_max - height_padding_for_arrow;

    let kg_label_position = Point {
        x: GRAPH_PADDING / 2.0,
        y: kg_label_position_y,
    };

    draw_text(
        frame,
        "kg".to_string(),
        AXIS_FONT_SIZE,
        kg_label_position,
        mascot.get_secondary_color(),
    );
}

impl canvas::Program<Message> for GraphWidget<'_> {
    type State = GraphWidgetState; //not used yet

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        _bounds: Rectangle,
        _cursor: iced_core::mouse::Cursor,
    ) -> Option<Action<Message>> {
        self.graph_state.update_graph();

        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some(Action::publish(Widget(Chart(ChartMessage::Graph(
                    self.graph_state.data_points_type,
                    GraphMessage::GraphCursorMoved(*position),
                )))))
            }

            canvas::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => {
                Some(Action::publish(Widget(Chart(ChartMessage::Graph(
                    self.graph_state.data_points_type,
                    GraphMessage::GraphKeyPressed(key.clone()),
                )))))
            }

            _ => Some(Action::publish(Widget(Chart(ChartMessage::Graph(
                self.graph_state.data_points_type,
                GraphMessage::AnimateGraph(iced_anim::Event::Target(1.0)),
            ))))),
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
                //FRAME CENTER
                let center_x = frame.width() / 2.0;
                let center_y = frame.height() / 2.0;
                let center = Point::new(center_x, center_y);

                match self.graph_state.data_points_type {
                    DataPointsType::Health(_, GoalType::Water)
                    | DataPointsType::Health(_, GoalType::Sleep) => {
                        draw_text(
                            frame,
                            "Coming soon".to_string(),
                            40.0,
                            center,
                            DESCRIPTION_TEXT_COLOR,
                        );
                    }
                    _ => match self.data_points.len() {
                        0 => draw_text(
                            frame,
                            "NO DATA".to_string(),
                            40.0,
                            center,
                            DESCRIPTION_TEXT_COLOR,
                        ),
                        _data_points_amount => {
                            let weights = extract_weights(self.data_points);

                            //DRAW_BACKGROUND
                            draw_background(frame, self);

                            //LABELS
                            draw_axis_labels(
                                frame,
                                self.graph_state,
                                self.data_points,
                                &self.active_mascot,
                            );

                            //DASHED LINES
                            draw_dashed_lines(self.graph_state, frame);

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

                            //CURSOR
                            if self.graph_state.visible_cursor_information {
                                draw_cursor_information(
                                    weights, //unwrap() in draw_cursor_information can't fail since the list can't be empty
                                    self.graph_state,
                                    bounds,
                                    cursor,
                                    frame,
                                );
                            }
                        }
                    },
                }
            });

        vec![graph_widget]
    }
}
