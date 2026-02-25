use crate::client::backend::mascot_manager::MascotManager;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::custom_button::{ButtonStyle, create_element_button};
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
use crate::common::mascot_mod::epic_mascot::EpicMascot;
use crate::common::mascot_mod::mascot::{Mascot, MascotRarity};
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::mascot_mod::rare_mascot::RareMascot;
use iced::Element;
use iced::widget::{Column, Row, container, text};
use iced_core::Length::Fill;
use iced_core::alignment::Horizontal;
use iced_core::image::{Handle, Image};
use iced_core::{Length, Padding};
use strum::IntoEnumIterator;

const SCROLLABLE_MASCOTS_HEIGHT: f32 = 500.0;
const MASCOT_IMAGE_HEIGHT: f32 = 360.0;
const PADDING: f32 = 125.0;
const MASCOT_BOX_HEIGHT: u16 = 46;
const MASCOT_BOX_TEXT_SIZE: u16 = 18;
const HEADERS_TEXT_SIZE: u16 = 30;

impl App {
    pub fn mascot_screen(&self) -> Element<Message> {
        let current_mascot_image = self
            .mascot_manager
            .view_active_mascot()
            .width(Fill)
            .height(Length::Fixed(MASCOT_IMAGE_HEIGHT));

        let current_mascot_text: Element<Message> =
            text(self.mascot_manager.selected_mascot.get_name())
                .font(FIRA_SANS_EXTRABOLD)
                .color(TEXT_COLOR)
                .size(35)
                .width(Fill)
                .center()
                .into();

        let current_mascot_with_text: Element<Message> = Column::new()
            .push(current_mascot_image)
            .push(current_mascot_text)
            .spacing(16.0)
            .align_x(Horizontal::Center)
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
            mascot_selection =
                mascot_selection.push(create_mascot_button(&self.mascot_manager, mascot))
        }

        for epic_mascot in EpicMascot::iter() {
            let mascot: Mascot = epic_mascot.into();
            mascot_selection =
                mascot_selection.push(create_mascot_button(&self.mascot_manager, mascot))
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
        .padding(Padding {
            left: PADDING,
            ..0.0.into()
        })
        .into();

        let rare_shop_widget = shop::ShopWidget::new(
            "Random Rare Mascot".to_string(),
            50,
            &self.mascot_manager.selected_mascot,
            Message::BuyMascot(MascotRarity::Rare),
        )
        .set_image(Image::new(Handle::from_path(
            "assets/images/rare_gacha.png",
        )));

        let epic_shop_widget = shop::ShopWidget::new(
            "Random Epic Mascot".to_string(),
            100,
            &self.mascot_manager.selected_mascot,
            Message::BuyMascot(MascotRarity::Epic),
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
            .spacing(20)
            .into();

        let bottom_half = container(bottom_column);

        let mascot_interface = Column::new().push(top_half).push(bottom_half);

        create_scrollable(
            mascot_interface,
            self.mascot_manager.selected_mascot,
            ScrollableStyle::Default,
        )
        .add_vertical_scrollbar(TAB_SCROLLBAR_WIDTH, TAB_SCROLLBAR_PADDING)
        .into()
    }
}

fn mascot_select_box(mascot_manager: &MascotManager, mascot: Mascot) -> Element<'static, Message> {
    let name = mascot.get_name().to_string();

    create_element_button(
        &mascot_manager.selected_mascot,
        format_button_text(text(name.to_string()))
            .size(MASCOT_BOX_TEXT_SIZE)
            .into(),
        ButtonStyle::InactiveTab,
        None,
    )
    .width(Fill)
    .height(MASCOT_BOX_HEIGHT)
    .on_press(Message::SelectMascot(mascot))
    .into()
}

fn mascot_current_box(mascot_manager: &MascotManager, mascot: Mascot) -> Element<'static, Message> {
    let name = mascot.get_name().to_string();

    create_element_button(
        &mascot_manager.selected_mascot,
        format_button_text(text(name.to_string()))
            .size(MASCOT_BOX_TEXT_SIZE)
            .into(),
        ButtonStyle::Active,
        None,
    )
    .height(MASCOT_BOX_HEIGHT)
    .width(Fill)
    .on_press(Message::SelectMascot(mascot))
    .into()
}

fn mascot_locked_box(mascot_manager: &MascotManager) -> Element<'static, Message> {
    create_element_button(
        &mascot_manager.selected_mascot,
        format_description_text(text("???"))
            .size(MASCOT_BOX_TEXT_SIZE)
            .center()
            .into(),
        ButtonStyle::InactiveTab,
        None,
    )
    .width(Fill)
    .height(MASCOT_BOX_HEIGHT)
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
