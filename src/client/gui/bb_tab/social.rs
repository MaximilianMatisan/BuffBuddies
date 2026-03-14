use crate::client::backend::pop_up_manager::PopUpType;
use crate::client::gui::app::App;
use crate::client::gui::bb_tab::tab::{FRAME_PADDING, Tab};
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::scrollable::{
    ScrollableExtension, ScrollableStyle, TAB_SCROLLBAR_PADDING, TAB_SCROLLBAR_WIDTH,
    create_scrollable,
};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::social_elements::{friend_user_button, user_profile_button};
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use crate::client::server_communication::user_communicator::{
    add_foreign_user_as_friend_on_server, remove_foreign_user_as_friend_on_server,
};
use crate::common::user_mod::friend_request::FriendRequest;
use crate::common::user_mod::user::UserType;
use iced::widget::{Column, Row, Space, container, text};
use iced::{Element, Task};
use iced_core::alignment::Horizontal;
use iced_core::{Length, Padding};

impl App {
    pub fn social_screen(&self) -> Element<'_, Message> {
        let friends = self.user_manager.get_friends();
        let non_friend_users = self.user_manager.get_non_friend_users();

        let mut friend_buttons = Row::new().spacing(INDENT).padding(INDENT);

        for friend in &friends {
            friend_buttons = friend_buttons.push(friend_user_button(self, friend))
        }
        let scrollable_friends = create_scrollable(
            friend_buttons,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Default,
        )
        .add_horizontal_scrollbar(6.5, 0.0);

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
            .padding(Padding {
                bottom: FRAME_PADDING,
                ..20.into()
            });
        if !friends.is_empty() {
            content = content.push(friend_container)
        }
        if !non_friend_users.is_empty() {
            content = content.push(user_container)
        }

        let aligned_content = Row::new()
            .push(Space::new().width(Length::Fill))
            .push(content)
            .push(Space::new().width(Length::Fill));

        create_scrollable(
            aligned_content,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Default,
        )
        .add_vertical_scrollbar(TAB_SCROLLBAR_WIDTH, TAB_SCROLLBAR_PADDING)
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum SocialMessage {
    AddUserAsFriend(String),
    RemoveUserAsFriend(String),
    ViewProfile(UserType),
}

impl SocialMessage {
    pub fn update(self, app: &mut App) -> Task<Message> {
        match self {
            SocialMessage::AddUserAsFriend(username) => {
                app.user_manager.add_user_as_friend(&username);
                if let Some(jwt) = app.jsonwebtoken.clone() {
                    return Task::perform(
                        add_foreign_user_as_friend_on_server(
                            jwt,
                            FriendRequest {
                                username: username.clone(),
                            },
                        ),
                        |result| {
                            Message::UpdateInfoOnServerResult(result, "added-Friend".to_string())
                        },
                    );
                } else {
                    app.pop_up_manager.new_pop_up(
                        PopUpType::Minor,
                        "Adding Friend Failed!".to_string(),
                        "Log in to add a friend persistently!".to_string(),
                    );
                }
            }
            SocialMessage::RemoveUserAsFriend(username) => {
                app.user_manager.remove_user_as_friend(&username);
                if let Some(jwt) = app.jsonwebtoken.clone() {
                    return Task::perform(
                        remove_foreign_user_as_friend_on_server(
                            jwt,
                            FriendRequest {
                                username: username.clone(),
                            },
                        ),
                        |result| {
                            Message::UpdateInfoOnServerResult(result, "removed-Friend".to_string())
                        },
                    );
                } else {
                    app.pop_up_manager.new_pop_up(
                        PopUpType::Minor,
                        "Removing Friend Failed!".to_string(),
                        "Log in to remove a friend persistently!".to_string(),
                    );
                }
            }
            SocialMessage::ViewProfile(user_type) => {
                match user_type {
                    UserType::Own => {
                        app.widget_manager.activity_widget.update_data(
                            app.user_manager.user_info.favorite_mascot,
                            app.user_manager
                                .user_info
                                .profile_stat_manager
                                .activity_data
                                .clone(),
                        );
                        app.user_manager.most_recently_viewed_user = UserType::Own
                    }
                    UserType::Other(username) => {
                        let opt_user = app.user_manager.get_user_by_username(&username);
                        if let Some(user) = opt_user {
                            app.widget_manager.activity_widget.update_data(
                                user.user_information.favorite_mascot,
                                user.user_information
                                    .profile_stat_manager
                                    .activity_data
                                    .clone(),
                            );
                        }
                        app.user_manager.most_recently_viewed_user =
                            UserType::Other(username.clone());
                    }
                }
                app.screen = Tab::ViewProfile;
            }
        }
        Task::none()
    }
}
