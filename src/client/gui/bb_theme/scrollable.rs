use std::cmp::PartialEq;
use iced::{Element, Renderer};
use iced::widget::{scrollable, Scrollable};
use iced::widget::scrollable::{Direction, Rail, Scrollbar, Scroller, Status, Style};
use iced_core::{Border, Color, Theme};
use crate::client::gui::bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR;
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;

pub const SCROLLBAR_PADDING: f32 = 15.0;
pub const TAB_SCROLLABLE_PADDING: f32 = 6.0;
pub const TAB_SCROLLABLE_WIDTH: f32 = 7.0;
pub const WIDGET_SCROLLABLE_WIDTH: f32 = 6.0;

#[derive(Clone)]
pub enum ScrollableStyle{
    Transparent,
    Mascot,
    Default
}
pub fn create_scrollable<'a>(element: impl Into<Element<'a, Message, Theme, Renderer>>,mascot: Mascot, scrollable_style: ScrollableStyle) -> Scrollable<'a, Message> {

    Scrollable::new(element)
        .style(move |theme: &_, status: iced::widget::scrollable::Status| {
            create_style(status,mascot,scrollable_style.clone())
    })
}

pub trait ScrollableExtension {
    fn add_horizontal_scrollbar(self, width: f32, padding: f32) -> Self;
    fn add_vertical_scrollbar(self, width: f32, padding: f32) -> Self;
}

impl <'a> ScrollableExtension for Scrollable<'a,Message> {
     fn add_horizontal_scrollbar(self, width: f32, padding: f32) -> Scrollable<'a, Message> {

        self.direction(Direction::Horizontal(Scrollbar::new().scroller_width(width).margin(padding)))
    }

     fn add_vertical_scrollbar(self, width: f32, padding: f32) -> Scrollable<'a,Message> {

        self.direction(Direction::Vertical(Scrollbar::new().scroller_width(width).margin(padding)))
    }
}

//STYLES

fn transparent_style() ->  Style {
    let scrollable_style = iced::widget::scrollable::Style {
        container: Default::default(),
        vertical_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        horizontal_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        gap: None,
    };

    scrollable_style

}

impl PartialEq<Color> for ScrollableStyle {
    fn eq(&self, other: &Color) -> bool {
        todo!()
    }
}

fn create_style(status: Status, mascot: Mascot, scrollable_style: ScrollableStyle) -> Style {
    let mut result_scrollable_style = transparent_style();

    let mut hovered_color: Color;
    let mut dragged_color: Color;

    match scrollable_style {

        ScrollableStyle::Mascot => {

            hovered_color = mascot.get_primary_color();
            hovered_color.a = 0.4;

            dragged_color = mascot.get_secondary_color();
            dragged_color.a = 0.7;
        }

        ScrollableStyle::Default => {
            hovered_color = HIGHLIGHTED_CONTAINER_COLOR;
            hovered_color.a = 0.4;

            dragged_color = hovered_color; //SAME COLOR BUT HIGHER ALPHA VALUE
            dragged_color.a = 0.7;
        },
        ScrollableStyle::Transparent => {
            hovered_color = Color::TRANSPARENT;
            dragged_color = Color::TRANSPARENT

        }
    }

    let hovered_rail = Rail {
        background: None,
        border: Default::default(),
        scroller: Scroller {
            color: hovered_color,
            border: Border {
                color:Color::TRANSPARENT,
                width: 0.5,
                radius: 6.into()
            }
        }
    };
    let dragged_rail = Rail {
        scroller: Scroller {
            color: dragged_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.1,
                radius: 6.0.into()
            }
        },..hovered_rail
    };

    match status {

        scrollable::Status::Active => {}

        scrollable::Status::Hovered{ .. } => {

            result_scrollable_style.horizontal_rail =  hovered_rail;
            result_scrollable_style.vertical_rail = hovered_rail
        }

        scrollable::Status::Dragged{ .. } => {
            result_scrollable_style.horizontal_rail = dragged_rail;
            result_scrollable_style.vertical_rail = dragged_rail
        }
    };

    result_scrollable_style


}