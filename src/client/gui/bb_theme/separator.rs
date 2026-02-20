use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::widget::container::Style;
use iced::widget::{Container, Space, container};
use iced_core::{Background, Length};

pub fn separator_line<'a, Msg>(mascot: &'a Mascot, height: impl Into<Length>) -> Container<'a, Msg>
where
    Msg: Clone + 'a,
{
    container(Space::new(0, 0))
        .style(|_theme| Style {
            text_color: None,
            background: Some(Background::Color(mascot.get_primary_color())),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(height)
}
