use crate::client::app::App;
use crate::client::bb_tab::tab::Tab;
use crate::client::bb_theme::custom_button::{create_text_button, ButtonStyle};
use crate::client::bb_theme::color;
use crate::client::bb_widget::workout_preset::WorkoutPresetWidget;
use crate::client::size;
use iced::widget::column;
use iced::widget::{container, row, Column};
use iced::{Element, Task};
use iced_core::border::Radius;
use iced_core::image::{Handle, Image};
use iced_core::window::{Position, Settings};
use iced_core::Length::Fill;
use iced_core::{Border, Size, Theme};
use strum::IntoEnumIterator;
use crate::client::bb_widget::shop;

mod client;

#[derive(Default)]
struct UserInterface {
    app: App,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Select(Tab),
    BuyMascot()
}

impl UserInterface {
    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::Select(Tab::Exit) => iced::exit(),
            Message::Select(tab) => {
                self.app.screen = tab;
                Task::none()
            },
            Message::BuyMascot() => {
                self.app.screen = Tab::Settings;
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message>{
        let mut tab_bar: Column<Message> = Column::new();
        for tab in Tab::iter() {
            tab_bar = tab_bar.push(
                create_text_button(self.app.active_mascot.clone(),
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


        let workout_preset: Element<Message> = WorkoutPresetWidget::default().into();

        let mut shop_widgets = row![
            shop::ShopWidget::new("Random rare mascot-egg".to_string(), 50, self.app.active_mascot.clone(), Message::BuyMascot()),
            shop::ShopWidget::new("Random epic mascot-egg".to_string(), 100, self.app.active_mascot.clone(), Message::BuyMascot())
            .set_image(Image::new(Handle::from_path("assets/images/epic_gacha.png")))
        ];

        shop_widgets = shop_widgets.spacing(30).padding(20);

        let contents: Element<Message> = column![workout_preset, shop_widgets].padding(30).into();

        let frame_container = container(row![tab_container, contents])
            .width(size::FRAME_WIDTH)
            .height(size::FRAME_HEIGHT)
            .style(|_theme: &Theme| container::Style{
                text_color: None,
                background: Some(iced::Background::Color(color::BACKGROUND_COLOR)),
                border: Default::default(),
                shadow: Default::default(),
            }).padding(20)
            .into();

        frame_container
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