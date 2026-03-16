use crate::client::backend::widget_state::progress_bar_manager::ProgressBarStateManager;
use crate::client::gui::app::App;
use crate::client::gui::bb_tab::health::HealthMessage::{SaveProgressBarChanges, SwitchEditMode};
use crate::client::gui::bb_tab::tab::FRAME_PADDING;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_theme::custom_button::ButtonStyle::InactiveTab;
use crate::client::gui::bb_theme::custom_button::{
    ButtonStyle, create_element_button, create_text_button,
};
use crate::client::gui::bb_theme::scrollable::{
    ScrollableExtension, ScrollableStyle, TAB_SCROLLBAR_PADDING, TAB_SCROLLBAR_WIDTH,
    create_scrollable,
};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::bmi_calculator;
use crate::client::gui::bb_widget::chart_widget::chart::health_chart_environment_widget;
use crate::client::gui::bb_widget::circle_widget::{CircleStart, CircleWidget};
use crate::client::gui::bb_widget::progress_bar::{
    ProgressBarWidget, create_progress_bar_environment,
};
use crate::client::gui::bb_widget::stats::health_stat_container;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::user_interface::Message;
use crate::client::gui::user_interface::Message::HealthTab;
use crate::common::mascot_mod::epic_mascot::EpicMascot::Capybara;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::rare_mascot::RareMascot::{Chameleon, Whale};
use crate::common::user_mod::user_goals::GoalType;
use iced::widget::{Column, Row, Space, container, image, row, text};
use iced::{Element, Task};
use iced_core::alignment::Vertical;
use iced_core::image::Handle;
use iced_core::{Length, Padding};

#[derive(Debug, Clone, PartialEq)]
pub enum HealthMessage {
    SwitchEditMode,
    SaveProgressBarChanges,
}

impl HealthMessage {
    pub fn update_health_tab(health_tab_message: HealthMessage, app: &mut App) -> Task<Message> {
        match health_tab_message {
            SaveProgressBarChanges => {
                app.widget_manager.progress_bar_state_manager = app
                    .widget_manager
                    .pending_progress_bar_state_manager
                    .take() //now pending_progress_bar_state_manager is None and I get Option<ProgressBarState>
                    .unwrap() //take() is going to return Some(progressbar_state) since SaveProgressBarChanges can only be sent in edit_mode so it can't fail

                //MODE IS AUTOMATICALLY SWITCHED AT THIS POINT SINCE PENDING STATE IS NONE
            }

            SwitchEditMode => {
                let edit_mode = app
                    .widget_manager
                    .pending_progress_bar_state_manager
                    .is_some();
                if edit_mode {
                    app.widget_manager.pending_progress_bar_state_manager = None
                } else {
                    app.widget_manager.pending_progress_bar_state_manager =
                        Some(ProgressBarStateManager::duplicate_states(
                            &app.widget_manager.progress_bar_state_manager,
                        ))
                }
            }
        }

        Task::none()
    }
}

impl App {
    pub fn health_screen(&self) -> Element<'_, Message> {
        let edit_mode: bool = self
            .widget_manager
            .pending_progress_bar_state_manager
            .is_some();

        let mut water_progress_bar_state = &self
            .widget_manager
            .progress_bar_state_manager
            .water_progress_bar_state;
        let mut steps_progress_bar_state = &self
            .widget_manager
            .progress_bar_state_manager
            .steps_progress_bar_state;
        let mut sleep_progress_bar_state = &self
            .widget_manager
            .progress_bar_state_manager
            .sleep_progress_bar_state;

        if edit_mode {
            let editing_progress_bar_manager = &self
                .widget_manager
                .pending_progress_bar_state_manager
                .as_ref()
                .unwrap();
            water_progress_bar_state = &editing_progress_bar_manager.water_progress_bar_state;
            steps_progress_bar_state = &editing_progress_bar_manager.steps_progress_bar_state;
            sleep_progress_bar_state = &editing_progress_bar_manager.sleep_progress_bar_state;
        }

        let health_header = text("Health")
            .font(FIRA_SANS_EXTRABOLD)
            .color(color::TEXT_COLOR)
            .size(42);
        let bmi_widget = bmi_calculator::BMIWidget::new(self).view();
        let water_progress_bar = ProgressBarWidget::new(water_progress_bar_state, GoalType::Water);

