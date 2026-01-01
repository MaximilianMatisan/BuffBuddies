use crate::client::backend::mascot_mod::epic_mascot::EpicMascot;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::rare_mascot::RareMascot;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::progress::progress_environment_widget;
use crate::client::gui::bb_widget::social_elements::friend_button;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, Space};
use iced_core::Length;

impl UserInterface {
    pub fn social_screen(&self) -> Element<Message> {
        let progress_widget = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(progress_environment_widget(&self.app))
            .push(Space::with_width(Length::Fill));

        let friend_buttons = Row::new()
            .push(friend_button(
                &self.app,
                "assets/images/profile_picture.png".to_string(),
                "Stefano".to_string(),
                12,
                &Mascot::Rare(RareMascot::Whale),
            ))
            .push(
                friend_button(
                    &self.app,
                    "assets/images/profile_picture.png".to_string(),
                    "Felix".to_string(),
                    11,
                    &Mascot::Rare(RareMascot::Chameleon),
                )
                .on_press(Message::Select(Tab::Home)),
            )
            .push(
                friend_button(
                    &self.app,
                    "assets/images/profile_picture.png".to_string(),
                    "BuffUserWithBuffMascots".to_string(),
                    1102,
                    &Mascot::Epic(EpicMascot::Reindeer),
                )
                .on_press(Message::Select(Tab::Home)),
            )
            .spacing(INDENT)
            .padding(INDENT);

        let content = Column::new()
            .push(friend_buttons)
            .push(progress_widget)
            .spacing(20);
        Scrollable::new(content)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}
