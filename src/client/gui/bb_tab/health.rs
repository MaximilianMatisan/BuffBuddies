use crate::client::gui::app::App;
use crate::client::gui::bb_widget::bmi_calculator;
use crate::client::gui::bb_widget::widget_utils::LARGE_INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::Column;

impl App {
    pub fn health_screen(&self) -> Element<Message> {
        let bmi_widget = bmi_calculator::BMIWidget::new(self).view();

        let content = Column::new()
            .push(bmi_widget)
            .padding(LARGE_INDENT)
            .spacing(LARGE_INDENT)
            .into();

        content
    }
}
