use iced::widget::canvas::{Cache, Frame, Geometry, Path, event};
use iced::widget::{Column, Row, Space, canvas, container, text};
use iced::{Element, Rectangle, Renderer, Theme};
use iced::{Task, mouse};
use iced_anim::{Animated, Animation, Event, Motion};
use iced_core::{Color, Length, Point};
use std::time::Duration;

use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::ButtonStyle::{Active, InactiveTab};
use crate::client::gui::bb_theme::custom_button::{DEFAULT_BUTTON_RADIUS, create_text_button};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::canvas_utils::generate_stroke;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use crate::client::gui::user_interface::Message::ProgressBar;
use crate::common::mascot_mod::epic_mascot::EpicMascot;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::mascot_mod::rare_mascot::RareMascot;
use iced_core::alignment::Horizontal;
use iced_core::border::Radius;
use strum_macros::Display;

//LENGTH OF THE BAR ITSELF
const PROGRESS_BAR_WIDTH: f32 = 700.0;
const ROUNDED_CORNERS_PADDING: f32 = 7.5;
//THICKNESS IS ALSO THE HEIGHT OF THE WIDGET
const BAR_THICKNESS: f32 = 15.0;
const PADDING_BETWEEN_BARS: f32 = 40.0;
const PROGRESS_BAR_TITLE_FONT_SIZE: f32 = 24.0;

//---------Environment constants
const BUTTON_WIDTH: f32 = 50.0;
const BUTTON_HEIGHT: f32 = 25.8;

//-------Variables used my multiple methods
static PROGRESS_BAR_WIDGET_WIDTH: f32 = PROGRESS_BAR_WIDTH + ROUNDED_CORNERS_PADDING * 2.0;
#[derive(Clone, Debug)]
pub enum ProgressBarMessage {
    IncrementCurrentValue(ProgressBarType),
    DecrementCurrentValue(ProgressBarType),
    UpdateProgressBarAnimation(Event<f32>),
}

#[derive(Debug, Display, Clone)]
pub enum ProgressBarType {
    Water,
    Steps,
    Sleep,
}

impl ProgressBarType {
    fn get_unit(&self) -> String {
        match self {
            ProgressBarType::Water => "L".to_string(),
            ProgressBarType::Steps => "".to_string(),
            ProgressBarType::Sleep => "h".to_string(),
        }
    }

    fn get_completed_bar_color(&self) -> Color {
        match self {
            ProgressBarType::Water => RareMascot::Whale.get_primary_color(),
            ProgressBarType::Steps => RareMascot::Chameleon.get_primary_color(),
            ProgressBarType::Sleep => EpicMascot::Capybara.get_primary_color(),
        }
    }

    fn get_remaining_bar_color(&self) -> Color {
        match self {
            ProgressBarType::Water => RareMascot::Whale.get_secondary_color(),
            ProgressBarType::Steps => RareMascot::Chameleon.get_secondary_color(),
            ProgressBarType::Sleep => EpicMascot::Capybara.get_secondary_color(),
        }
    }
}

impl ProgressBarMessage {
    pub fn update_progress_bar_message(self, app: &mut App) -> Task<Message> {
        match self {
            ProgressBarMessage::UpdateProgressBarAnimation(event) => {
                app.widget_manager
                    .progress_bar_state_manager
                    .water_progress_bar_state
                    .animation_progress
                    .update(event);

                app.widget_manager
                    .progress_bar_state_manager
                    .water_progress_bar_state
                    .update_progress_bar();
                Task::none()
            }

            ProgressBarMessage::IncrementCurrentValue(progress_bar_type) => {
                //unwrap can't fail since the Message IncrementCounterValue can only be sent in edit mode, which means pending_progress_state is not None
                let progress_bars = app
                    .widget_manager
                    .pending_progress_bar_state_manager
                    .as_mut()
                    .unwrap();

                match progress_bar_type {
                    ProgressBarType::Water => progress_bars
                        .water_progress_bar_state
                        .increment(progress_bar_type),
                    ProgressBarType::Steps => progress_bars
                        .steps_progress_bar_state
                        .increment(progress_bar_type),
                    ProgressBarType::Sleep => progress_bars
                        .sleep_progress_bar_state
                        .increment(progress_bar_type),
                };
                Task::none()
            }

            ProgressBarMessage::DecrementCurrentValue(progress_bar_type) => {
                //unwrap can't fail since the Message DecrementCurrentValue can only be sent in edit mode, which means pending_progress_state is not None
                let progress_bars = app
                    .widget_manager
                    .pending_progress_bar_state_manager
                    .as_mut()
                    .unwrap();
                match progress_bar_type {
                    ProgressBarType::Water => {
                        if progress_bars.water_progress_bar_state.current_value > 0.0 {
                            progress_bars
                                .water_progress_bar_state
                                .decrement(progress_bar_type)
                        }
                    }
                    ProgressBarType::Steps => {
                        if progress_bars.steps_progress_bar_state.current_value > 0.0 {
                            progress_bars
                                .steps_progress_bar_state
                                .decrement(progress_bar_type)
                        }
                    }
                    ProgressBarType::Sleep => {
                        if progress_bars.sleep_progress_bar_state.current_value > 0.0 {
                            progress_bars
                                .sleep_progress_bar_state
                                .decrement(progress_bar_type)
                        }
                    }
                };
                Task::none()
            }
        }
    }
}

