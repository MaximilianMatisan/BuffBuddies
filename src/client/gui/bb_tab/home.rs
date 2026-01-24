use crate::client::gui::bb_widget::progress::progress_environment_widget;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, Space};
use iced_core::Length;
use crate::client::gui::bb_widget::graph::{graph_environment_widget, GraphWidget};

impl UserInterface {
    pub fn homescreen(&self) -> Element<Message> {
        let activity_widget: Element<Message> = self.app.activity_widget.view(&self.app);

        let progress_widget = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(progress_environment_widget(&self.app))
            .push(Space::with_width(Length::Fill));

        let debug_test = GraphWidget::new(&self.app).view();
        let actual_version = graph_environment_widget(&self.app);

        let contents = Column::new()
            .push(activity_widget)
            .push(progress_widget)
            .push(actual_version)
            .spacing(INDENT)
            .padding(INDENT);

        Scrollable::new(contents)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}
