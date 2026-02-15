use crate::client::gui::app::App;
use crate::client::gui::bb_widget::bmi_calculator;
use crate::client::gui::bb_widget::widget_utils::LARGE_INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{progress_bar, Column};
use crate::client::gui::bb_widget::progress_bar::ProgressBarWidget;

impl App {
    pub fn health_screen(&self) -> Element<Message> {
        let bmi_widget = bmi_calculator::BMIWidget::new(self).view();
        let progress_bar = ProgressBarWidget::new(self).view();

        let content = Column::new()
            .push(bmi_widget)
            .push(progress_bar)
            .padding(LARGE_INDENT)
            .spacing(LARGE_INDENT)
            .into();

        content
    }
}
