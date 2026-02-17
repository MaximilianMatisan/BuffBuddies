use crate::client::gui::app::App;
use crate::client::gui::bb_widget::bmi_calculator;
use crate::client::gui::bb_widget::widget_utils::LARGE_INDENT;
use crate::client::gui::user_interface::Message;
use iced::{Element, Task};
use iced::widget::{button, progress_bar, Column};
use crate::client::gui::bb_widget::progress_bar::{create_progress_bar_environment, ProgressBarType, ProgressBarWidget};
use crate::client::gui::user_interface::Message::HealthTab;
use crate::common::mascot_mod::epic_mascot::EpicMascot::Capybara;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::rare_mascot::RareMascot::{Chameleon, Whale};

#[derive(Debug, Clone)]
pub enum HealthTabMessage {
    SwitchEditMode
}

impl HealthTabMessage {
    pub fn update_health_tab(health_tab_message: HealthTabMessage, app: &mut App) -> Task<Message> {
        let current_mode = app.widget_manager.progress_bar_state_manager.edit_mode;
        app.widget_manager.progress_bar_state_manager.edit_mode = !current_mode;
        Task::none()
    }
}

impl App {
    pub fn health_screen(&self) -> Element<Message> {
        let bmi_widget = bmi_calculator::BMIWidget::new(self).view();
        let water_progress_bar = ProgressBarWidget::new(
                &self.widget_manager.progress_bar_state_manager.water_progress_bar_state,
                ProgressBarType::Water);

        let steps_progress_bar = ProgressBarWidget::new(
            &self.widget_manager.progress_bar_state_manager.steps_progress_bar_state,
            ProgressBarType::Steps);

        let sleep_progress_bar = ProgressBarWidget::new(
            &self.widget_manager.progress_bar_state_manager.sleep_progress_bar_state,
            ProgressBarType::Sleep);

        let edit_mode_test_button = button("Edit mode").on_press(HealthTab(HealthTabMessage::SwitchEditMode));

        let content = Column::new()
            .push(edit_mode_test_button)
            .push(bmi_widget)
            .push(create_progress_bar_environment(water_progress_bar,&Mascot::Rare(Whale),self.widget_manager.progress_bar_state_manager.edit_mode))
            .push(create_progress_bar_environment(steps_progress_bar,&Mascot::Rare(Chameleon),self.widget_manager.progress_bar_state_manager.edit_mode))
            .push(create_progress_bar_environment(sleep_progress_bar,&Mascot::Epic(Capybara),self.widget_manager.progress_bar_state_manager.edit_mode))
            .padding(LARGE_INDENT)
            .spacing(LARGE_INDENT)
            .into();

        content
    }
}
