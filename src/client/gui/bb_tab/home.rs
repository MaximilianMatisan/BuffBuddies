use crate::client::gui::bb_widget::progress::progress_environment_widget;
use crate::client::gui::bb_widget::shop;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::{Message, UserInterface};
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, Space, row};
use iced_core::image::Handle;
use iced_core::{Image, Length};

impl UserInterface {
    pub fn homescreen(&self) -> Element<Message> {
        let activity_widget: Element<Message> = self.app.activity_widget.view(&self.app);

        let progress_widget = Row::new()
            .push(Space::with_width(Length::Fill))
            .push(progress_environment_widget(&self.app))
            .push(Space::with_width(Length::Fill));

        let shop_widgets = row![
            shop::ShopWidget::new(
                "Random rare mascot-egg".to_string(),
                50,
                self.app.mascot_manager.selected_mascot,
                Message::BuyMascot()
            ),
            shop::ShopWidget::new(
                "Random epic mascot-egg".to_string(),
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
            .push(progress_widget)
            .push(shop_widgets)
            .spacing(INDENT)
            .padding(INDENT);

        Scrollable::new(contents)
            .direction(Direction::Vertical(Scrollbar::new()))
            .into()
    }
}
