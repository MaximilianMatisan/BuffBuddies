use crate::client::gui::app::App;
use crate::client::gui::bb_widget::graph::graph_environment_widget;
use crate::client::gui::bb_widget::progress::progress_environment_widget;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, Space};
use iced_core::Length;

impl App {
    pub fn homescreen(&self) -> Element<Message> {
        let activity_widget: Element<Message> = self.activity_widget.view(self);

        let progress_widget = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(progress_environment_widget(self))
            .push(Space::with_width(Length::Fill));

        let actual_version = graph_environment_widget(self);

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