pub struct ProgressBarWidget<'a> {
    progress_bar_state: &'a ProgressBarState,
    progress_bar_type: ProgressBarType,
}

impl<'a> ProgressBarWidget<'a> {
    pub(crate) fn new(
        progress_bar_state: &'a ProgressBarState,
        progress_bar_type: ProgressBarType,
    ) -> Self {
        ProgressBarWidget {
            progress_bar_state,
            progress_bar_type,
        }
    }
    pub(crate) fn view(self) -> Element<'a, Message> {
        let draw_percentage = &self.progress_bar_state.animation_progress;

        let canvas = canvas(self)
            .width(PROGRESS_BAR_WIDGET_WIDTH)
            .height(BAR_THICKNESS);

        Animation::new(draw_percentage, canvas)
            .on_update(|event| {
                Message::ProgressBar(ProgressBarMessage::UpdateProgressBarAnimation(event))
            })
            .into()
    }
}

#[derive(Debug)]
pub struct ProgressBarState {
    progress_bar: Cache,
    pub animation_progress: Animated<f32>,
    pub current_value: f32,
    pub goal_value: f32,
}

impl ProgressBarState {
    pub(crate) fn increment(&mut self, progress_bar_type: ProgressBarType) {
        let value_to_increment = match progress_bar_type {
            ProgressBarType::Water => 0.25,
            ProgressBarType::Steps => 500.0,
            ProgressBarType::Sleep => 0.5,
        };

        self.current_value += value_to_increment;
    }

    pub(crate) fn decrement(&mut self, progress_bar_type: ProgressBarType) {
        let value_to_increment = match progress_bar_type {
            ProgressBarType::Water => -0.25,
            ProgressBarType::Steps => -500.0,
            ProgressBarType::Sleep => -0.5,
        };

        self.current_value += value_to_increment;
    }
}

impl ProgressBarState {
    pub(crate) fn update_progress_bar(&self) {
        self.progress_bar.clear();
    }
}

impl ProgressBarState {
    pub fn new(current_value: f32, goal_value: f32) -> Self {
        let animation_motion = Motion {
            response: Duration::from_millis(3000),
            damping: Motion::SMOOTH.damping(),
        };

        Self {
            progress_bar: Cache::default(),
            animation_progress: Animated::new(0.0, animation_motion),
            current_value,
            goal_value,
        }
    }
}

impl Default for ProgressBarState {
    fn default() -> Self {
        Self::new(0.0, 1.0)
    }
}

