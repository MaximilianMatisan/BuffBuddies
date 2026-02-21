use iced::widget::scrollable;
use iced::widget::scrollable::{Rail, Scroller, Status, Style};
use iced_core::{Border, Color};
use crate::client::gui::bb_theme::color::HIGHLIGHTED_CONTAINER_COLOR;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;

pub fn mascot_style(status: Status, mascot: Mascot) ->  Style {

    let mut scrollable_style = iced::widget::scrollable::Style {
        container: Default::default(),
        vertical_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        horizontal_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        gap: None,
    };

    let mut hovered_color = mascot.get_primary_color();
    hovered_color.a = 0.4;
    let mut dragged_color = mascot.get_secondary_color();
    dragged_color.a = 0.7;

    match status {

        scrollable::Status::Active => {
            scrollable_style
        }
        scrollable::Status::Hovered{ .. } => {
            scrollable_style.horizontal_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: hovered_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.5,
                        radius: 6.into()
                    },
                }
            };

            scrollable_style.vertical_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: hovered_color,
                    border: Border {
                        color:Color::TRANSPARENT,
                        width: 0.5,
                        radius: 6.into()
                    },
                }
            };
            scrollable_style
        }
        scrollable::Status::Dragged{ .. } => {
            scrollable_style.horizontal_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color:dragged_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.1,
                        radius: 6.into()
                    },
                }
            };
            scrollable_style.vertical_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: dragged_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.1,
                        radius: 6.into()
                    },
                }
            };

            scrollable_style
        }
    }
}

pub fn main_style (status: Status, mascot: Mascot) ->  Style {

    let mut scrollable_style = iced::widget::scrollable::Style {
        container: Default::default(),
        vertical_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        horizontal_rail: Rail { background:None, border: Default::default(), scroller: Scroller { color: Color::TRANSPARENT, border: Default::default() }},
        gap: None,
    };

    let mut hovered_color = HIGHLIGHTED_CONTAINER_COLOR;//color!(180,180,180);
    hovered_color.a = 0.4;
    let mut dragged_color = hovered_color;
    dragged_color.a = 0.7;

    match status {

        scrollable::Status::Active => {
            scrollable_style
        }
        scrollable::Status::Hovered{ .. } => {
            scrollable_style.horizontal_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: mascot.get_primary_color(),
                    border: Border {
                        color: mascot.get_primary_color(),
                        width: 0.5,
                        radius: 6.into()
                    },
                }
            };

            scrollable_style.vertical_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: hovered_color,
                    border: Border {
                        color:Color::TRANSPARENT,
                        width: 0.5,
                        radius: 6.into()
                    },
                }
            };
            scrollable_style
        }
        scrollable::Status::Dragged{ .. } => {
            scrollable_style.horizontal_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: mascot.get_secondary_color(),
                    border: Border {
                        color: mascot.get_secondary_color(),
                        width: 0.1,
                        radius: 6.into()
                    },
                }
            };
            scrollable_style.vertical_rail = Rail {
                background: None,
                border: Default::default(),
                scroller: Scroller {
                    color: dragged_color,
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.1,
                        radius: 6.into()
                    },
                }
            };

            scrollable_style
        }
    }
}
