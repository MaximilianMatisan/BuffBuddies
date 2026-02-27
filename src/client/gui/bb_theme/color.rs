use iced::color;
use iced_core::gradient::{ColorStop, Linear};
use iced_core::{Background, Color, Gradient, Radians};

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

pub fn create_color_stops(stops: Vec<(iced::Color, f32)>) -> Vec<ColorStop> {
    stops
        .into_iter()
        .map(|(color, offset)| ColorStop { offset, color })
        .collect()
}

pub fn create_one_colored_stops(color: Color, amount: usize) -> Vec<(Color, f32)> {
    let mut one_colored_stops = Vec::new();

    for index in 0..amount {
        one_colored_stops.push((color, index as f32 / amount as f32));
    }

    one_colored_stops
}

pub fn create_new_gradient_background(
    angle: impl Into<Radians>,
    color_stops: Vec<ColorStop>,
) -> Background {
    Background::Gradient(Gradient::Linear(Linear::new(angle).add_stops(color_stops)))
}