impl canvas::Program<Message> for ProgressBarWidget<'_> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: iced::widget::canvas::Event,
        _bounds: Rectangle,
        _cursor: iced_core::mouse::Cursor,
    ) -> (iced::event::Status, std::option::Option<Message>) {
        self.progress_bar_state.update_progress_bar();

        (
            event::Status::Ignored,
            Some(crate::client::gui::user_interface::Message::ProgressBar(
                ProgressBarMessage::UpdateProgressBarAnimation(iced_anim::Event::Target(1.0)),
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
        let progress_bar_geometry =
            self.progress_bar_state
                .progress_bar
                .draw(renderer, bounds.size(), |frame| {
                    //DRAW BAR REPRESENTING COMPLETED PERCENTAGE
                    draw_bar_completion(frame, self);

                    //DRAW BAR REPRESENTING UNCOMPLETED PERCENTAGE
                    draw_bar_remaining(frame, self);
                });

        vec![progress_bar_geometry]
    }
}

fn draw_bar_completion(frame: &mut Frame, progress_bar_widget: &ProgressBarWidget) {
    let start_point = Point {
        x: Point::ORIGIN.x + ROUNDED_CORNERS_PADDING,
        y: Point::ORIGIN.y + frame.center().y,
    };

    let end_point = Point {
        x: Point::ORIGIN.x
            + ROUNDED_CORNERS_PADDING
            + calculate_length_completion_bar(progress_bar_widget.progress_bar_state),
        y: Point::ORIGIN.y + frame.center().y,
    };

    frame.stroke(
        &Path::line(start_point, end_point),
        generate_stroke(
            BAR_THICKNESS,
            progress_bar_widget
                .progress_bar_type
                .get_completed_bar_color(),
        ),
    )
}
fn draw_bar_remaining(frame: &mut Frame, progress_bar_widget: &ProgressBarWidget) {
    if progress_bar_widget.progress_bar_state.current_value
        < progress_bar_widget.progress_bar_state.goal_value
    {
        let start_point = Point {
            x: Point::ORIGIN.x
                + ROUNDED_CORNERS_PADDING
                + calculate_length_completion_bar(progress_bar_widget.progress_bar_state)
                + PADDING_BETWEEN_BARS / 2.0,
            y: Point::ORIGIN.y + frame.center().y,
        };

        let end_point = Point {
            x: PROGRESS_BAR_WIDTH,
            y: Point::ORIGIN.y + frame.center().y,
        };

        frame.stroke(
            &Path::line(start_point, end_point),
            generate_stroke(
                BAR_THICKNESS,
                progress_bar_widget
                    .progress_bar_type
                    .get_remaining_bar_color(),
            ),
        )
    }
}

//LOGIC

fn calculate_length_completion_bar(progress_bar_state: &ProgressBarState) -> f32 {
    let total_possible_bar_length = PROGRESS_BAR_WIDTH - ROUNDED_CORNERS_PADDING;
    let percentage = match progress_bar_state.current_value / progress_bar_state.goal_value {
        1.0.. => 1.0,
        _ => progress_bar_state.current_value / progress_bar_state.goal_value,
    };
    let padding_to_other_bar = match percentage {
        1.0 => 0.0,
        0.0 => 0.0,
        _ => PADDING_BETWEEN_BARS / 2.0,
    };

    //separate handling for the case in which the padding is too big and the bar goes backwards

    let length = total_possible_bar_length * percentage - padding_to_other_bar;

    match length {
        ..0.0 => 0.0,
        _ => length,
    }
}

pub fn create_progress_bar_environment<'a>(
    progress_bar_widget: ProgressBarWidget<'a>,
    mascot: &Mascot,
    counter: bool,
) -> Element<'a, Message> {
    let title = text(progress_bar_widget.progress_bar_type.to_string())
        .font(FIRA_SANS_EXTRABOLD)
        .size(PROGRESS_BAR_TITLE_FONT_SIZE)
        .color(TEXT_COLOR);

    let progress_text = text(format!(
        "{}/{} {}",
        progress_bar_widget.progress_bar_state.current_value,
        progress_bar_widget.progress_bar_state.goal_value,
        progress_bar_widget.progress_bar_type.get_unit()
    ))
    .font(FIRA_SANS_EXTRABOLD)
    .size(PROGRESS_BAR_TITLE_FONT_SIZE)
    .color(TEXT_COLOR);

    let mut header = Row::new().push(title);

    if counter {
        let increment_button = create_text_button(
            mascot,
            "+".to_string(),
            Active,
            Some(Radius {
                top_left: 0.0,
                top_right: DEFAULT_BUTTON_RADIUS,
                bottom_right: DEFAULT_BUTTON_RADIUS,
                bottom_left: 0.0,
            }),
        )
        .width(BUTTON_WIDTH)
        .height(BUTTON_HEIGHT)
        .on_press(ProgressBar(ProgressBarMessage::IncrementCurrentValue(
            progress_bar_widget.progress_bar_type.clone(),
        )));

        let decrement_button = create_text_button(
            mascot,
            "-".to_string(),
            InactiveTab,
            Some(Radius {
                top_left: DEFAULT_BUTTON_RADIUS,
                top_right: 0.0,
                bottom_right: 0.0,
                bottom_left: DEFAULT_BUTTON_RADIUS,
            }),
        )
        .width(BUTTON_WIDTH)
        .height(BUTTON_HEIGHT)
        .on_press(ProgressBar(ProgressBarMessage::DecrementCurrentValue(
            progress_bar_widget.progress_bar_type.clone(),
        )));

        header = header
            .push(Space::with_width(Length::Fixed(INDENT)))
            .push(decrement_button)
            .push(increment_button)
    }

    header = header
        .push(Space::with_width(Length::Fill))
        .push(progress_text);

    let progress_bar = progress_bar_widget.view();

    let progress_widget = Column::new()
        .push(header)
        .push(progress_bar)
        .spacing(INDENT)
        .width(PROGRESS_BAR_WIDGET_WIDTH)
        .align_x(Horizontal::Center);

    let content = container(progress_widget)
        .style(create_container_style(ContainerStyle::Default, None, None))
        .padding(INDENT);

    content.into()
}
