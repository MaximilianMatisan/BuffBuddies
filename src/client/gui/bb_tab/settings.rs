use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::{HIGHLIGHTED_CONTAINER_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::container::{ContainerStyle, create_style_container};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{
    FIRA_SANS_EXTRABOLD, cm_to_string, format_button_text, format_description_text, kg_to_string,
};
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::size;
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::{Column, Container, Row, Space, container, image, text};
use iced_core::image::Handle;
use iced_core::{Length, Padding};

impl UserInterface {
    pub fn settings_screen(&self) -> Element<Message> {
        settings_user_info_preview(&self.app)
    }
}
fn settings_user_info_preview(app: &App) -> Element<Message> {
    let user_info = &app.user_manager.user_info;

    let profile_picture = image(Handle::from_path(user_info.profile_picture_handle.clone()))
        .width(size::LARGE_PROFILE_PICTURE_DIMENSION)
        .height(size::LARGE_PROFILE_PICTURE_DIMENSION);

    //Username should currently not be changeable
    let username = text(user_info.username.clone())
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR)
        .size(40);
    let edit_profile_button = create_element_button(
        app.mascot_manager.selected_mascot,
        image(Handle::from_path("assets/images/edit.png")).into(),
        ButtonStyle::InactiveTransparent,
        None,
    ).on_press(Message::EditProfile);

    let username_and_edit_button = Row::new()
        .push(username)
        .push(edit_profile_button)
        .spacing(INDENT);

    let description_text = if user_info.description.is_empty() {
        format_description_text(text("Tell something about you!"))
    } else {
        text(user_info.description.clone())
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR)
    };
    let description_text_container: Container<Message> = container(description_text)
        .style(create_style_container(
            ContainerStyle::Background,
            None,
            Some(HIGHLIGHTED_CONTAINER_COLOR),
        ))
        .width(Length::FillPortion(10))
        .padding([3.0, INDENT]); //[top/bottom, left/right]

    let description = Row::new()
        .push(format_description_text(text("Description:")))
        .push(Space::with_width(Length::FillPortion(3)))
        .push(description_text_container);

    let user_data_column = Column::new()
        .push(create_user_data_preview(
            "Gender:",
            user_info.gender.to_string(),
        ))
        .push(create_user_data_preview(
            "Weight:",
            kg_to_string(user_info.weight),
        ))
        .push(create_user_data_preview(
            "Height:",
            cm_to_string(user_info.height),
        ))
        .push(create_user_data_preview(
            "Weekly workout goal:",
            user_info.weekly_workout_goal.to_string(),
        ))
        .push(create_user_data_preview(
            "Favorite Mascot:",
            app.mascot_manager.favorite_mascot.to_string(),
        ))
        .push(description)
        .width(Length::FillPortion(15));

    let username_and_data_column = Column::new()
        .push(username_and_edit_button)
        .push(Space::with_height(INDENT))
        .push(user_data_column);

    let contents = Row::new()
        .push(Space::with_width(Length::FillPortion(1)))
        .push(profile_picture)
        .push(Space::with_width(Length::FillPortion(1)))
        .push(username_and_data_column)
        .push(Space::with_width(Length::FillPortion(1)));

    let user_info_container = container(contents)
        .style(create_style_container(ContainerStyle::Default, None, None))
        .height(Length::Shrink)
        .width(Length::FillPortion(10))
        .padding(Padding {
            top: LARGE_INDENT,
            bottom: LARGE_INDENT,
            ..Default::default()
        });

    let user_info_element: Element<Message> = Row::new()
        .push(Space::with_width(Length::FillPortion(1)))
        .push(user_info_container)
        .push(Space::with_width(Length::FillPortion(1)))
        .into();

    user_info_element
}

fn create_user_data_preview(description_text: &str, information_text: String) -> Row<Message> {
    Row::new()
        .push(format_description_text(text(description_text)))
        .push(Space::with_width(Length::Fill))
        .push(format_button_text(text(information_text)))
}
