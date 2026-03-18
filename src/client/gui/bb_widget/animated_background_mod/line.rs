use crate::client::gui::bb_widget::animated_background_mod::animated_background::BASE_LINE_WIDTH;
use iced::widget::canvas::Path;
use iced_anim::{Animated, Motion};
use iced_core::{Point, Size};
use rand::RngExt;
use std::time::Duration;

pub struct AnimatedLine {
    pub start: Point,
    pub control: Point,
    pub end: Point,
    pub animation_progress: Animated<f32>,
    pub has_spawned_line: bool,
}
impl AnimatedLine {
    pub fn new(start: Point, control: Point, end: Point) -> Self {
        let animation_motion = Motion {
            response: Duration::from_secs(40),
            damping: Motion::SMOOTH.damping(),
        };

        Self {
            start,
            control,
            end,
            animation_progress: Animated::new(0.0, animation_motion),
            has_spawned_line: false,
        }
    }
    #[allow(dead_code)]
    pub fn straight_line(&self) -> Path {
        let start = self.start;
        let end = self.end;
        let progress = self.animation_progress.value();

        let current = Point::new(
            start.x + (end.x - start.x) * progress,
            start.y + (end.y - start.y) * progress,
        );
        Path::new(|builder| {
            builder.move_to(start);
            builder.line_to(current);
        })
    }
    pub fn bezier_curve(&self) -> Path {
        let progress = self.animation_progress.value();
        let steps = 500;

        Path::new(|builder| {
            builder.move_to(self.start);

            for step in 1..=steps {
                let line_progress = (step as f32 / steps as f32) * progress;
                let line_to = bezier(self.start, self.control, self.end, line_progress);
                builder.line_to(line_to)
            }
        })
    }
}

fn bezier(start: Point, control: Point, end: Point, line_progress: f32) -> Point {
    let progress_left = 1.0 - line_progress;

    Point {
        x: progress_left * progress_left * start.x
            + 2.0 * progress_left * line_progress * control.x
            + line_progress * line_progress * end.x,
        y: progress_left * progress_left * start.y
            + 2.0 * progress_left * line_progress * control.y
            + line_progress * line_progress * end.y,
    }
}
/// Returns a random start and end points on an edge of the frame
pub fn get_random_start_and_end_point_of_line(size: Size) -> (Point, Point) {
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
