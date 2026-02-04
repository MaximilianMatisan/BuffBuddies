use crate::client::gui::app::App;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::color::{DARK_SHADOW, HIGHLIGHTED_CONTAINER_COLOR};
use crate::client::gui::bb_theme::container::{ContainerStyle, DEFAULT_CONTAINER_RADIUS};
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_text_button};
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::container::Style;
use iced::widget::{Column, container, text};
use iced_core::Length::{Fill, Shrink};
use iced_core::{Border, Shadow, Theme, Vector};

pub fn view_pop_up(app: &App) -> Element<'_, Message> {
    let title: Element<Message> = format_button_text(text(&app.pop_up_manager.title))
        .size(30)
        .center()
        .into();

    let text: Element<Message> = format_button_text(text(&app.pop_up_manager.text))
        .size(15)
        .center()
        .into();

    let ok_button: Element<Message> = create_text_button(
        &app.mascot_manager.selected_mascot,
        "Okay".to_string(),
        ButtonStyle::Active,
        None,
    )
    .on_press(Message::ResetPopUp)
    .into();

    let centered_ok = container(ok_button).center(Fill).height(Shrink);

    let column: Element<Message> = Column::new()
        .push(title)
        .push(text)
        .push(centered_ok)
        .spacing(10)
        .into();

    let pop_up_container = container(column)
        .padding(29)
        .max_width(399)
        .max_height(419)
        .style(bb_theme::container::create_container_style(
            ContainerStyle::Highlighted,
            None,
            None,
        ))
        .style(|_theme: &Theme| Style {
            text_color: None,
            background: Some(iced::Background::Color(HIGHLIGHTED_CONTAINER_COLOR)),
            border: Border {
                color: HIGHLIGHTED_CONTAINER_COLOR,
                width: 1.0,
                radius: DEFAULT_CONTAINER_RADIUS.into(),
            },
            shadow: Shadow {
                color: DARK_SHADOW,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 15.0,
            },
        })
        .width(Fill)
        .height(Shrink);

    container(pop_up_container)
        .width(Fill)
        .height(Shrink)
        .style(|_theme: &Theme| container::Style {
            text_color: None,
            background: None,
            border: Default::default(),
            shadow: Default::default(),
        })
        .center(Fill)
        .into()
}
