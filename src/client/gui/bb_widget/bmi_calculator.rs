use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Frame, Geometry, Path, event};
use iced::{Element, Rectangle, Renderer, Size, Theme};
use iced_anim::{Animated, Animation, Event, Motion};
use iced_core::{color, Color, Point};
use std::time::Duration;

use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::CONTAINER_COLOR;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_widget::canvas_utils::{create_arc_path, draw_text, generate_stroke};
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::epic_mascot::EpicMascot;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::mascot_mod::rare_mascot::RareMascot;
use crate::common::user_mod::user::UserInformation;

const BMI_WIDGET_WIDTH: f32 = 250.0;
const BMI_WIDGET_HEIGHT: f32 = 250.0;
const BMI_CIRCLE_RADIUS: f32 = 90.0;
const AMOUNT_ARCS: usize = 5;
const BMI_PADDING_BETWEEN_INNER_ARCS: f32 = 17.0;
const BMI_PADDING_BETWEEN_LAST_ARCS: f32 = 44.0;
const BMI_FONT_SIZE_RESULT: f32 = 36.0;
const BMI_FONT_SIZE_DESCRIPTION: f32 = 26.0;
// Arc angles are defined as clockwise rotations starting from the positive X-axis.
// For our use case, it is more intuitive to measure angles clockwise from the negative Y-axis
// This offset converts between the two coordinate systems.
const BMI_DEGREE_START_TRANSLATION: f32 = 90.0;

pub struct BMIWidget<'a> {
    active_mascot: Mascot,
    bmi_widget_state: &'a BMIWidgetState,
    bmi_value: f32
}

#[derive(Clone, Debug)]
pub enum BMIMessage {
    UpdateBMIAnimation(Event<f32>),
}

impl<'a> BMIWidget<'a> {
    pub(crate) fn new(app: &'a App) -> Self {
        BMIWidget {
            active_mascot: app.mascot_manager.selected_mascot,
            bmi_widget_state: &app.widget_manager.bmi_widget_state,
            bmi_value: calculate_bmi(&app.user_manager.user_info),
        }
    }
     fn colors() -> [Color; AMOUNT_ARCS] {
        [
            RareMascot::Whale.get_primary_color(),
            color!(0,192,232),
            RareMascot::Chameleon.get_primary_color(),
            RareMascot::Duck.get_primary_color(),
            EpicMascot::Reindeer.get_primary_color(),
        ]
    }
    pub(crate) fn view(self) -> Element<'a, Message> {
        let draw_percentage = &self.bmi_widget_state.animation_progress;

        let canvas = canvas(self)
            .width(BMI_WIDGET_WIDTH)
            .height(BMI_WIDGET_HEIGHT);

        Animation::new(draw_percentage, canvas)
            .on_update(|event| Message::BMI(BMIMessage::UpdateBMIAnimation(event)))
            .into()
    }
}

pub struct BMIWidgetState {
    circle: Cache,
    pub animation_progress: Animated<f32>,
}

impl BMIWidgetState {
    pub(crate) fn update_circle(&self) {
        self.circle.clear();
    }
}

impl BMIWidgetState {
    pub fn new() -> Self {
        let animation_motion = Motion {
            response: Duration::from_millis(3000),
            damping: Motion::SMOOTH.damping(),
        };

        Self {
            circle: Cache::default(),
            animation_progress: Animated::new(0.0, animation_motion),
        }
    }
}

impl Default for BMIWidgetState {
    fn default() -> Self {
        Self::new()
    }
}

impl canvas::Program<Message> for BMIWidget<'_> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: iced::widget::canvas::Event,
        _bounds: Rectangle,
        _cursor: iced_core::mouse::Cursor,
    ) -> (iced::event::Status, std::option::Option<Message>) {
        self.bmi_widget_state.update_circle();

        (
            event::Status::Ignored,
            Some(crate::client::gui::user_interface::Message::BMI(
                BMIMessage::UpdateBMIAnimation(iced_anim::Event::Target(1.0)),
            )),
        )
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let circle_widget =
            self.bmi_widget_state
                .circle
                .draw(renderer, bounds.size(), |frame| {
                    let circle_center = frame.center();

                    //DRAW BACKGORUND
                    draw_background(frame);

                    //DRAW BMI ARCS
                    draw_bmi_arcs(frame, circle_center, self.bmi_widget_state);

                    //FILL BMI ARCS
                    //fill_bmi_arcs(frame, circle_center, self);

                    //DRAW TEXT BMI VALUE
                    draw_bmi_text(frame, self)
                });

        vec![circle_widget]
    }
}
fn draw_background(frame: &mut Frame) {
    let background_size = Path::rounded_rectangle(
        Point::ORIGIN, //START
        Size {
            width: frame.width(),
            height: frame.height(),
        },
        DEFAULT_CONTAINER_RADIUS.into(),
    );

    frame.fill(&background_size, CONTAINER_COLOR);
}

