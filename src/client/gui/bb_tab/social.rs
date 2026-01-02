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

        let friends = self.app.user_manager.get_friends();
        let non_friend_users = self.app.user_manager.get_non_friend_users();
        
        let mut friend_buttons = Row::new().spacing(INDENT).padding(INDENT);
        
        for friend in friends {
            friend_buttons = friend_buttons.push(friend_button(&self.app, friend))
        }
        
        let mut user_buttons = Column::new().spacing(INDENT).padding(INDENT);
        
        for user in non_friend_users {
            user_buttons = user_buttons.push(user_profile_button(&self.app.mascot_manager.selected_mascot, user))
        }

        let content = Column::new()
            .push(friend_buttons)
            .push(user_buttons)
            .push(progress_widget);

        Scrollable::new(content)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}
