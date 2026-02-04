use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::social_elements::{friend_user_button, user_profile_button};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, Space, container, text};
use iced_core::Length;
use iced_core::alignment::Horizontal;

impl App {
    pub fn social_screen(&self) -> Element<Message> {
        let friends = self.user_manager.get_friends();
        let non_friend_users = self.user_manager.get_non_friend_users();

        let mut friend_buttons = Row::new().spacing(INDENT).padding(INDENT);

        for friend in &friends {
            friend_buttons = friend_buttons.push(friend_user_button(self, friend))
        }
        let scrollable_friends =
            Scrollable::new(friend_buttons).direction(Direction::Horizontal(Scrollbar::new()));

        let friends_with_title = Column::new()
            .push(
                text("Buddies")
                    .font(FIRA_SANS_EXTRABOLD)
                    .color(TEXT_COLOR)
                    .size(40),
            )
            .push(scrollable_friends)
            .align_x(Horizontal::Center);

        let friend_container = container(friends_with_title)
            .style(create_container_style(ContainerStyle::Default, None, None))
            .width(Length::Shrink)
            .padding(INDENT);

        let mut user_buttons = Column::new().spacing(INDENT).padding(INDENT);

        for user in &non_friend_users {
            user_buttons = user_buttons.push(user_profile_button(
                &self.mascot_manager.selected_mascot,
                user,
            ))
        }

        let user_navigation = Row::new() //TODO add text_input element for search
            .push(
                text("Find other users")
                    .font(FIRA_SANS_EXTRABOLD)
                    .color(TEXT_COLOR)
                    .size(24),
            );

        let user_elements = Column::new().push(user_navigation).push(user_buttons);

        let user_container = container(user_elements)
            .style(create_container_style(ContainerStyle::Default, None, None))
            .padding(INDENT);

        let mut content = Column::new()
            .align_x(Horizontal::Center)
            .spacing(INDENT)
            .padding(INDENT);
        if !friends.is_empty() {
            content = content.push(friend_container)
        }
        if !non_friend_users.is_empty() {
            content = content.push(user_container)
        }

        let aligned_content = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(content)
            .push(Space::with_width(Length::Fill));

        Scrollable::new(aligned_content)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}
