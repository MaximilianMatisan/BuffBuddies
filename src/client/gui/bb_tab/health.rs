use crate::client::backend::widget_state::progress_bar_manager::ProgressBarStateManager;
use crate::client::gui::app::App;
use crate::client::gui::bb_tab::health::HealthTabMessage::{
    SaveProgressBarChanges, SwitchEditMode,
};
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_theme::custom_button::ButtonStyle::InactiveTab;
use crate::client::gui::bb_theme::custom_button::{
    ButtonStyle, create_element_button, create_text_button,
};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::bmi_calculator;
use crate::client::gui::bb_widget::progress_bar::{
    ProgressBarType, ProgressBarWidget, create_progress_bar_environment,
};
use crate::client::gui::bb_widget::widget_utils::LARGE_INDENT;
use crate::client::gui::user_interface::Message;
use crate::client::gui::user_interface::Message::HealthTab;
use crate::common::mascot_mod::epic_mascot::EpicMascot::Capybara;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::rare_mascot::RareMascot::{Chameleon, Whale};
use iced::widget::{Column, Row, Space, row, text};
use iced::{Element, Task};
use iced_core::Padding;
use iced_core::alignment::Vertical;
use iced_core::image::Handle;

#[derive(Debug, Clone, PartialEq)]
pub enum HealthTabMessage {
    SwitchEditMode,
    SaveProgressBarChanges,
}

impl HealthTabMessage {
    pub fn update_health_tab(health_tab_message: HealthTabMessage, app: &mut App) -> Task<Message> {
        match health_tab_message {
            SaveProgressBarChanges => {
                app.widget_manager.progress_bar_state_manager = app
                    .widget_manager
                    .pending_progress_bar_state_manager
                    .take() //now pending_progress_bar_state_manager is None and I get Option<ProgressBarState>
                    .unwrap() //take() is going to return Option<ProgressBarState> since SaveProgressBarChanges can only be sent in edit_mode so it can't fail

                //MODE IS AUTOMATICALLY SWITCHED AT THIS POINT
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
                        Some(ProgressBarStateManager::copy_states(
                            &app.widget_manager.progress_bar_state_manager,
                        ))
                }
            }
        }

        Task::none()
    }
}

impl App {
    pub fn health_screen(&self) -> Element<Message> {
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
            if let Some(editing_progress_bar_manager) =
                &self.widget_manager.pending_progress_bar_state_manager
            {
                water_progress_bar_state = &editing_progress_bar_manager.water_progress_bar_state;
                steps_progress_bar_state = &editing_progress_bar_manager.steps_progress_bar_state;
                sleep_progress_bar_state = &editing_progress_bar_manager.sleep_progress_bar_state;
            }
        }

        let health_header = text("Health")
            .font(FIRA_SANS_EXTRABOLD)
            .color(color::TEXT_COLOR)
            .size(42);
        let bmi_widget = bmi_calculator::BMIWidget::new(self).view();
        let water_progress_bar =
            ProgressBarWidget::new(water_progress_bar_state, ProgressBarType::Water);

        let steps_progress_bar =
            ProgressBarWidget::new(steps_progress_bar_state, ProgressBarType::Steps);

        let sleep_progress_bar =
            ProgressBarWidget::new(sleep_progress_bar_state, ProgressBarType::Sleep);

        let edit_mode_image =
            iced::widget::image(Handle::from_path("assets/images/pen_tool.png")).width(30.0);
        let edit_image_with_space = Column::new()
            .push(Space::with_height(5.0))
            .push(edit_mode_image);
        let edit_mode_button = create_element_button(
            &self.mascot_manager.selected_mascot,
            edit_image_with_space.into(),
            InactiveTab,
            Some(100.into()),
        )
        .height(50.0)
        .on_press(HealthTab(HealthTabMessage::SwitchEditMode));

        let mut content = Column::new().push(health_header);

        let save_changes_button_content = row![
            iced::widget::image(Handle::from_path("assets/images/check_box.png")),
            text("Save").font(FIRA_SANS_EXTRABOLD).color(TEXT_COLOR)
        ]
        .align_y(Vertical::Center)
        .spacing(10.0);

        if edit_mode {
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

            content = content.push(button_row)
        } else {
            content = content.push(edit_mode_button)
        }

        content
            .push(bmi_widget)
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
            .padding(Padding {
                top: LARGE_INDENT,
                ..0.0.into()
            })
            .spacing(LARGE_INDENT)
            .into()
    }
}
