use iced::Element;
use iced::widget::{container, row, Column};
use iced_core::Image;
use iced_core::image::Handle;
use crate::client::gui::bb_widget::progress::progress_environment_widget;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::bb_widget::workout_preset::WorkoutPresetWidget;
use crate::client::gui::{bb_theme, size};
use crate::client::gui::bb_theme::container::ContainerStyle;
use crate::{Message, UserInterface};
use crate::client::gui::bb_widget::shop;

impl UserInterface {
    pub fn homescreen(&self) -> Element<Message> {
        let workout_preset: Element<Message> = WorkoutPresetWidget::default().into();

        let activity_widget: Element<Message> = self.app.activity_widget.view(&self.app);

        let mut shop_widgets = row![
                shop::ShopWidget::new("Random rare mascot-egg".to_string(), 50, self.app.active_mascot.clone(), Message::BuyMascot()),
                shop::ShopWidget::new("Random epic mascot-egg".to_string(), 100, self.app.active_mascot.clone(), Message::BuyMascot())
                .set_image(Image::new(Handle::from_path("assets/images/epic_gacha.png")))
            ];

        shop_widgets = shop_widgets.spacing(30);

        let contents = Column::new()
            .push(activity_widget)
            //.push(shop_widgets)
            .push(workout_preset)
            .spacing(INDENT).padding(INDENT);

        let frame_container = container(contents)
            .width(size::FRAME_WIDTH)
            .height(size::FRAME_HEIGHT)
            .style(bb_theme::container::create_style_container(ContainerStyle::Background)).padding(20)
            .into();

        frame_container
    }
}