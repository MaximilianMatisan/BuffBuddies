use iced::color;
use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Color, Gradient, Point, Radians};

pub const BACKGROUND_COLOR: iced::Color = color!(36, 43, 51);
pub const CONTAINER_COLOR: iced::Color = color!(57, 63, 68);
pub const LIGHTER_CONTAINER_COLOR: iced::Color = color!(70, 76, 80);
pub const HIGHLIGHTED_CONTAINER_COLOR: iced::Color = color!(102, 102, 102);
//pub const BORDER_CONTAINER_COLOR: iced::Color = color!(145, 142, 141);
pub const TEXT_COLOR: iced::Color = color!(255, 255, 255);
pub const ERROR_COLOR: iced::Color = color!(220, 54, 46);
pub const DESCRIPTION_TEXT_COLOR: iced::Color = color!(142, 142, 147);
pub const DARK_SHADOW: iced::Color = color!(0, 0, 0);
pub const DASHED_LINES_COLOR: iced::Color = color!(120, 120, 122);

/// Converts a list of `(Color, offset)` tuples into a `Vec<ColorStop>`.
///
/// Each tuple represents a gradient stop.
pub fn create_color_stops(stops: Vec<(iced::Color, f32)>) -> Vec<ColorStop> {
    stops
        .into_iter()
        .map(|(color, offset)| ColorStop { offset, color })
        .collect()
}

/// Creates a vector of `(Color, offset)` tuples containing the same `color`
/// repeated `amount` times.
///
/// The offsets are distributed linearly between `0.0` and `1.0`. This helper
/// function is primarily intended to be used together with `create_color_stops`
/// to easily construct a `Vec<ColorStop>` consisting of a single repeated color.
///
/// # Example
/// ```
/// let stops = create_color_stops(create_one_colored_stops(color, 5));
/// ```
pub fn create_one_colored_stops(color: Color, amount: usize) -> Vec<(Color, f32)> {
    let mut one_colored_stops = Vec::new();

    for index in 0..amount {
        one_colored_stops.push((color, index as f32 / amount as f32));
    }

    one_colored_stops
}

/// Creates a gradient `Background` that transitions between the given
/// `color_stops` along the specified `angle`.
pub fn create_new_gradient_background(
    angle: impl Into<Radians>,
    color_stops: Vec<ColorStop>,
) -> Background {
    Background::Gradient(Gradient::Linear(Linear::new(angle).add_stops(color_stops)))
}

/// Creates a 2D linear gradient that can be used as a fill for widget backgrounds.
pub fn create_2d_gradient(
    start: Point,
    end: Point,
    color_stops: Vec<ColorStop>,
) -> iced::advanced::graphics::Gradient {
    iced::advanced::graphics::Gradient::Linear(iced::advanced::graphics::gradient::Linear::new(start,end).add_stops(color_stops))
}

pub fn create_solid_stroke_style(color: Color) -> iced::widget::canvas::stroke::Style {
    iced::widget::canvas::stroke::Style::Solid(color)
}

pub fn create_gradient_stroke_style(gradient: iced::advanced::graphics::Gradient) -> iced::widget::canvas::stroke::Style {
    iced::widget::canvas::stroke::Style::Gradient(gradient)
}
