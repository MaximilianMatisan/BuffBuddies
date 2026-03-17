use std::time::Duration;
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
            response: Duration::from_millis(5000),
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
}