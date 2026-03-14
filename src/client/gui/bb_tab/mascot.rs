use crate::client::backend::pop_up_manager::PopUpType;
use crate::client::gui::app::App;
use crate::client::gui::bb_tab::tab::FRAME_PADDING;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::custom_button::ButtonStyle::{Active, InactiveTab, Rainbow};
use crate::client::gui::bb_theme::custom_button::create_element_button;
use crate::client::gui::bb_theme::scrollable::{
    ScrollableExtension, ScrollableStyle, TAB_SCROLLBAR_PADDING, TAB_SCROLLBAR_WIDTH,
    create_scrollable,
};
use crate::client::gui::bb_theme::text_format::{
    FIRA_SANS_EXTRABOLD, format_button_text, format_description_text,
};
use crate::client::gui::bb_widget::shop;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use crate::client::server_communication::mascot_communicator::{
    buy_mascot, update_selected_mascot_on_server,
};
use crate::client::server_communication::server_communicator::ServerRequestError;
use crate::common::mascot_mod::epic_mascot::EpicMascot;
use crate::common::mascot_mod::mascot::{Mascot, MascotRarity};
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::mascot_mod::rare_mascot::RareMascot;
use iced::widget::{Column, Row, Space, container, image, row, text};
use iced::{Element, Task};
use iced_core::Length::Fill;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::image::{Handle, Image};
use iced_core::{Length, Padding};
use strum::IntoEnumIterator;

const SCROLLABLE_MASCOTS_HEIGHT: f32 = 540.0;
const MASCOT_IMAGE_HEIGHT: f32 = 360.0;
const MASCOT_BOX_HEIGHT: f32 = 46.0;
const MASCOT_BOX_TEXT_SIZE: f32 = 18.0;
const BOX_PADDING: f32 = 12.5;
const HEADERS_TEXT_SIZE: f32 = 30.0;

#[derive(Clone, Debug)]
pub enum MascotMessage {
    BuyMascot(MascotRarity),
    SaveMascot(Result<Mascot, ServerRequestError>),
    SelectMascot(Mascot),
}

impl MascotMessage {
    pub fn update(&self, app: &mut App) -> Task<Message> {
        match self {
            MascotMessage::BuyMascot(rarity) => {
                if match rarity {
                    MascotRarity::Rare => app.user_manager.user_info.coin_balance >= 50,
                    MascotRarity::Epic => app.user_manager.user_info.coin_balance >= 100,
                } {
                    let mut mascot_maybe: Option<Mascot> = None;
                    match rarity {
                        MascotRarity::Rare => {
                            match RareMascot::random_new_rare(&app.mascot_manager) {
                                Ok(mascot) => mascot_maybe = Some(mascot.into()),
                                Err(_err) => app.pop_up_manager.new_pop_up(
                                    PopUpType::Minor,
                                    "Failed to buy mascot!".to_string(),
                                    "All mascots of this rarity have already been purchased!"
                                        .to_string(),
                                ),
                            }
                        }
                        MascotRarity::Epic => {
                            match EpicMascot::random_new_epic(&app.mascot_manager) {
                                Ok(mascot) => mascot_maybe = Some(mascot.into()),
                                Err(_err) => app.pop_up_manager.new_pop_up(
                                    PopUpType::Minor,
                                    "Failed to buy mascot!".to_string(),
                                    "All mascots of this rarity have already been purchased!"
                                        .to_string(),
                                ),
                            }
                        }
                    };
                    if let Some(mascot) = mascot_maybe {
                        if let Some(jwt) = &app.jsonwebtoken {
                            Task::perform(buy_mascot(jwt.clone(), mascot), |result| {
                                Message::Mascot(MascotMessage::SaveMascot(result))
                            })
                        } else {
                            app.pop_up_manager.new_pop_up(
                                PopUpType::Minor,
                                "Buying Mascot failed!".to_string(),
                                "Log in to buy mascots!".to_string(),
                            );
                            Task::none()
                        }
                    } else {
                        Task::none()
                    }
                } else {
                    app.pop_up_manager.new_pop_up(
                        PopUpType::Minor,
                        "Funds lacking!".to_string(),
                        "You do not have enough money to buy a mascot of this type".to_string(),
                    );
                    Task::none()
                }
            }
            MascotMessage::SaveMascot(Ok(mascot)) => {
                app.user_manager.user_info.coin_balance -= mascot.get_prize();
                app.mascot_manager.add_mascot(*mascot);
                Task::none()
            }
            MascotMessage::SaveMascot(Err(_err)) => {
                app.pop_up_manager.new_pop_up(
                    PopUpType::Minor,
                    "Server error!".to_string(),
                    "Server is either offline or had an internal error!\nPlease start server or report bug".to_string(),
                );
                Task::none()
            }
            MascotMessage::SelectMascot(mascot) => {
                let active_mascot = &mut app.mascot_manager.selected_mascot;
                *active_mascot = *mascot;
                app.widget_manager
                    .activity_widget
                    .update_active_mascot(*active_mascot);

                if let Some(jwt) = app.jsonwebtoken.clone() {
                    Task::perform(update_selected_mascot_on_server(jwt, *mascot), |result| {
                        Message::UpdateInfoOnServerResult(result, "selected Mascot".to_string())
                    })
                } else {
                    println!("Log in to select a Mascot!");
                    Task::none()
                }
            }
        }
    }
}

