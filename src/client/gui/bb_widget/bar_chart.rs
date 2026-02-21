use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_theme::{color, text_format};
use crate::client::gui::bb_widget::chart::{CHART_WIDGET_HEIGHT, CHART_WIDGET_WIDTH};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::common::exercise_mod::exercise;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::Element;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::border::Radius;
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::{Quad, Style};
use iced_core::widget::Tree;
use iced_core::{
    Border, Layout, Length, Point, Rectangle, Size, Text, Theme, Widget, renderer, text,
};

const LINE_THICKNESS: f32 = 3.0;
const MOUSE_HIGHLIGHT_LINE_THICKNESS: f32 = 3.0;
const AXIS_FONT_SIZE: f32 = 12.0;
const PERCENTAGE_PLACEHOLDER: f32 = 0.05;
const PERCENTAGE_SPACING_WIDGET_AXIS: f32 = 0.1;
const BASE_SPACING_BETWEEN_COLUMNS: f32 = 150.0;
const FREQUENCY_OF_X_AXIS_LABELS: usize = 6;
const FREQUENCY_OF_Y_AXIS_LABELS: u32 = 10;

pub struct BarChart<'a, Renderer>
where
    Renderer: text::Renderer,
{
    width: f32,
    height: f32,
    active_mascot: Mascot,
    exercise_manager: &'a ExerciseManager,
    font: <Renderer>::Font,
}
impl<'a, Renderer> BarChart<'a, Renderer>
where
    Renderer: text::Renderer<Font = iced::Font>,
{
    pub fn new(active_mascot: Mascot, exercise_manager: &'a ExerciseManager) -> Self {
        BarChart {
            width: CHART_WIDGET_WIDTH,
            height: CHART_WIDGET_HEIGHT,
            active_mascot,
            exercise_manager,
            font: text_format::FIRA_SANS_EXTRABOLD,
        }
    }
    pub fn update_active_mascot(&mut self, mascot: Mascot) {
        self.active_mascot = mascot;
    }
    pub fn get_width(&self) -> f32 {
        self.width
    }
}
impl<Message, Renderer> Widget<Message, Theme, Renderer> for BarChart<'_, Renderer>
where
    Renderer: renderer::Renderer + text::Renderer,
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(&self, _tree: &mut Tree, _renderer: &Renderer, _limits: &Limits) -> Node {
        Node::new(Size::new(self.width, self.height))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let widget_x_axis_padding = self.height * PERCENTAGE_SPACING_WIDGET_AXIS;
        let widget_y_axis_padding = self.width * PERCENTAGE_SPACING_WIDGET_AXIS;
        let x_axis_length = self.width - 2.0 * (widget_y_axis_padding);
        let y_axis_length = self.height - widget_x_axis_padding;

        let left_x_coordinate = layout.bounds().x + widget_y_axis_padding;
        let top_y_coordinate = layout.bounds().y;
        let bottom_y_coordinate = layout.bounds().y + y_axis_length;

        let coordinate_bounds = Rectangle {
            x: left_x_coordinate,
            y: top_y_coordinate,
            width: x_axis_length,
            height: y_axis_length,
        };

        match &self.exercise_manager.data_points.len() {
            0 => renderer.fill_text(
                Text {
                    content: "NO DATA".to_string(),
                    bounds: layout.bounds().size(),
                    size: 40.into(),
                    line_height: Default::default(),
                    font: self.font,
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    shaping: Default::default(),
                    wrapping: Default::default(),
                },
                Point {
                    x: layout.bounds().center_x(),
                    y: layout.bounds().center_y(),
                },
                color::DESCRIPTION_TEXT_COLOR,
                *viewport,
            ),
            amount_of_data_points => {
                renderer.fill_quad(
                    Quad {
                        bounds: coordinate_bounds,
                        border: Border {
                            color: Default::default(),
                            width: 0.0,
                            radius: Radius {
                                top_left: 0.0,
                                top_right: DEFAULT_CONTAINER_RADIUS,
                                bottom_right: 0.0,
                                bottom_left: 0.0,
                            },
                        },
                        shadow: Default::default(),
                    },
                    color::BACKGROUND_COLOR,
                );

                let heaviest_weight =
                    exercise::get_maximum_weight(&self.exercise_manager.data_points).unwrap()
                        as u32; //100% transform to f32 in the future
                let lightest_weight =
                    exercise::get_minimum_weight(&self.exercise_manager.data_points).unwrap()
                        as u32; // 0% transform to f32 in the future
                let range = heaviest_weight - lightest_weight;

                let column_spacing = BASE_SPACING_BETWEEN_COLUMNS / *amount_of_data_points as f32;
                let width_of_graph_canvas: f32 = x_axis_length - 2.0 * column_spacing;
                let height_of_graph_canvas: f32 =
                    y_axis_length - 2.0 * PERCENTAGE_PLACEHOLDER * y_axis_length;

                let graph_axis_padding_x: f32 = (x_axis_length - width_of_graph_canvas) / 2.0;
                let graph_axis_padding_y: f32 = (y_axis_length - height_of_graph_canvas) / 2.0;

                let width_of_columns: f32 = (width_of_graph_canvas
                    - (amount_of_data_points - 1) as f32 * column_spacing)
                    / *amount_of_data_points as f32;

                let modulo_number = (*amount_of_data_points / FREQUENCY_OF_X_AXIS_LABELS).max(1);

                for (i, (date, kg)) in self.exercise_manager.data_points.iter().enumerate() {
                    let integer_kg = *kg as u32;
                    let share = if range == 0 {
                        0.0
                    } else {
                        (integer_kg - lightest_weight) as f32 / range as f32
                    };

                    let x_of_column = left_x_coordinate
                        + graph_axis_padding_x
                        + i as f32 * (width_of_columns + column_spacing);
                    let y_of_column =
                        bottom_y_coordinate - graph_axis_padding_y - share * height_of_graph_canvas;

                    renderer.fill_quad(
                        Quad {
                            bounds: Rectangle {
                                x: x_of_column,
                                y: y_of_column,
                                width: width_of_columns,
                                height: graph_axis_padding_y + share * height_of_graph_canvas,
                            },
                            border: Border {
                                color: Default::default(),
                                width: Default::default(),
                                radius: Radius {
                                    top_left: DEFAULT_CONTAINER_RADIUS,
                                    top_right: DEFAULT_CONTAINER_RADIUS,
                                    bottom_left: 0.0,
                                    bottom_right: 0.0,
                                },
                            },
                            shadow: Default::default(),
                        },
                        self.active_mascot.get_primary_color(),
                    );

                    if i % modulo_number == 0 {
                        let date_string = date.format("%d.%m.%y").to_string();
                        //DATE
                        renderer.fill_text(
                            Text {
                                content: date_string,
                                bounds: layout.bounds().size(),
                                size: AXIS_FONT_SIZE.into(),
                                line_height: Default::default(),
                                font: self.font,
                                horizontal_alignment: Horizontal::Left,
                                vertical_alignment: Vertical::Top,
                                shaping: Default::default(),
                                wrapping: Default::default(),
                            },
                            Point {
                                x: x_of_column,
                                y: bottom_y_coordinate + INDENT,
                            },
                            color::TEXT_COLOR,
                            *viewport,
                        );
                    }
                }
                let milestones = exercise::get_weight_milestones(
                    lightest_weight,
                    heaviest_weight,
                    FREQUENCY_OF_Y_AXIS_LABELS,
                );
                for val in milestones {
                    //WEIGHT
                    let share = if range == 0 {
                        0.0
                    } else {
                        (val - lightest_weight) as f32 / range as f32
                    };

                    let weight_string = format!("{} kg", val);
                    let weight_bounds = Size::new(widget_y_axis_padding - INDENT, AXIS_FONT_SIZE);
                    renderer.fill_text(
                        Text {
                            content: weight_string,
                            bounds: weight_bounds,
                            size: AXIS_FONT_SIZE.into(),
                            line_height: Default::default(),
                            font: self.font,
                            horizontal_alignment: Horizontal::Right,
                            vertical_alignment: Vertical::Center,
                            shaping: Default::default(),
                            wrapping: Default::default(),
                        },
                        Point {
                            x: layout.bounds().x + widget_y_axis_padding - INDENT,
                            y: bottom_y_coordinate
                                - graph_axis_padding_y
                                - (share) * height_of_graph_canvas,
                        },
                        color::TEXT_COLOR,
                        *viewport,
                    );
                }

                //Y_AXIS
                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: left_x_coordinate,
                            y: top_y_coordinate,
                            width: LINE_THICKNESS,
                            height: y_axis_length,
                        },
                        ..Default::default()
                    },
                    self.active_mascot.get_secondary_color(),
                );
                //X-AXIS
                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: left_x_coordinate,
                            y: bottom_y_coordinate,
                            width: x_axis_length,
                            height: LINE_THICKNESS,
                        },
                        ..Default::default()
                    },
                    self.active_mascot.get_secondary_color(),
                );

                //MOUSE_INTERACTION

                let cursor_position = match cursor.position() {
                    None => return,
                    Some(pos) => pos,
                };
                if !coordinate_bounds.contains(cursor_position) {
                    return;
                }
                let see_through_mouse_follower_color = {
                    let mut base = color::DESCRIPTION_TEXT_COLOR;
                    base.a = 0.5;
                    base
                };
                //HORIZONTAL_LINE
                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: left_x_coordinate,
                            y: cursor_position.y,
                            width: cursor_position.x - left_x_coordinate,
                            height: MOUSE_HIGHLIGHT_LINE_THICKNESS,
                        },
                        border: Default::default(),
                        shadow: Default::default(),
                    },
                    see_through_mouse_follower_color,
                );
                //VERTICAL_LINE
                renderer.fill_quad(
                    Quad {
                        bounds: Rectangle {
                            x: cursor_position.x,
                            y: cursor_position.y,
                            width: MOUSE_HIGHLIGHT_LINE_THICKNESS,
                            height: bottom_y_coordinate - cursor_position.y,
                        },
                        border: Border {
                            radius: Radius {
                                top_right: 15.0,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        shadow: Default::default(),
                    },
                    see_through_mouse_follower_color,
                );
            }
        }
    }
}
impl<'a, Message: 'a, Renderer> From<BarChart<'a, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: 'a + renderer::Renderer + text::Renderer,
{
    fn from(value: BarChart<'a, Renderer>) -> Self {
        Self::new(value)
    }
}
