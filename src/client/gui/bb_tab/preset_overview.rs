use crate::client::gui::bb_theme::color::TEXT_COLOR;
use crate::client::gui::bb_theme::scrollable::{
    ScrollableExtension, ScrollableStyle, TAB_SCROLLBAR_PADDING, TAB_SCROLLBAR_WIDTH,
    create_scrollable,
};
use crate::client::gui::bb_theme::separator::{DEFAULT_SEPARATOR_HEIGHT, separator_line};
use crate::client::gui::bb_theme::text_format::FIRA_SANS_EXTRABOLD;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::bb_widget::workout::{DEFAULT_WORKOUT_WIDGET_WIDTH, WorkoutWidget};
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::workout_preset::WorkoutPreset;
use iced::Element;
use iced::widget::{Column, Row, Space, text};
use iced_core::Length;
use iced_core::alignment::Horizontal;

const PRESET_TITLE_FONT_SIZE: f32 = 50.0;
const PRESETS_PER_ROW: usize = 4;
pub fn preset_overview_screen<'a>(
    mascot: &'a Mascot,
    presets: &[WorkoutPreset],
) -> Element<'a, Message> {
    let preset_title = text("Presets")
        .size(PRESET_TITLE_FONT_SIZE)
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR);

    let preset_grid = preset_grid_column(mascot, presets);

    let separator_width = PRESETS_PER_ROW as f32 * DEFAULT_WORKOUT_WIDGET_WIDTH;
    let separator_line = separator_line(mascot, DEFAULT_SEPARATOR_HEIGHT).width(separator_width);

    let contents = Column::new()
        .push(preset_title)
        .push(Space::with_height(INDENT))
        .push(separator_line)
        .push(Space::with_height(INDENT))
        .push(preset_grid)
        .padding([LARGE_INDENT, 0.0]) //[TOP/BOTTOM,LEFT_RIGHT]
        .align_x(Horizontal::Center);

    let padded_contents = Row::new()
        .push(Space::with_width(Length::Fill))
        .push(contents)
        .push(Space::with_width(Length::Fill));

    create_scrollable(padded_contents, *mascot, ScrollableStyle::Default)
        .add_vertical_scrollbar(TAB_SCROLLBAR_WIDTH, TAB_SCROLLBAR_PADDING)
        .into()
}
fn preset_grid_column<'a>(mascot: &Mascot, presets: &[WorkoutPreset]) -> Column<'a, Message> {
    let mut column = Column::new().spacing(INDENT);

    for preset_chunk in presets.chunks(PRESETS_PER_ROW) {
        let mut row: Row<Message> = Row::new().spacing(INDENT);

        for preset in preset_chunk {
            row = row.push(WorkoutWidget::new_workout_preset_widget(preset, mascot));
        }
        column = column.push(row);
    }
    column
}
