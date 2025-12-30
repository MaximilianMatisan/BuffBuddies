use crate::client::gui::bb_theme::container::{ContainerStyle, create_style_container};
use crate::client::gui::bb_widget::shop;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::size;
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::{Column, container, row};
use iced_core::Image;
use iced_core::image::Handle;

impl UserInterface {
    pub fn homescreen(&self) -> Element<Message> {
        let activity_widget: Element<Message> = self.app.activity_widget.view(&self.app);

        let shop_widgets = row![
            shop::ShopWidget::new(
                "Random rare mascot_mod-egg".to_string(),
                50,
                self.app.mascot_manager.selected_mascot,
                Message::BuyMascot()
            ),
            shop::ShopWidget::new(
                "Random epic mascot_mod-egg".to_string(),
                100,
                self.app.mascot_manager.selected_mascot,
                Message::BuyMascot()
            )
            .set_image(Image::new(Handle::from_path(
                "assets/images/epic_gacha.png"
            )))
        ]
        .spacing(30);

        let contents = Column::new()
            .push(activity_widget)
            .push(shop_widgets)
            .spacing(INDENT)
            .padding(INDENT);

        let frame_container = container(contents)
            .width(size::FRAME_WIDTH)
            .height(size::FRAME_HEIGHT)
            .style(create_style_container(
                ContainerStyle::Background,
                Some(0.0.into()),
            ))
            .padding(20)
            .into();

        frame_container
    }
}
