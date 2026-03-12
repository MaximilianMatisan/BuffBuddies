use crate::client::gui::app::App;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_theme::color::{BACKGROUND_COLOR, CONTAINER_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::custom_button::{ButtonType, create_button_style};
use crate::client::gui::bb_theme::scrollable::{
    ScrollableExtension, ScrollableStyle, TAB_SCROLLBAR_PADDING, TAB_SCROLLBAR_WIDTH,
    create_scrollable,
};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::general_exercise_info_elements::general_exercise_browser;
use crate::client::gui::bb_widget::new_widget;
use crate::client::gui::bb_widget::preset_workout_rows::{
    view_preset_row, view_recent_workout_row,
};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::size::FRAME_WIDTH;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{Column, Row, Space, Stack, container, text};
use iced_anim::Motion;
use iced_core::image::Handle;
use iced_core::{Alignment, Border, Length, Padding, Theme};
use std::time::Duration;

const SPACING: f32 = 20.0;
const MASCOT_SIZE: f32 = 200.0;
const TEXT_SIZE: f32 = 31.0;
const HEIGHT_RECENT_WORKOUTS_TEXT: f32 = 100.0; // Vertical space reserved for the "Recent Workouts" header
const HEIGHT_RECENT_WORKOUTS_WIDGET: f32 = 285.0; // Vertical space reserved for the row of recent workouts

impl App {
    pub fn workout_screen(&self) -> Element<'_, Message> {
        let selected_mascot_image: iced::widget::Image<Handle> = self
            .mascot_manager
            .view_active_mascot()
            .height(MASCOT_SIZE)
            .width(MASCOT_SIZE);
        let recent_workouts_text = container(
            text("Recent workouts")
                .font(FIRA_SANS_EXTRABOLD)
                .color(TEXT_COLOR)
                .size(TEXT_SIZE),
        )
        .height(HEIGHT_RECENT_WORKOUTS_TEXT)
        .align_y(Alignment::End);

        let background_mascot_with_text = Row::new()
            .push(selected_mascot_image)
            .push(Space::new().width(Length::FillPortion(1)))
            .push(recent_workouts_text)
            .push(Space::new().width(Length::FillPortion(10)))
            .height(HEIGHT_RECENT_WORKOUTS_WIDGET)
            .width(FRAME_WIDTH);

        let recent_workouts_row = Row::new()
            .push(new_widget::new_workout_widget_button(
                self.mascot_manager.selected_mascot,
            ))
            .push(view_recent_workout_row(
                &self.mascot_manager.selected_mascot,
                &self.exercise_manager.recent_workouts,
            ))
            .spacing(INDENT);

        let aligned_recent_workouts = container(recent_workouts_row)
            .height(HEIGHT_RECENT_WORKOUTS_WIDGET)
            .align_y(Alignment::End);

        let recent_workouts_with_mascot = Stack::new()
            .push(background_mascot_with_text)
            .push(aligned_recent_workouts);

        let workout_preset_scrollable = view_preset_row(
            &self.mascot_manager.selected_mascot,
            &self.workout_preset_manager.presets,
        );

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
                .color(TEXT_COLOR)
                .size(TEXT_SIZE),
        )
        .on_press(Message::Select(Tab::PresetOverview))
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
                ButtonType::Normal,
            )
        });

        let exercise_browser_container = container(general_exercise_browser(
            &self.mascot_manager.selected_mascot,
            &self.exercise_manager,
        ))
        .padding(Padding {
            right: SPACING,
            ..0.0.into()
        });

        let workout_interface = Column::new()
            .push(recent_workouts_with_mascot)
            .push(workout_presets_button)
            .push(workout_preset_scrollable_with_button)
            .push(exercise_browser_container)
            .spacing(SPACING)
            .padding(Padding {
                top: SPACING,
                bottom: SPACING,
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