fn draw_bmi_arcs(
    frame: &mut Frame,
    center_of_circle: Point,
    bmi_widget_state: &BMIWidgetState
) {

    let not_filled_colors: Vec<Color> = BMIWidget::colors().iter()
        .map(|color|{
            Color{
                a: 0.25,
                    ..*color
            }
        })
        .collect();

    //Vector with Arc colors,start degrees, end degrees
    let colors_with_angles: Vec<(Color,f32,f32)> =
        not_filled_colors.iter().enumerate().map(|(index,color)|{

            let start_angle = BMI_DEGREE_START_TRANSLATION + calculate_degree_offset(index,DegreeOffsetType::Start);
            let end_angle = BMI_DEGREE_START_TRANSLATION + calculate_degree_offset(index,DegreeOffsetType::End);

            (*color,start_angle,end_angle)

        })
            .collect();
    for (color, start_angle, end_angle) in colors_with_angles {

        let arc_path = &create_arc_path(center_of_circle, BMI_CIRCLE_RADIUS, start_angle, end_angle);

        frame.stroke(
            arc_path,
            generate_stroke(20.0, color),
        );
    }
}

fn fill_bmi_arcs(
    frame: &mut Frame,
    center_of_circle: Point,
    bmi_widget: &BMIWidget
) {
    let start_angle = BMI_DEGREE_START_TRANSLATION + calculate_degree_offset(0, DegreeOffsetType::Start);

    let end_angle = BMI_DEGREE_START_TRANSLATION + calculate_degree_offset(0, DegreeOffsetType::End);

    let arc_path = &create_arc_path(center_of_circle, BMI_CIRCLE_RADIUS, start_angle, end_angle);

    frame.stroke(
        arc_path,
        generate_stroke(20.0, color!(255,255,255)),
    );
}

fn draw_bmi_text(frame: &mut Frame, bmi_widget: &BMIWidget) {
    let circle_center = frame.center();
    let text_padding = BMI_FONT_SIZE_RESULT + 2.0;

    //BMI VALUE
    draw_text(
        frame,
        bmi_widget.bmi_value.to_string(),
        BMI_FONT_SIZE_RESULT,
        Point {
            x: circle_center.x,
            y: circle_center.y - text_padding / 2.0,
        },
    );

    //WEIGHT CLASS
    let weight_class = translate_bmi_to_class(bmi_widget.bmi_value);

    draw_text(
        frame,
        weight_class,
        BMI_FONT_SIZE_DESCRIPTION,
        Point {
            x: circle_center.x,
            y: circle_center.y + text_padding / 2.0,
        },
    );
}

//LOGIC

fn calculate_bmi(user_information: &UserInformation) -> f32 {
    //BMI = weight(kg) / height (m) * height (m)
    let weight = user_information.weight;
    let height = user_information.height as f32 / 100.0;

    let bmi_value = weight / (height * height);

    let formatted_bmi_value = (bmi_value * 10.0).ceil() / 10.0;
    formatted_bmi_value

}

fn translate_bmi_to_class (bmi_value: f32) -> String {

    let weight_class = match bmi_value {
        ..16.0 => "Severely underweight",
        16.0 ..18.5 => "Underweight",
        18.5 ..25.0 => "Normal",
        25.0 ..30.0 => "Overweight",
        _ => "Severely overweight",
    };
    weight_class.to_string()
}

enum DegreeOffsetType {
    Start,
    End
}
fn calculate_degree_offset(arc_number: usize, degree_offset_type: DegreeOffsetType) -> f32 {
    //The circle should have a bigger gap between the lowest to gaps
    //To keep it symmetrical -> division by 2
    let start_offset = BMI_PADDING_BETWEEN_LAST_ARCS / 2.0;

    //The circle also has gaps between the arcs which means I have to subtract them from the total degrees to know how many degrees are available for the arc
    //Multiply by 4 because there are 4 gaps between the 5 arcs,excluding the gap between the last and first arc
    let sum_total_degrees_arcs = 360.0 - BMI_PADDING_BETWEEN_LAST_ARCS - BMI_PADDING_BETWEEN_INNER_ARCS * 4.0;

    //Available degrees pro arc
    let degrees_pro_arc = sum_total_degrees_arcs / AMOUNT_ARCS as f32;

    //FORMULA: start-offset + the amount degrees that have already been drawn (all the arcs + all the paddings between the arcs)
    let drawn_arcs = match degree_offset_type {
        DegreeOffsetType::Start => arc_number,
        DegreeOffsetType::End => arc_number + 1
    };

    start_offset + drawn_arcs as f32 * degrees_pro_arc + BMI_PADDING_BETWEEN_INNER_ARCS * arc_number as f32
}