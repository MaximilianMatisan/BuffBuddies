use chrono::NaiveDate;
use crate::client::gui::bb_theme::color;
use crate::client::gui::mascots::Mascot;
use iced::{Element};
use iced_core::layout::{Limits, Node};
use iced_core::mouse::Cursor;
use iced_core::renderer::{Quad, Style};
use iced_core::widget::Tree;
use iced_core::{renderer, text, Border, Layout, Length, Point, Rectangle, Size, Text, Theme, Widget};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::border::Radius;
use crate::client::backend::exercise::weight::Kg;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_widget::widget_utils::INDENT;

const PROGRESS_WIDGET_WIDTH: f32 = 800.0;
const PROGRESS_WIDGET_HEIGHT: f32 = 600.0;
const LINE_WIDTH: f32 = 3.0;
const AXIS_FONT_SIZE: f32 = 12.0;
const PERCENTAGE_PLACEHOLDER: f32 = 0.05;
const PERCENTAGE_SPACING_WIDGET_AXIS: f32 = 0.1;
const BASE_SPACING_BETWEEN_COLUMNS: f32 = 150.0;
const FREQUENCY_OF_AXIS_LABELS: usize = 6;

pub struct ProgressWidget<Renderer>
    where Renderer: text::Renderer
{
    width: f32,
    height: f32,
    active_mascot: Mascot,
    data_points: Vec<(NaiveDate, Kg)>,
    font: <Renderer>::Font
}
impl<Renderer> ProgressWidget<Renderer>
    where Renderer: text::Renderer<Font = iced::Font>
{
    pub fn new(active_mascot: Mascot, data_points: Vec<(NaiveDate, Kg)>) -> Self {
        ProgressWidget {
            width: PROGRESS_WIDGET_WIDTH,
            height: PROGRESS_WIDGET_HEIGHT,
            active_mascot,
            data_points,
            font: bb_theme::text_format::FIRA_SANS_EXTRABOLD,
        }
    }
    pub fn update_data_points(&mut self, data_points: Vec<(NaiveDate, Kg)>) {
        self.data_points = data_points;
    }
    pub fn update_active_mascot(&mut self, mascot: Mascot) {
        self.active_mascot = mascot;
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for ProgressWidget<Renderer>
where
    Renderer: renderer::Renderer + text::Renderer,
    Message: Clone
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.height))
    }

    fn layout(&self,
              _tree: &mut Tree,
              _renderer: &Renderer,
              _limits: &Limits) -> Node
    {
        Node::new(Size::new(self.width, self.height))
    }

    fn draw(&self,
            _tree: &Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &Style,
            layout: Layout<'_>,
            _cursor: Cursor,
            viewport: &Rectangle)
    {
       renderer.fill_quad(Quad { //TODO TEMP
           bounds: layout.bounds(),
           border: Border {
               color: Default::default(),
               width: 0.0,
               radius: DEFAULT_CONTAINER_RADIUS.into()
           },
           shadow: Default::default(),
       }, color::CONTAINER_COLOR);

        let widget_x_axis_padding = self.height * PERCENTAGE_SPACING_WIDGET_AXIS;
        let widget_y_axis_padding = self.width * PERCENTAGE_SPACING_WIDGET_AXIS;
        let x_axis_length = self.width - 2.0 * (widget_y_axis_padding);
        let y_axis_length = self.height -2.0 * (widget_x_axis_padding);

        let left_x_coordinate = layout.bounds().x + widget_y_axis_padding;
        let top_y_coordinate = layout.bounds().y + widget_x_axis_padding;
        let bottom_y_coordinate = layout.bounds().y + widget_x_axis_padding + y_axis_length;

        //Y_AXIS
        renderer.fill_quad(Quad {
            bounds: Rectangle {
                x: left_x_coordinate,
                y: top_y_coordinate,
                width: LINE_WIDTH,
                height: y_axis_length
            },
            border: Default::default(),
            shadow: Default::default(),
        }, self.active_mascot.get_secondary_color());
        //X-AXIS
        renderer.fill_quad(Quad {
            bounds: Rectangle {
                x: left_x_coordinate,
                y: bottom_y_coordinate,
                width: x_axis_length,
                height: LINE_WIDTH,
            },
            border: Default::default(),
            shadow: Default::default(),
        }, self.active_mascot.get_secondary_color());

        match &self.data_points.len() {
            0  => {
                renderer.fill_text(Text{
                    content: "NO DATA".to_string(),
                    bounds: layout.bounds().size(),
                    size: 40.into(),
                    line_height: Default::default(),
                    font: self.font,
                    horizontal_alignment: Horizontal::Center,
                    vertical_alignment: Vertical::Center,
                    shaping: Default::default(),
                    wrapping: Default::default(),
                }, Point {
                    x: layout.bounds().center_x(),
                    y: layout.bounds().center_y(),
                }, color::DESCRIPTION_TEXT_COLOR, *viewport)
            },
            amount_of_data_points => {
                let kg_iterator = self.data_points.iter().map(|(_,kg)| *kg as u32);
                let heaviest_weight = kg_iterator.clone().max().unwrap(); //100%
                let lightest_weight = kg_iterator.min().unwrap();         // 0%
                let range = heaviest_weight - lightest_weight;

                let column_spacing = BASE_SPACING_BETWEEN_COLUMNS / *amount_of_data_points as f32 ;
                let width_of_graph_canvas: f32 = x_axis_length - 2.0 * column_spacing;
                let height_of_graph_canvas: f32 = y_axis_length - 2.0 * PERCENTAGE_PLACEHOLDER * y_axis_length;

                let graph_axis_padding_x: f32 = (x_axis_length - width_of_graph_canvas) / 2.0;
                let graph_axis_padding_y: f32 = (y_axis_length - height_of_graph_canvas) / 2.0;

                let width_of_columns: f32 =
                    (width_of_graph_canvas - (amount_of_data_points - 1) as f32 * column_spacing)
                    / *amount_of_data_points as f32;

                let modulo_number = *amount_of_data_points / FREQUENCY_OF_AXIS_LABELS;

                for (i, (date, kg)) in self.data_points.iter().enumerate() {
                    let integer_kg = *kg as u32;
                    let share = if range == 0 {0.0}
                        else {(integer_kg - lightest_weight) as f32 / range as f32};

                    let x_of_column =
                        left_x_coordinate + graph_axis_padding_x + i as f32 * (width_of_columns + column_spacing);
                    let y_of_column =
                        bottom_y_coordinate - graph_axis_padding_y - share * height_of_graph_canvas;

                    renderer.fill_quad(Quad{
                        bounds: Rectangle {
                            x: x_of_column,
                            y: y_of_column,
                            width: width_of_columns,
                            height: graph_axis_padding_y + share * height_of_graph_canvas
                        },
                        border: Border {
                            color: Default::default(),
                            width: Default::default(),
                            radius: Radius {
                                top_left: DEFAULT_CONTAINER_RADIUS,
                                top_right: DEFAULT_CONTAINER_RADIUS,
                                bottom_left: 0.0,
                                bottom_right: 0.0
                            }
                        },
                        shadow: Default::default()
                    }, self.active_mascot.get_primary_color());

                    if i % modulo_number == 0 {
                        let date_string = date.format("%d.%m.%y").to_string();
                        //DATUM
                        renderer.fill_text(Text{
                            content: date_string,
                            bounds: layout.bounds().size(),
                            size: AXIS_FONT_SIZE.into(),
                            line_height: Default::default(),
                            font: self.font,
                            horizontal_alignment: Horizontal::Left,
                            vertical_alignment: Vertical::Top,
                            shaping: Default::default(),
                            wrapping: Default::default(),
                        }, Point {
                            x: x_of_column,
                            y: bottom_y_coordinate + INDENT
                        }, color::DESCRIPTION_TEXT_COLOR, *viewport);

                        println!("{:?}",self.data_points[i]);
                        let weight_string = format!("{} kg", integer_kg);
                        let weight_bounds = Size::new(widget_y_axis_padding-INDENT, AXIS_FONT_SIZE);
                        renderer.fill_text( Text {
                            content: weight_string.clone(),
                            bounds: weight_bounds,
                            size: AXIS_FONT_SIZE.into(),
                            line_height: Default::default(),
                            font: self.font,
                            horizontal_alignment: Horizontal::Right,
                            vertical_alignment: Vertical::Top,
                            shaping: Default::default(),
                            wrapping: Default::default(),
                        }, Point {
                            x: layout.bounds().x + widget_y_axis_padding - INDENT,
                            y: y_of_column,
                        }, color::DESCRIPTION_TEXT_COLOR, *viewport);
                    }
                }
            }
        }
    }
}
impl<'a, Message: 'a, Renderer> From<ProgressWidget<Renderer>> for Element<'a, Message, Theme, Renderer>
where Message: Clone,
      Renderer: 'a + renderer::Renderer + text::Renderer
{
    fn from(value: ProgressWidget<Renderer>) -> Self {
        Self::new(value)
    }
}
