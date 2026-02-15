use crate::client::gui::app::App;
use crate::client::gui::bb_widget::chart::chart_environment_widget;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{container, image, stack, text, Column, Row, Scrollable, Space};
use iced_core::alignment::Vertical;
use iced_core::{Length, Padding};
use iced_core::image::{Handle, Image};
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::text_format::{format_button_text, FIRA_SANS_EXTRABOLD};
use crate::client::gui::bb_widget::activity_widget::date_utils::DateScope;
use crate::client::gui::bb_widget::workout::{WorkoutWidget, DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT};
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::widget::scrollable;
use iced::widget::scrollable::{Rail, Scroller, Status, Style};
use iced_core::{color, Background, Border, Color, Theme};
use crate::client::gui::bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR;
use crate::client::gui::bb_widget::circle_widget;
use crate::common::mascot_mod::mascot::Mascot;

impl App {
    pub fn homescreen(&self) -> Element<Message> {
        let welcome_back_text = format_button_text(text("Welcome back!")).size(42);
        let activity_widget: Element<Message> = self.widget_manager.activity_widget.view(self);


        //TODO: draw mascot with text in canvas
        let mascot_message_icon = format_button_text(text("<")).size(31);
        let mascot_message = format_button_text(text("Stay hard!")).size(31);
        let message_with_icon = Row::new()
            .push(mascot_message_icon)
            .push(mascot_message)
            .spacing(20);
        let mascot_image = image(self.mascot_manager.selected_mascot.get_file_path());
        let mascot_with_message = stack![
                Row::new()
                    .push(mascot_image)
                    .push(container(message_with_icon).align_y(Vertical::Top))
            ];

        let activity_widget_with_welcome =
            Column::new()
                .push(welcome_back_text)
                .push(activity_widget)
                .spacing(10);

        let activity_widget_with_mascot:Element<Message> = match self.widget_manager.activity_widget.current_scope {
            DateScope::Year => {
                Row::new().push(
                    activity_widget_with_welcome
                ).align_y(Vertical::Center).height(250).into()
            }
            _ =>  Row::new()
                .push(activity_widget_with_welcome)
                .push(Space::with_width(Length::FillPortion(2)))
                .push(mascot_with_message)
                .push(Space::with_width(Length::FillPortion(4)))
                .align_y(Vertical::Center).height(250).into()
        };

        let track_new_workout_text = text("Track a new workout!").font(FIRA_SANS_EXTRABOLD).color(color::TEXT_COLOR).size(30);

        let mut workout_presets = Row::new()
            .height(DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT + 15.0).spacing(10);

        let base_path = "assets/images/";

        let images = [
            "duck_bench.png",
            "duck_pullup.png",
            "duck_squats.png",
            "running_duck.png",
        ];

        for image in images {
            let path = format!("{base_path}{image}");

            workout_presets = workout_presets.push(
                WorkoutWidget::default_workout_preset_widget()
                    .set_image(Some(Image::new(Handle::from_path(path)))),
            );
        }

        let stats_text = format_button_text(text("Stats")).size(30);

        let chart_widget = chart_environment_widget(self);
        let circle_widget = circle_widget::CircleWidget::new(self).view();

        let chart_widget_with_circle_widget = Row::new()
            .push(circle_widget)
            .push(chart_widget)
            .spacing(20);

        let test_content = Column::new()
            .push(activity_widget_with_mascot)
            .push(track_new_workout_text)
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(Scrollable::new(workout_presets).direction(Direction::Horizontal(Scrollbar::new().scroller_width(6))).style(|theme: &_, status: iced::widget::scrollable::Status| {
                mascot_style(status,self.mascot_manager.selected_mascot)
            }))
            .push(stats_text)
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(chart_widget_with_circle_widget)
            .push(Space::with_height(Length::Fixed(30.0)))
            .padding(Padding {
                top: 20.0,
                right: 0.0,
                bottom: 0.0,
                left: 20.0,
            });

        Scrollable::new(test_content)
            .direction(Direction::Vertical(
                Scrollbar::new().scroller_width(7).margin(4)))
            .style(|theme,status|main_style(status, self.mascot_manager.selected_mascot))
            .into()
    }
}

pub fn mascot_style(status: Status, mascot: Mascot) ->  Style {

    let mut scrollable_style = iced::widget::scrollable::Style {
        container: Default::default(),
        vertical_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        horizontal_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        gap: None,
    };

    let mut hovered_color = mascot.get_primary_color();
    hovered_color.a = 0.4;
    let mut dragged_color = mascot.get_secondary_color();
    dragged_color.a = 0.7;

    match status {

        scrollable::Status::Active => {
            scrollable_style
        }
        scrollable::Status::Hovered{ .. } => {
            scrollable_style.horizontal_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: hovered_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.5,
                        radius: 6.into()
                    },
                }
            };

            scrollable_style.vertical_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: hovered_color,
                    border: Border {
                        color:Color::TRANSPARENT,
                        width: 0.5,
                        radius: 6.into()
                    },
                }
            };
            scrollable_style
        }
        scrollable::Status::Dragged{ .. } => {
            scrollable_style.horizontal_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color:dragged_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.1,
                        radius: 6.into()
                    },
                }
            };
            scrollable_style.vertical_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: dragged_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.1,
                        radius: 6.into()
                    },
                }
            };

            scrollable_style
        }
    }
}

pub fn main_style (status: Status, mascot: Mascot) ->  Style {

    let mut scrollable_style = iced::widget::scrollable::Style {
        container: Default::default(),
        vertical_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        horizontal_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        gap: None,
    };

    let mut hovered_color = HIGHLIGHTED_CONTAINER_COLOR;//color!(180,180,180);
    hovered_color.a = 0.4;
    let mut dragged_color = hovered_color;
    dragged_color.a = 0.7;

    match status {

        scrollable::Status::Active => {
            scrollable_style
        }
        scrollable::Status::Hovered{ .. } => {
            scrollable_style.horizontal_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: mascot.get_primary_color(),
                    border: Border {
                        color: mascot.get_primary_color(),
                        width: 0.5,
                        radius: 6.into()
                    },
                }
            };

            scrollable_style.vertical_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: hovered_color,
                    border: Border {
                        color:Color::TRANSPARENT,
                        width: 0.5,
                        radius: 6.into()
                    },
                }
            };
            scrollable_style
        }
        scrollable::Status::Dragged{ .. } => {
            scrollable_style.horizontal_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: mascot.get_secondary_color(),
                    border: Border {
                        color: mascot.get_secondary_color(),
                        width: 0.1,
                        radius: 6.into()
                    },
                }
            };
            scrollable_style.vertical_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: dragged_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.1,
                        radius: 6.into()
                    },
                }
            };

            scrollable_style
        }
    }
}
