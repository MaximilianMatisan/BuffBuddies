use iced::{Element, Task};
use iced::widget::button;
use iced_core::Size;
use iced_core::window::{Position, Settings};
use crate::client::mascots::Mascot;

mod client;

#[derive(Default)]
struct UserInterface {
    active_mascot: Mascot,
}

#[derive(Debug, Clone, Copy)]
enum Message {

}

impl UserInterface {
    fn update(&mut self, message: Message) -> Task<Message>{
        Task::none()
    }
    fn view(&self) -> Element<'_, Message>{
        let test_button = button("test");

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
