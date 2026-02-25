use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::scrollable::{main_style, mascot_style};
use crate::client::gui::bb_theme::text_format::{format_button_text, FIRA_SANS_EXTRABOLD};
use crate::client::gui::bb_widget::activity_widget::date_utils::DateScope;
use crate::client::gui::bb_widget::bmi_calculator::BMIWidget;
use crate::client::gui::bb_widget::chart::chart_environment_widget;
use crate::client::gui::bb_widget::circle_widget;
use crate::client::gui::bb_widget::workout::WorkoutWidget;
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{container, image, text, Column, Row, Scrollable, Space};
use iced::{Element, Renderer};
use iced_core::alignment::Vertical;
use iced_core::{Length, Padding};
use crate::client::gui::size::FRAME_WIDTH;

impl App {
    pub fn homescreen(&self) -> Element<Message> {
        let welcome_back_text = format_button_text(text("Welcome back!")).size(42);
        let activity_widget: Element<Message> = self.widget_manager.activity_widget.view(self);

        let mascot_message_icon = format_button_text(text("<")).size(31);
        let mascot_message = format_button_text(text("Keep going!")).size(31);
        let message_with_icon = Row::new()
            .push(mascot_message_icon)
            .push(mascot_message)
            .spacing(20);
        let mascot_image = image(self.mascot_manager.selected_mascot.get_file_path());
        let mascot_with_message =
                Row::new()
                    .push(mascot_image)
                    .push(container(message_with_icon).align_y(Vertical::Top));

        let activity_widget_with_welcome =
            Column::new()
                .push(welcome_back_text)
                .push(activity_widget)
                .spacing(20.0);

        let activity_widget_with_mascot:Element<Message> = match self.widget_manager.activity_widget.current_scope {
            DateScope::Year => {
                Row::new().push(
                    activity_widget_with_welcome
                ).align_y(Vertical::Center).height(250).into()
            }
            _ =>  Row::new()
                .push(activity_widget_with_welcome)
                .push(Space::with_width(Length::Fixed(62.5)))
                .push(mascot_with_message)
                .align_y(Vertical::Center).height(250).into()
        };

        let track_new_workout_text = text("Track a new workout!").font(FIRA_SANS_EXTRABOLD).color(color::TEXT_COLOR).size(30);

        let workout_presets = WorkoutWidget::<Renderer>::create_preset_row(&self.mascot_manager.selected_mascot);

        let stats_text = format_button_text(text("Stats")).size(30);

        let chart_widget = chart_environment_widget(self);
        let circle_widget = circle_widget::CircleWidget::new(self).view();
        let bmi_widget= BMIWidget::new(self).view();
        let circle_widgets_column = Column::new()
            .push(circle_widget)
            .push(bmi_widget)
            .spacing(20.0);

        let chart_widget_with_circle_widget = Row::new()
            .push(chart_widget)
            .push(circle_widgets_column)
            .spacing(20);

        let home_screen_content = Column::new()
            .push(activity_widget_with_mascot)
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(track_new_workout_text)
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(Scrollable::new(workout_presets).direction(Direction::Horizontal(Scrollbar::new().scroller_width(6))).style(|theme: &_, status: iced::widget::scrollable::Status| {
                mascot_style(status,self.mascot_manager.selected_mascot)
            }))
            .push(Space::with_height(Length::Fixed(10.0)))
            .push(stats_text)
            .push(Space::with_height(Length::Fixed(20.0)))
            .push(chart_widget_with_circle_widget)
            //.push(Space::with_height(Length::Fixed(30.0)))
            .padding(Padding {
                top: 20.0,
                left: 20.0,
                ..0.0.into()
            });

        Scrollable::new(home_screen_content)
            .width(FRAME_WIDTH)
            .style(|theme,status|main_style(status, self.mascot_manager.selected_mascot))
            .direction(Direction::Vertical(
                Scrollbar::new()
                    .scroller_width(7).margin(6)))


            .into()
    }
}