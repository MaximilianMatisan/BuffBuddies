use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::color::{ERROR_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::container::{ContainerStyle, create_style_container};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_button_text};
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{Column, Space, container, text, text_input};
use iced_core::Length::Fill;
use iced_core::Theme;

pub fn view_login(app: &App) -> Element<'_, Message> {
    let login_text: Element<Message> = text("LOGIN")
        .color(TEXT_COLOR)
        .font(FIRA_SANS_EXTRABOLD)
        .size(30)
        .width(Fill)
        .center()
        .into();

    let error_text: Element<Message> = text(&app.login_state.error_text)
        .font(FIRA_SANS_EXTRABOLD)
        .width(Fill)
        .center()
        .color(ERROR_COLOR)
        .into();

    let username_field: Element<Message> =
        text_input("Enter username...", &app.login_state.username)
            .on_input(Message::UsernameEntered)
            .into();

    let password_field: Element<Message> =
        text_input("Enter password...", &app.login_state.password)
            .on_input(Message::PasswordEntered)
            .on_submit(Message::TryLogin)
            .secure(true)
            .into();

    let login_button_text: Element<Message> = format_button_text(text("Login")).width(Fill).into();
    let login_button = create_element_button(
        app.mascot_manager.selected_mascot,
        login_button_text,
        ButtonStyle::Active,
        None,
    )
    .on_press(Message::TryLogin)
    .width(Fill)
    .height(40);

    let register_button_text: Element<Message> =
        format_button_text(text("Register")).width(Fill).into();
    let register_button = create_element_button(
        app.mascot_manager.selected_mascot,
        register_button_text,
        ButtonStyle::Active,
        None,
    )
    .on_press(Message::TryRegister)
    .width(Fill)
    .height(40);

    let login_elements = Column::new()
        .push(login_text)
        .push(error_text)
        .push(username_field)
        .push(password_field)
        .push(Space::with_height(0))
        .push(login_button)
        .push(register_button)
        .width(Fill)
        .height(Fill)
        .spacing(20);

    let login_container = container(login_elements)
        .padding(20)
        .max_width(400)
        .max_height(420)
        .style(create_style_container(ContainerStyle::Default, None, None))
        .width(Fill)
        .height(Fill);

    container(login_container)
        .width(Fill)
        .height(Fill)
        .style(|_theme: &Theme| container::Style {
            text_color: None,
            background: Some(iced::Background::Color(color::BACKGROUND_COLOR)),
            border: Default::default(),
            shadow: Default::default(),
        })
        .center(Fill)
        .into()
}
