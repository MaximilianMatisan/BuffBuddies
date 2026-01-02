use crate::client::backend::mascot_mod::epic_mascot::EpicMascot;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::rare_mascot::RareMascot;
use crate::client::gui::bb_widget::progress::progress_environment_widget;
use crate::client::gui::bb_widget::social_elements::{friend_button, user_profile_button};
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
                "assets/images/profile_picture.png",
                "Stefano".to_string(),
                12,
                &Mascot::Rare(RareMascot::Whale),
            ))
            .push(
                friend_button(
                    &self.app,
                    "assets/images/profile_picture.png",
                    "Felix".to_string(),
                    11,
                    &Mascot::Rare(RareMascot::Chameleon),
                )
            )
            .push(
                friend_button(
                    &self.app,
                    "assets/images/profile_picture.png",
                    "Robert".to_string(),
                    12,
                    &Mascot::Epic(EpicMascot::Reindeer),
                )
            )
            .spacing(INDENT)
            .padding(INDENT);

        let user_buttons = Column::new()
            .push(user_profile_button(&self.app.mascot_manager.selected_mascot, "assets/images/profile_picture.png", "BuffUser".to_string(), 8))
            .push(user_profile_button(&self.app.mascot_manager.selected_mascot, "assets/images/profile_picture.png", "JohnP".to_string(), 304))
            .push(user_profile_button(&self.app.mascot_manager.selected_mascot, "assets/images/profile_picture.png", "Dmytro".to_string(), 1052))
            .spacing(INDENT)
            .padding(INDENT);

        let content = Column::new()
            .push(friend_buttons)
            .push(user_buttons)
            .push(progress_widget);

        Scrollable::new(content)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}
