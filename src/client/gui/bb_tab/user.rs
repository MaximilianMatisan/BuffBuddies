use iced::Element;
use iced::widget::{image, row, column, Column, text, Row, container, Container, Space};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::Length;
use crate::client::backend::user_mod::user::User;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::{HIGHLIGHTED_CONTAINER_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::container::{create_style_container, ContainerStyle};
use crate::client::gui::bb_theme::text_format::{format_description_text, FIRA_SANS_EXTRABOLD};
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::size::LARGE_PROFILE_PICTURE_DIMENSION;
use crate::client::gui::user_interface::Message;

pub fn view_profile<'a>(app: &'a App, user: &User) -> Element<'a, Message> {
    let profile_picture = container(
        image(user.profile_picture_handle.clone())
            .width(Length::Shrink)
            .height(Length::Fill)
    ).padding([0,40]);

    let username: Element<Message>= text(user.username.clone())
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR)
        .size(40)
        .into();

    let description_container: Container<Message> =
        container(format_description_text(text(user.description.clone())))
            .style(create_style_container(ContainerStyle::Background, None, Some(HIGHLIGHTED_CONTAINER_COLOR)))
            .width(Length::FillPortion(10))
            .height(Length::Fill)
            .padding(5);

    let description_element: Row<Message> = row![
        format_description_text(text("Description: ")),
        Space::with_width(Length::FillPortion(1)),
        description_container
    ];

    let username_and_description: Column<Message> = column![
        username,
        description_element
    ]
        .height(Length::Fill)
        .spacing(INDENT);

    let header: Container<Message> = container(row![
        profile_picture,
        username_and_description
    ].align_y(Vertical::Center))
        .width(Length::Fill)
        .height(LARGE_PROFILE_PICTURE_DIMENSION)
        .into();
    
    let activity_widget = app.activity_widget.view(app);

    let content = column![
        header,
        activity_widget,
    ]
        .align_x(Horizontal::Center)
        .spacing(LARGE_INDENT);

    content.into()
}