impl App {
    pub fn mascot_screen(&self) -> Element<'_, Message> {
        let current_mascot_image = self
            .mascot_manager
            .view_active_mascot()
            .width(Fill)
            .height(MASCOT_IMAGE_HEIGHT);

        let current_mascot_text: Element<Message> =
            text(self.mascot_manager.selected_mascot.get_name())
                .font(FIRA_SANS_EXTRABOLD)
                .color(TEXT_COLOR)
                .size(35)
                .width(Fill)
                .center()
                .into();

        let randomize_text = format_button_text(text("Randomize")).size(18);

        let dice_image = image(Handle::from_path("assets/images/dice.png"))
            .width(23)
            .height(23);

        let randomize_text_with_image = container(row![
            dice_image,
            Space::new().width(Length::Fixed(5.0)),
            randomize_text
        ])
        .align_y(Vertical::Center)
        .center_x(Fill);

        let randomize_button = create_element_button(
            &self.mascot_manager.selected_mascot,
            randomize_text_with_image.into(),
            Rainbow,
            Some(7.5.into()),
        )
        .on_press(Message::Mascot(MascotMessage::SelectMascot(
            self.mascot_manager.get_random_owned_mascot(),
        )))
        .height(37.0)
        .width(210.0);

        let current_mascot_with_text: Element<Message> = Column::new()
            .push(current_mascot_image)
            .push(current_mascot_text)
            .push(randomize_button)
            .spacing(16.0)
            .align_x(Horizontal::Center) //align the randomize_button
            .into();

