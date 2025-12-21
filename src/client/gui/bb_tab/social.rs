use iced::Element;
use iced::widget::{Row, Scrollable, Space};
use iced::widget::scrollable::{Direction, Scrollbar};
use iced_core::Length;
use crate::{Message, UserInterface};
use crate::client::gui::bb_widget::progress::progress_environment_widget;

impl UserInterface {

    pub fn social_screen(&self) -> Element<Message> {
        let content = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(progress_environment_widget(&self.app))
            .push(Space::with_width(Length::Fill));

        Scrollable::new(content)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}