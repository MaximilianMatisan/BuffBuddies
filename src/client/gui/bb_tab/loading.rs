use iced::{
    Element, Length,
    widget::{Column, container, text},
};

use crate::client::gui::{
    app::App,
    bb_theme::{
        container::{ContainerStyle, create_container_style},
        text_format::format_description_text,
    },
    user_interface::Message,
};


impl App {
    pub fn view_loading_screen(&self) -> Element<'_, Message> {
        let loading_text = format_description_text(text("loading...")).center();
        let contents = Column::new().push(loading_text);

        container(contents)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .style(create_container_style(
                ContainerStyle::Background,
                Some(0.0.into()),
                None,
            ))
            .into()
    }
}
