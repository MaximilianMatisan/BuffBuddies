use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::container::DEFAULT_CONTAINER_RADIUS;
use crate::client::gui::bb_theme::custom_button::DEFAULT_BUTTON_RADIUS;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::overlay::menu;
use iced::widget::{combo_box, text_input};
use iced::widget::text_input::Status;
use iced_core::{Background, Border, Theme};
use crate::common::exercise_mod::exercise::Exercise;

pub fn create_text_input_style(mascot: &Mascot) -> impl Fn(&Theme, Status) -> text_input::Style {
    |_theme: &Theme, _status: Status| text_input::Style {
        background: Background::Color(color::BACKGROUND_COLOR),
        border: Border {
            color: Default::default(),
            width: 0.0,
            radius: DEFAULT_BUTTON_RADIUS.into(),
        },
        icon: Default::default(),
        placeholder: color::DESCRIPTION_TEXT_COLOR,
        value: color::TEXT_COLOR,
        selection: mascot.get_secondary_color(),
    }
}

pub fn create_menu_style(mascot: &Mascot) -> impl Fn(&Theme) -> menu::Style {
    |_theme: &Theme| menu::Style {
        background: Background::Color(color::LIGHTER_CONTAINER_COLOR),
        border: Border {
            color: Default::default(),
            width: 0.0,
            radius: DEFAULT_CONTAINER_RADIUS.into(),
        },
        text_color: color::TEXT_COLOR,
        selected_text_color: color::TEXT_COLOR,
        selected_background: Background::Color(mascot.get_primary_color()),
    }
}
pub fn get_combo_box_all_exercises_state(exercise: &Vec<Exercise>) -> combo_box::State<String> {
    combo_box::State::new( exercise.iter().map(|ex| ex.general_exercise_info.name.clone()).collect() )
}
pub fn get_combo_box_tracked_exercise_state(exercises: &Vec<Exercise>) -> combo_box::State<String> {
    let tracked_exercises = exercises.iter().filter(|ex| ex.is_tracked());
    combo_box::State::new(
        tracked_exercises
            .map(|ex| ex.general_exercise_info.name.clone())
            .collect(),
    )
}