        let my_mascots_text: Element<Message> = text("My mascots")
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR)
            .size(HEADERS_TEXT_SIZE)
            .width(Fill)
            .into();

        let mut mascot_selection: Column<Message> =
            Column::new().spacing(INDENT).padding(Padding {
                right: 15.0,
                ..0.0.into()
            });

        for rare_mascot in RareMascot::iter() {
            let mascot: Mascot = rare_mascot.into();
            mascot_selection = mascot_selection.push(create_mascot_button(self, mascot))
        }

        for epic_mascot in EpicMascot::iter() {
            let mascot: Mascot = epic_mascot.into();
            mascot_selection = mascot_selection.push(create_mascot_button(self, mascot))
        }

        let scrollable_mascot_selection: Element<Message> = create_scrollable(
            mascot_selection,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Transparent,
        )
        .into();

        let title_with_selection: Element<Message> = Column::new()
            .push(my_mascots_text)
            .push(scrollable_mascot_selection)
            .spacing(15.0)
            .into();

        let top_half = Row::new()
            .push(current_mascot_with_text)
            .push(title_with_selection)
            .height(SCROLLABLE_MASCOTS_HEIGHT)
            .padding(Padding {
                top: 42.5,
                right: 30.0,
                ..0.0.into()
            });

        let shop_text: Element<Message> = container(
            text("Shop")
                .font(FIRA_SANS_EXTRABOLD)
                .color(TEXT_COLOR)
                .size(HEADERS_TEXT_SIZE),
        )
        .into();

        let rare_shop_widget = shop::ShopWidget::new(
            "Random Rare Mascot".to_string(),
            50,
            &self.mascot_manager.selected_mascot,
            Message::Mascot(MascotMessage::BuyMascot(MascotRarity::Rare)),
        )
        .set_image(Image::new(Handle::from_path(
            "assets/images/rare_gacha.png",
        )));

        let epic_shop_widget = shop::ShopWidget::new(
            "Random Epic Mascot".to_string(),
            100,
            &self.mascot_manager.selected_mascot,
            Message::Mascot(MascotMessage::BuyMascot(MascotRarity::Epic)),
        )
        .set_image(Image::new(Handle::from_path(
            "assets/images/epic_gacha.png",
        )));

        let shop_widgets: Element<Message> = Row::new()
            .push(rare_shop_widget)
            .push(epic_shop_widget)
            .spacing(60)
            .into();

        let shop_widget_container: Element<Message> = container(shop_widgets)
            .width(Fill)
            .align_x(Horizontal::Center)
            .into();

        let bottom_column: Element<Message> = Column::new()
            .push(shop_text)
            .push(shop_widget_container)
            .spacing(30)
            .align_x(Horizontal::Center)
            .into();

        let bottom_half = container(bottom_column);

        let mascot_interface = Column::new()
            .push(top_half)
            .push(bottom_half)
            .spacing(20.0)
            .padding(Padding {
                bottom: FRAME_PADDING,
                ..0.into()
            });

        create_scrollable(
            mascot_interface,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Default,
        )
        .add_vertical_scrollbar(TAB_SCROLLBAR_WIDTH, TAB_SCROLLBAR_PADDING)
        .into()
    }
}

#[derive(PartialEq)]
enum BoxType {
    Selectable,
    Current,
    Locked,
}

fn create_mascot_box(app: &App, mascot: Mascot, box_type: BoxType) -> Element<'static, Message> {
    let mut content = Row::new()
        .push(Space::new().width(Length::Fixed(BOX_PADDING)))
        .spacing(INDENT)
        .align_y(Vertical::Center)
        .height(Fill);

    match box_type {
        BoxType::Locked => {
            content = content.push(
                format_description_text(text("???"))
                    .size(MASCOT_BOX_TEXT_SIZE)
                    .center(),
            );
        }

        _ => {
            let name = mascot.get_name().to_string();
            let mascot_handle = app
                .image_manager
                .cropped_mascot_head_handles
                .get(&mascot)
                .unwrap();

            let mascot_head_image = image(mascot_handle);

            content = content
                .push(mascot_head_image)
                .push(format_button_text(text(name.to_string())).size(MASCOT_BOX_TEXT_SIZE))
        }
    }

    let button_style = match box_type {
        BoxType::Selectable => InactiveTab,
        BoxType::Current => Active,
        BoxType::Locked => InactiveTab,
    };

    let mut mascot_box = create_element_button(
        &app.mascot_manager.selected_mascot,
        content.into(),
        button_style,
        None,
    )
    .height(MASCOT_BOX_HEIGHT)
    .width(Fill);

    if box_type != BoxType::Locked {
        mascot_box = mascot_box.on_press(Message::Mascot(MascotMessage::SelectMascot(mascot)))
    }

    mascot_box.into()
}

fn create_mascot_button(app: &App, mascot: Mascot) -> Element<'_, Message> {
    if app.mascot_manager.owns_mascot(mascot) {
        if app.mascot_manager.selected_mascot == mascot {
            create_mascot_box(app, mascot, BoxType::Current)
        } else {
            create_mascot_box(app, mascot, BoxType::Selectable)
        }
    } else {
        create_mascot_box(app, mascot, BoxType::Locked)
    }
}
