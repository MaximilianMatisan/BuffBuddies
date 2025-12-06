use crate::client::app::App;
use crate::client::bb_theme::custom_button;
use crate::client::bb_theme::custom_button::ButtonStyle;
use iced::{Element, Task};
use iced_core::window::{Position, Settings};
use iced_core::Size;

mod client;

#[derive(Default)]
struct UserInterface {
    app: App,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Test
}

impl UserInterface {
    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::Test => Task::none()
        }
    }
    fn view(&self) -> Element<'_, Message>{
        let test_button =
            custom_button::create_text_button(&self.app, "test", ButtonStyle::InactiveSolid)
                .on_press(Message::Test);
        test_button.into()
    }
}

pub fn main() -> iced::Result {
    let settings: Settings = Settings {
        size: Size::new(client::size::FRAME_WIDTH, client::size::FRAME_HEIGHT),
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
        .run_with(|| (UserInterface::default(), iced::Task::none()))
}
