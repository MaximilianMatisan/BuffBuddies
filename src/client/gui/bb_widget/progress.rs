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

const PROGRESS_WIDGET_WIDTH: f32 = 500.0;
const PROGRESS_WIDGET_HEIGHT: f32 = 500.0;
const LINE_WIDTH: f32 = 2.0;
const PERCENTAGE_PLACEHOLDER: f32 = 0.05;
const BASE_SPACING_BETWEEN_COLUMNS: f32 = 150.0;

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
        //Y_AXIS
        renderer.fill_quad(Quad {
            bounds: Rectangle {
                x: layout.bounds().x,
                y: layout.bounds().y,
                width: LINE_WIDTH,
                height: self.height
            },
            border: Default::default(),
            shadow: Default::default(),
        }, self.active_mascot.get_secondary_color());
        //X-AXIS
        renderer.fill_quad(Quad {
            bounds: Rectangle {
                x: layout.bounds().x,
                y: layout.bounds().y + self.height,
                width: self.width,
                height: LINE_WIDTH,
            },
            border: Default::default(),
            shadow: Default::default(),
        }, self.active_mascot.get_secondary_color());

        println!("{:?}", self.data_points); //TODO DELETE IN THE END
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
                let lightest_weight = kg_iterator.clone().min().unwrap(); // 0%
                let range = heaviest_weight - lightest_weight;

                let column_spacing = BASE_SPACING_BETWEEN_COLUMNS / *amount_of_data_points as f32 ;
                let width_of_graph_canvas: f32 = self.width - 2.0 * column_spacing;
                let height_of_graph_canvas: f32 = self.height - 2.0 * PERCENTAGE_PLACEHOLDER * self.height;

                let padding_x: f32 = (self.width - width_of_graph_canvas) / 2.0;
                let padding_y: f32 = (self.height - height_of_graph_canvas) / 2.0;

                let width_of_columns: f32 = 
                    (width_of_graph_canvas - (amount_of_data_points - 1) as f32 * column_spacing) 
                    / *amount_of_data_points as f32;
                
                for (i, (date, kg)) in self.data_points.iter().enumerate() {
                    let share = if range == 0 {0.0} 
                        else {(*kg as u32 - lightest_weight) as f32 / range as f32};

                    renderer.fill_quad(Quad{
                        bounds: Rectangle {
                            x: layout.bounds().x + padding_x + i as f32 * (width_of_columns + column_spacing),
                            y: layout.bounds().y + padding_y + (1.0-share)* height_of_graph_canvas,
                            width: width_of_columns,
                            height: self.height - (1.0-share) * height_of_graph_canvas - padding_y 
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
                    }, self.active_mascot.get_primary_color())
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
