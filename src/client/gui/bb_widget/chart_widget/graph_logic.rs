use crate::client::gui::bb_widget::chart_widget::chart::{CHART_WIDGET_HEIGHT, CHART_WIDGET_WIDTH};
use crate::client::gui::bb_widget::chart_widget::graph::GraphWidgetState;
use crate::common::exercise_mod::exercise::ExerciseDataPoints;
use crate::common::exercise_mod::weight::Kg;
use chrono::NaiveDate;
use iced_core::Point;

pub fn chop_weights(graph_widget_state: &GraphWidgetState, y_values: Vec<Kg>) -> Vec<Kg> {
    let chop_start_index: isize =
        y_values.len() as isize - graph_widget_state.points_to_draw as isize;

    let safe_chop_start_index = match chop_start_index {
        ..=0 => 0,
        _ => chop_start_index,
    };

    y_values[(safe_chop_start_index) as usize..].to_vec()
}

pub fn chop_dates(
    graph_widget_state: &GraphWidgetState,
    x_values: Vec<NaiveDate>,
) -> Vec<NaiveDate> {
    let chop_start_index: isize =
        x_values.len() as isize - graph_widget_state.points_to_draw as isize;

    let safe_chop_start_index = match chop_start_index {
        ..=0 => 0,
        _ => chop_start_index,
    };

    x_values[(safe_chop_start_index) as usize..].to_vec()
}

pub fn calculate_points(graph_widget_state: &GraphWidgetState, y_values: Vec<Kg>) -> Vec<Point> {
    let chopped_y_values = &chop_weights(graph_widget_state, y_values.clone());

    //PADDING LEFT AND RIGHT: LEFT FOR Y_LABELS, RIGHT FOR FREE SPACE
    let block_width = (CHART_WIDGET_WIDTH
        - crate::client::gui::bb_widget::chart_widget::graph::GRAPH_PADDING * 2.0)
        / graph_widget_state.points_to_draw as f32; //division with 0 is not possible since limit is 1
    //PADDING UP AND DOWN:  UP FOR Y-AXIS-ARROW SPACE,DOWN FOR X-LABELS
    let block_height = (CHART_WIDGET_HEIGHT
        - crate::client::gui::bb_widget::chart_widget::graph::GRAPH_PADDING * 2.0)
        / crate::client::gui::bb_widget::chart_widget::graph::FREQUENCY_OF_Y_AXIS_LABELS as f32;
    let mut current_x =
        CHART_WIDGET_WIDTH - crate::client::gui::bb_widget::chart_widget::graph::GRAPH_PADDING;

    //I can unwrap() since the function is not going to get called if exercises.len() = 0
    let min_y: Kg = get_f32_min(chopped_y_values);
    let max_y: Kg = get_f32_max(chopped_y_values);

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
    let height_graph: f32 = CHART_WIDGET_HEIGHT
        - crate::client::gui::bb_widget::chart_widget::graph::GRAPH_PADDING * 2.0
        - height_padding_for_arrow
        - x_axis_padding;
    let lowest_point_graph: f32 = CHART_WIDGET_HEIGHT
        - crate::client::gui::bb_widget::chart_widget::graph::GRAPH_PADDING
        - x_axis_padding;

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

pub fn extract_weights(exercise_data_points: &ExerciseDataPoints) -> Vec<Kg> {
    let mut weights: Vec<Kg> = vec![];
    for (_date, weight) in exercise_data_points {
        weights.push(*weight)
    }
    weights
}

pub fn extract_dates(exercise_data_points: &ExerciseDataPoints) -> Vec<NaiveDate> {
    let mut dates: Vec<NaiveDate> = vec![];
    for (date, _weight) in exercise_data_points {
        dates.push(*date)
    }

    dates
}

pub fn get_f32_min(vec: &[Kg]) -> f32 {
    vec.iter()
        .map(|value| (*value * 10.0) as usize)
        .min()
        .unwrap_or(0) as f32
        / 10.0
}

pub fn get_f32_max(vec: &[Kg]) -> f32 {
    vec.iter()
        .map(|value| (*value * 10.0) as usize)
        .max()
        .unwrap_or(0) as f32
        / 10.0
}
