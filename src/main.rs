use crate::client::app::App;
use crate::client::bb_tab::tab::Tab;
use crate::client::bb_theme::color;
use crate::client::bb_theme::custom_button::{create_text_button, ButtonStyle};
use crate::client::size;
use iced::widget::{container, row, Column};
use iced::{Element, Task};
use iced_core::border::Radius;
use iced_core::window::{Position, Settings};
use iced_core::Length::Fill;
use iced_core::{Border, Size, Theme};
use strum::IntoEnumIterator;

mod client;

#[derive(Default)]
struct UserInterface {
    app: App,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Select(Tab)
}

impl UserInterface {
    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::Select(Tab::Exit) => iced::exit(),
            Message::Select(_) => Task::none()
        }
    }
    fn view(&self) -> Element<'_, Message>{
        let mut tab_bar: Column<Message> = Column::new();
        for tab in Tab::iter() {
            tab_bar = tab_bar.push(
                create_text_button(&self.app,
                                   tab.to_string(),
                                   if self.app.screen == tab
                                   { ButtonStyle::ActiveTab }
                                   else { ButtonStyle::InactiveTab })
                    .on_press(Message::Select(tab))
            );
        }
        let tab_container = container(tab_bar.spacing(10).padding(30))
            .padding(10)
            .style(|_theme: &Theme| container::Style {
                text_color: None,
                background: Some(iced::Background::Color(color::CONTAINER_COLOR)),
                border: Border {
                    color: color::DARKER_CONTAINER_COLOR,
                    width: 1.0,
                    radius: Radius::new(15.0),
                },
                shadow: Default::default(),
            })
            .height(Fill);
        container(row![tab_container])
            .width(size::FRAME_WIDTH)
            .height(size::FRAME_HEIGHT)
            .style(|_theme: &Theme| container::Style{
                text_color: None,
                background: Some(iced::Background::Color(color::BACKGROUND_COLOR)),
                border: Default::default(),
                shadow: Default::default(),
            }).padding(20)
            .into()
    }
}

pub fn main() -> iced::Result {
    let settings: Settings = Settings {
        size: Size::new(size::FRAME_WIDTH, size::FRAME_HEIGHT),
        position: Position::Default,
        min_size: None,
        max_size: None,
        visible: true,
        resizable: false,

        decorations: true,
        transparent: true,
        level: Default::default(),
        icon: None,
        platform_specific: Default::default(),
        exit_on_close_request: true,
    };
    iced::application("BuffBuddies", UserInterface::update, UserInterface::view)
        .window(settings)
        .run_with(|| (UserInterface::default(), Task::none()))
}
