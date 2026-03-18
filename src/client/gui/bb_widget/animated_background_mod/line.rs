use std::time::Duration;
use iced::widget::canvas::Path;
use iced_anim::{Animated, Motion};
use iced_core::Point;

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
            response: Duration::from_millis(10000),
            damping: Motion::BOUNCY.damping(),
        };

        Self {
            start,
            control,
            end,
            animation_progress: Animated::new(0.0, animation_motion),
            has_spawned_line: false,
        }
    }
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
        Path::new( |builder| {
          builder.move_to(self.start);
            builder.bezier_curve_to(self.control, self.control, self.end)
        })
    }
}