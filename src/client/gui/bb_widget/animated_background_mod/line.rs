use iced::widget::canvas::Path;
use iced_anim::{Animated, Motion};
use iced_core::Point;
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
            response: Duration::from_millis(40000),
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
