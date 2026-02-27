use crate::client::gui::app::App;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::color::{BACKGROUND_COLOR, CONTAINER_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::create_button_style;
use crate::client::gui::bb_theme::scrollable::{
    SCROLLBAR_PADDING, ScrollableExtension, ScrollableStyle, TAB_SCROLLBAR_PADDING,
    TAB_SCROLLBAR_WIDTH, WIDGET_SCROLLBAR_WIDTH, create_scrollable,
};
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_description_text};
use crate::client::gui::bb_widget::general_exercise_info_elements::display_general_exercise_infos;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::bb_widget::workout::{DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT, WorkoutWidget};
use crate::client::gui::bb_widget::{new_widget, workout};
use crate::client::gui::size::FRAME_WIDTH;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{Column, Row, Space, Stack, container, text};
use iced_anim::Motion;
use iced_core::alignment::Vertical;
use iced_core::image::Handle;
use iced_core::{Alignment, Border, Length, Padding, Theme};
use std::time::Duration;

const SPACING: f32 = 20.0;
const MASCOT_SIZE: u16 = 200;
const TEXT_SIZE: u16 = 31;
const HEIGHT_RECENT_WORKOUTS_TEXT: u16 = 100; // Vertical space reserved for the "Recent Workouts" header
const HEIGHT_RECENT_WORKOUTS_WIDGET: u16 = 285; // Vertical space reserved for the row of recent workouts

impl App {
    pub fn workout_screen(&self) -> Element<Message> {
        let selected_mascot_image: iced::widget::Image<Handle> = self
            .mascot_manager
            .view_active_mascot()
            .height(MASCOT_SIZE)
            .width(MASCOT_SIZE);
        let recent_workouts_text = container(
            text("Recent workouts")
                .font(FIRA_SANS_EXTRABOLD)
                .color(color::TEXT_COLOR)
                .size(TEXT_SIZE),
        )
        .height(HEIGHT_RECENT_WORKOUTS_TEXT)
        .align_y(Alignment::End);

        let background_mascot_with_text = container(
            Row::new()
                .push(selected_mascot_image)
                .push(Space::with_width(Length::FillPortion(1)))
                .push(recent_workouts_text)
                .push(Space::with_width(Length::FillPortion(10))),
        )
        .height(HEIGHT_RECENT_WORKOUTS_WIDGET)
        .width(FRAME_WIDTH);

        let recent_workouts_row = Row::new()
            .push(new_widget::new_workout_widget_button(
                self.mascot_manager.selected_mascot,
            ))
            .push(WorkoutWidget::default_recent_workout_widget())
            .push(WorkoutWidget::default_recent_workout_widget())
            .push(WorkoutWidget::default_recent_workout_widget())
            .spacing(INDENT);

        let aligned_recent_workouts = container(recent_workouts_row)
            .height(HEIGHT_RECENT_WORKOUTS_WIDGET)
            .align_y(Alignment::End);

        let recent_workouts_with_mascot = Stack::new()
            .push(background_mascot_with_text)
            .push(aligned_recent_workouts);

        let mut workout_preset_row = Row::new()
            .height(DEFAULT_WORKOUT_PRESET_WIDGET_HEIGHT + SCROLLBAR_PADDING)
            .spacing(INDENT);

        for preset in &self.workout_preset_manager.presets {
            workout_preset_row =
                workout_preset_row.push(workout::WorkoutWidget::new_workout_preset_widget(preset));
        }

        let workout_preset_scrollable = create_scrollable(
            workout_preset_row,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Mascot,
        )
        .add_horizontal_scrollbar(WIDGET_SCROLLBAR_WIDTH, 0.0);

        let workout_preset_scrollable_with_button = Row::new()
            .push(new_widget::new_preset_widget_button(
                self.mascot_manager.selected_mascot,
            ))
            .push(workout_preset_scrollable)
            .spacing(INDENT);

        let workout_presets_button_border = Border {
            radius: 10.0.into(),
            ..Default::default()
        };

        let workout_presets_button = iced_anim::widget::button(
            text("Workout presets >")
                .font(FIRA_SANS_EXTRABOLD)
                .color(color::TEXT_COLOR)
                .size(TEXT_SIZE),
        )
        .on_press(Message::Select(Tab::Settings)) //TODO: Screen with presets
        .animation(Motion {
            damping: Motion::SMOOTH.damping,
            response: Duration::from_millis(350),
        })
        .style(move |_: &Theme, status| {
            create_button_style(
                status,
                workout_presets_button_border,
                BACKGROUND_COLOR,
                CONTAINER_COLOR,
                CONTAINER_COLOR,
            )
        });

        let general_exercise_info_elements = display_general_exercise_infos(
            &self.mascot_manager.selected_mascot,
            &self.exercise_manager,
        );
        let browse_exercises_title = text("Browse exercises")
            .size(TEXT_SIZE)
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR);

        let title_bar = Row::new()
            .push(browse_exercises_title)
            .push(format_description_text(text(format!(
                " - {} results",
                self.exercise_manager.exercises.len()
            ))))
            .align_y(Vertical::Center);

        let exercise_browser = Column::new()
            .push(title_bar)
            .push(general_exercise_info_elements)
            .spacing(SPACING);

        let exercise_info_container = container(exercise_browser)
            .style(create_container_style(ContainerStyle::Default, None, None))
            .padding(LARGE_INDENT);

        let workout_interface = Column::new()
            .push(recent_workouts_with_mascot)
            .push(workout_presets_button)
            .push(workout_preset_scrollable_with_button)
            .push(exercise_info_container)
            .spacing(SPACING)
            .padding(Padding {
                top: SPACING,
                ..0.0.into()
            });

        let scrollable_workout_interface = create_scrollable(
            workout_interface,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Default,
        )
        .add_vertical_scrollbar(TAB_SCROLLBAR_WIDTH, TAB_SCROLLBAR_PADDING);

        scrollable_workout_interface.into()
    }
}