        let steps_progress_bar = ProgressBarWidget::new(steps_progress_bar_state, GoalType::Steps);

        let sleep_progress_bar = ProgressBarWidget::new(sleep_progress_bar_state, GoalType::Sleep);

        let mut content = Column::new().push(health_header);

        let mut edit_buttons_row = Row::new();

        if edit_mode {
            let save_changes_button_content = row![
                iced::widget::image(Handle::from_path("assets/images/check_box.png")),
                text("Save").font(FIRA_SANS_EXTRABOLD).color(TEXT_COLOR)
            ]
            .align_y(Vertical::Center)
            .spacing(10.0);

            let save_changes_button = create_element_button(
                &self.mascot_manager.selected_mascot,
                save_changes_button_content.into(),
                ButtonStyle::Active,
                Some(DEFAULT_CONTAINER_RADIUS.into()),
            )
            .height(35.0)
            .on_press(Message::HealthTab(SaveProgressBarChanges));
            let discard_changes_button = create_text_button(
                &self.mascot_manager.selected_mascot,
                "Discard changes".to_string(),
                ButtonStyle::InactiveTab,
                Some(DEFAULT_CONTAINER_RADIUS.into()),
            )
            .height(35.0)
            .on_press(Message::HealthTab(SwitchEditMode));
            let button_row = Row::new()
                .push(save_changes_button)
                .push(discard_changes_button)
                .spacing(10.0);

            edit_buttons_row = button_row
        } else {
            let edit_mode_image =
                iced::widget::image(Handle::from_path("assets/images/pen_tool.png")).width(30.0);
            let edit_image_with_space = Row::new()
                .push(edit_mode_image)
                .height(Length::Fill)
                .align_y(Vertical::Center);

            let edit_mode_button = create_element_button(
                &self.mascot_manager.selected_mascot,
                edit_image_with_space.into(),
                InactiveTab,
                Some(100.into()),
            )
            .height(50.0)
            .on_press(HealthTab(HealthMessage::SwitchEditMode));

            edit_buttons_row = edit_buttons_row.push(edit_mode_button)
        }

        let circle_widget = CircleWidget::new(self, CircleStart::Bottom).view();
        let circle_widgets_column = Column::new()
            .push(bmi_widget)
            .push(circle_widget)
            .spacing(INDENT);

        let circle_widgets_with_graph = Row::new()
            .push(circle_widgets_column)
            .push(health_chart_environment_widget(self))
            .spacing(INDENT);

        let mascot_handle = self
            .image_manager
            .cropped_mascot_image_handles
            .get(&self.mascot_manager.selected_mascot)
            .unwrap();
        let mascot_image = image(mascot_handle).width(300);

        let edit_buttons_with_stats = Column::new()
            .push(edit_buttons_row)
            .push(container(health_stat_container()).padding(Padding {
                bottom: LARGE_INDENT,
                ..0.0.into()
            }))
            .spacing(LARGE_INDENT);

        let stat_container_with_mascot = Row::new()
            .push(edit_buttons_with_stats)
            .push(mascot_image)
            .spacing(20.0)
            .align_y(Vertical::Bottom);

        let circles_with_stats_with_graph = Column::new()
            .push(stat_container_with_mascot)
            .push(circle_widgets_with_graph);

        let progress_bars = Column::new()
            .push(create_progress_bar_environment(
                water_progress_bar,
                &Mascot::Rare(Whale),
                edit_mode,
            ))
            .push(create_progress_bar_environment(
                steps_progress_bar,
                &Mascot::Rare(Chameleon),
                edit_mode,
            ))
            .push(create_progress_bar_environment(
                sleep_progress_bar,
                &Mascot::Epic(Capybara),
                edit_mode,
            ))
            .spacing(15.0);

        content = content
            .push(circles_with_stats_with_graph)
            .push(progress_bars)
            .padding(Padding {
                top: LARGE_INDENT,
                bottom: FRAME_PADDING,
                ..0.0.into()
            })
            .spacing(LARGE_INDENT);

        create_scrollable(
            row![content, Space::new().width(Length::Fill)], //the row is needed for the scrollbar to go to the end of the frame,,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Default,
        )
        .add_vertical_scrollbar(TAB_SCROLLBAR_WIDTH, TAB_SCROLLBAR_PADDING)
        .into()
    }
}
