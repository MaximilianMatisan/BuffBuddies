use crate::client::backend::mascot_mod::epic_mascot::EpicMascot;
use crate::client::backend::mascot_mod::mascot::{Mascot, MascotRarity};
use crate::client::backend::mascot_mod::mascot_manager::MascotManager;
use crate::client::backend::mascot_mod::mascot_trait::MascotTrait;
use crate::client::backend::mascot_mod::rare_mascot::RareMascot;
use crate::client::gui::bb_theme::color::{HIGHLIGHTED_CONTAINER_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::custom_button::{
    ButtonStyle, create_element_button, create_text_button,
};
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_description_text};
use crate::client::gui::bb_widget::shop;
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::image;
use iced::widget::scrollable::{Direction, Rail, Scrollbar, Scroller, Style};
use iced::widget::{Column, Row, Scrollable, container, row, text};
use iced_core::Length::{Fill, FillPortion};
use iced_core::image::{Handle, Image};
use iced_core::{Border, Color, Shadow, Theme};
use strum::IntoEnumIterator;

impl UserInterface {
    pub fn mascot_screen(&self) -> Element<Message> {
        let current_mascot_image: Element<Message> = view_active_mascot(&self.app.mascot_manager);

        let current_mascot_text: Element<Message> =
            text(self.app.mascot_manager.selected_mascot.get_name())
                .font(FIRA_SANS_EXTRABOLD)
                .color(TEXT_COLOR)
                .size(30)
                .width(Fill)
                .center()
                .into();

        let current_mascot: Element<Message> = Column::new()
            .push(current_mascot_image)
            .push(current_mascot_text)
            .width(FillPortion(1))
            .padding(10)
            .spacing(10)
            .into();

        let my_pet_text: Element<Message> = text("My Pets")
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR)
            .size(30)
            .width(Fill)
            .center()
            .into();

        let mut pet_selection: Column<Message> = Column::new().spacing(10);

        for rare_mascot in RareMascot::iter() {
            let mascot: Mascot = rare_mascot.into();
            pet_selection =
                pet_selection.push(create_mascot_button(&self.app.mascot_manager, mascot))
        }

        for epic_mascot in EpicMascot::iter() {
            let mascot: Mascot = epic_mascot.into();
            pet_selection =
                pet_selection.push(create_mascot_button(&self.app.mascot_manager, mascot))
        }

        let scroll: Element<Message> = Scrollable::new(pet_selection)
            .direction(Direction::Vertical(Scrollbar::new().scroller_width(6)))
            .style(
                |_theme: &Theme, _status: iced::widget::scrollable::Status| Style {
                    container: container::Style {
                        text_color: None,
                        background: None,
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 5.0,
                            radius: 15.into(),
                        },
                        shadow: Shadow::default(),
                    },
                    vertical_rail: Rail {
                        background: None,
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 5.0,
                            radius: 15.into(),
                        },
                        scroller: Scroller {
                            color: HIGHLIGHTED_CONTAINER_COLOR,
                            border: Border {
                                color: Color::TRANSPARENT,
                                width: 5.0,
                                radius: 15.into(),
                            },
                        },
                    },
                    horizontal_rail: Rail {
                        background: None,
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 5.0,
                            radius: 15.into(),
                        },
                        scroller: Scroller {
                            color: HIGHLIGHTED_CONTAINER_COLOR,
                            border: Border {
                                color: Color::TRANSPARENT,
                                width: 5.0,
                                radius: 15.into(),
                            },
                        },
                    },
                    gap: None,
                },
            )
            .into();

        let my_mascots: Element<Message> = Column::new()
            .spacing(10)
            .push(my_pet_text)
            .push(scroll)
            .width(FillPortion(1))
            .into();

        let top_half = Row::new()
            .push(current_mascot)
            .push(my_mascots)
            .height(FillPortion(3));

        let shop_text: Element<Message> = text("Shop")
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR)
            .size(30)
            .width(Fill)
            .center()
            .into();

        let shop_widgets: Element<Message> = row![
            shop::ShopWidget::new(
                "Random rare pet-egg".to_string(),
                50,
                self.app.mascot_manager.selected_mascot,
                Message::BuyMascot(MascotRarity::Rare),
            )
            .set_image(Image::new(Handle::from_path(
                "assets/images/rare_gacha.png"
            ))),
            shop::ShopWidget::new(
                "Random epic pet-egg".to_string(),
                100,
                self.app.mascot_manager.selected_mascot,
                Message::BuyMascot(MascotRarity::Epic),
            )
            .set_image(Image::new(Handle::from_path(
                "assets/images/epic_gacha.png"
            ))),
        ]
        .spacing(30)
        .padding(20)
        .into();

        let shop_widget_container: Element<Message> = container(shop_widgets).center(Fill).into();

        let bottom_column: Element<Message> = Column::new()
            .push(shop_text)
            .push(shop_widget_container)
            .spacing(10)
            .into();

        let bottom_half = container(bottom_column).height(FillPortion(4));

        let combined = Column::new().push(top_half).push(bottom_half);

        container(combined).height(Fill).into()
    }
}

fn view_active_mascot(mascot_manager: &MascotManager) -> Element<Message> {
    let image = image(mascot_manager.selected_mascot.get_file_path())
        .width(Fill)
        .height(Fill);
    image.into()
}

fn mascot_select_box(mascot_manager: &MascotManager, mascot: Mascot) -> Element<'static, Message> {
    let name = mascot.get_name().to_string();

    create_text_button(
        mascot_manager.selected_mascot,
        name,
        ButtonStyle::InactiveTab,
        None,
    )
    .width(Fill)
    .height(44)
    .on_press(Message::SelectMascot(mascot))
    .into()
}

fn mascot_current_box(mascot_manager: &MascotManager, mascot: Mascot) -> Element<'static, Message> {
    let name = mascot.get_name().to_string();

    create_text_button(
        mascot_manager.selected_mascot,
        name,
        ButtonStyle::Active,
        None,
    )
    .height(44)
    .width(Fill)
    .on_press(Message::SelectMascot(mascot))
    .into()
}

fn mascot_locked_box(mascot_manager: &MascotManager) -> Element<'static, Message> {
    create_element_button(
        mascot_manager.selected_mascot,
        format_description_text(text("???"))
            .size(15)
            .center()
            .into(),
        ButtonStyle::InactiveTab,
        None,
    )
    .width(Fill)
    .height(44)
    .into()
}

fn create_mascot_button(mascot_manager: &MascotManager, mascot: Mascot) -> Element<Message> {
    if mascot_manager.owns_mascot(mascot) {
        if mascot_manager.selected_mascot == mascot {
            mascot_current_box(mascot_manager, mascot)
        } else {
            mascot_select_box(mascot_manager, mascot)
        }
    } else {
        mascot_locked_box(mascot_manager)
    }
}
