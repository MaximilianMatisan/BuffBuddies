use crate::client::gui::app::App;
use crate::client::gui::bb_widget::bmi_calculator;
use crate::client::gui::bb_widget::widget_utils::LARGE_INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{progress_bar, Column};
use crate::client::gui::bb_widget::progress_bar::{ProgressBarType, ProgressBarWidget};

impl App {
    pub fn health_screen(&self) -> Element<Message> {
        let bmi_widget = bmi_calculator::BMIWidget::new(self).view();
        let water_progress_bar = ProgressBarWidget::new(
                &self.widget_manager.progress_bar_state_manager.water_progress_bar_state,
                ProgressBarType::Water)
            .view();

        let steps_progress_bar = ProgressBarWidget::new(
            &self.widget_manager.progress_bar_state_manager.steps_progress_bar_state,
            ProgressBarType::Steps)
            .view();

        let sleep_progress_bar = ProgressBarWidget::new(
            &self.widget_manager.progress_bar_state_manager.sleep_progress_bar_state,
            ProgressBarType::Sleep)
            .view();

        let content = Column::new()
            .push(bmi_widget)
            .push(water_progress_bar)
            .push(steps_progress_bar)
            .push(sleep_progress_bar)
            .padding(LARGE_INDENT)
            .spacing(LARGE_INDENT)
            .into();

        content
    }
}
