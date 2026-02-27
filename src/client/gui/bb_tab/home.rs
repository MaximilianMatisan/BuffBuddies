use crate::client::gui::app::App;
use crate::client::gui::bb_tab::workout::view_presets;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::scrollable::{
    ScrollableExtension, ScrollableStyle, TAB_SCROLLBAR_PADDING, TAB_SCROLLBAR_WIDTH,
    WIDGET_SCROLLBAR_WIDTH, create_scrollable,
};
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_button_text};
use crate::client::gui::bb_widget::activity_widget::date_utils::DateScope;
use crate::client::gui::bb_widget::bmi_calculator::BMIWidget;
use crate::client::gui::bb_widget::chart::chart_environment_widget;
use crate::client::gui::bb_widget::circle_widget;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{Column, Row, Space, container, row, text};
use iced_core::alignment::Vertical;
use iced_core::{Length, Padding};

const WELCOME_BACK_TEXT_SIZE: u16 = 42;
const SECONDARY_TEXT_SIZE: u16 = 31;
const SPACING: f32 = 20.0;
const DISTANCE_MASCOT_ACTIVITY_WIDGET: f32 = 62.5;
const HEIGHT_MASCOT: u16 = 250;

impl App {
    pub fn homescreen(&self) -> Element<Message> {
        let welcome_back_text =
            format_button_text(text("Welcome back!")).size(WELCOME_BACK_TEXT_SIZE);
        let activity_widget: Element<Message> = self.widget_manager.activity_widget.view(self);

        let mascot_message_icon = format_button_text(text("<")).size(SECONDARY_TEXT_SIZE);
        let mascot_message = format_button_text(text("Keep going!")).size(SECONDARY_TEXT_SIZE);
        let message_with_icon = Row::new()
            .push(mascot_message_icon)
            .push(mascot_message)
            .spacing(SPACING);
        let mascot_image = self.mascot_manager.view_active_mascot();
        let mascot_with_message = Row::new()
            .push(mascot_image)
            .push(container(message_with_icon).align_y(Vertical::Top));

        let activity_widget_with_welcome = Column::new()
            .push(welcome_back_text)
            .push(activity_widget)
            .spacing(SPACING);

        let activity_widget_with_mascot: Element<Message> =
            match self.widget_manager.activity_widget.current_scope {
                DateScope::Year => Row::new()
                    .push(activity_widget_with_welcome)
                    .align_y(Vertical::Center)
                    .height(HEIGHT_MASCOT)
                    .into(),
                _ => Row::new()
                    .push(activity_widget_with_welcome)
                    .push(Space::with_width(Length::Fixed(
                        DISTANCE_MASCOT_ACTIVITY_WIDGET,
                    )))
                    .push(mascot_with_message)
                    .align_y(Vertical::Center)
                    .height(HEIGHT_MASCOT)
                    .into(),
            };

        let mut track_new_workout_and_presets = Column::new();

        if !self.workout_preset_manager.presets.is_empty() {
            let track_new_workout_text = text("Track a new workout!")
                .font(FIRA_SANS_EXTRABOLD)
                .color(color::TEXT_COLOR)
                .size(SECONDARY_TEXT_SIZE);

            let workout_presets = view_presets(
                &self.mascot_manager.selected_mascot,
                &self.workout_preset_manager.presets,
            );
            track_new_workout_and_presets = track_new_workout_and_presets
                .push(track_new_workout_text)
                .push(Space::with_height(Length::Fixed(SPACING)))
                .push(
                    create_scrollable(
                        workout_presets,
                        self.mascot_manager.selected_mascot,
                        ScrollableStyle::Mascot,
                    )
                    .add_horizontal_scrollbar(WIDGET_SCROLLBAR_WIDTH, 0.0),
                )
                .push(Space::with_height(Length::Fixed(INDENT)))
        }

        let stats_text = format_button_text(text("Stats")).size(SECONDARY_TEXT_SIZE);

        let chart_widget = chart_environment_widget(self);
        let circle_widget = circle_widget::CircleWidget::new(self).view();
        let bmi_widget = BMIWidget::new(self).view();
        let circle_widgets_column = Column::new()
            .push(circle_widget)
            .push(bmi_widget)
            .spacing(SPACING);

        let chart_widget_with_circle_widget = Row::new()
            .push(chart_widget)
            .push(circle_widgets_column)
            .spacing(SPACING);

        let home_screen_content = Column::new()
            .push(activity_widget_with_mascot)
            .push(Space::with_height(Length::Fixed(SPACING)))
            .push(track_new_workout_and_presets)
            .push(stats_text)
            .push(Space::with_height(Length::Fixed(SPACING)))
            .push(chart_widget_with_circle_widget)
            .padding(Padding {
                top: SPACING,
                ..0.0.into()
            });

        create_scrollable(
            row![home_screen_content, Space::with_width(Length::Fill)], //the row is needed for the scrollbar to go to the end of the frame,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Default,
        )
        .add_vertical_scrollbar(TAB_SCROLLBAR_WIDTH, TAB_SCROLLBAR_PADDING)
        .into()
    }
}
