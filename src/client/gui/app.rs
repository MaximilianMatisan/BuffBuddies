use crate::client::backend::login_state::LoginState;
use chrono::{Duration, NaiveDate};
use rand::Rng;
use crate::client::backend::exercise::set::StrengthSet;
use crate::client::backend::exercise::weight::ExerciseWeight;
use crate::client::backend::exercise::exercise::{generate_example_exercise, Exercise};
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::activity::activity::ActivityWidget;
use crate::client::gui::mascots::Mascot;

pub struct App {
    pub loading: bool,
    pub screen: Tab,
    pub active_mascot: Mascot,
    pub activity_widget: ActivityWidget,
    pub login_state: LoginState,

    pub exercises: Vec<Exercise>,
    /// Not necessarily a valid exercise name
    pub selected_exercise_name: String
}

impl Default for App {
    fn default() -> Self {
        let default_mascot = Mascot::default();
        let preacher_curl =
            generate_example_exercise("Preacher curl".to_string(), 50, 40.0);
        let bench_press =
            generate_example_exercise("Benchpress".to_string(), 200, 60.0);
        let barbell_row =
            generate_example_exercise("Barbell row".to_string(), 1, 80.0);

        App {
            loading: false,
            screen: Tab::Home,
            active_mascot: default_mascot.clone(),
            login_state: LoginState::default(),
            activity_widget: ActivityWidget::new(default_mascot.clone()),
            //EXAMPLE
            exercises: vec![
                preacher_curl,
                bench_press,
                barbell_row
            ],
            selected_exercise_name: "".to_string()
        }
    }
}
impl App {
    pub fn get_selected_exercise(&self) -> Option<&Exercise> {
        self.exercises.iter()
            .find(|ex| ex.name.eq_ignore_ascii_case(&self.selected_exercise_name))
    }
}