use crate::client::backend::login_state::LoginState;
use chrono::NaiveDate;
use crate::client::backend::exercise::exercise::Exercise;
use crate::client::backend::exercise::set::StrengthSet;
use crate::client::backend::exercise::weight::ExerciseWeight;
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
    pub selected_exercise_id: usize
}

impl Default for App {
    fn default() -> Self {
        let default_mascot = Mascot::default();
        let mut preacher_curl = Exercise::new("Preacher curl".to_string()); //TODO EXAMPLE
        preacher_curl.sets.insert(NaiveDate::from_ymd_opt(2025, 12, 2).unwrap(),
                                  vec![
                                       StrengthSet::new(ExerciseWeight::Kg(45.0), 6),
                                       StrengthSet::new(ExerciseWeight::Kg(40.0), 10)
                                  ]);
        App {
            loading: false,
            screen: Tab::Home,
            active_mascot: default_mascot.clone(),
            login_state: LoginState::default(),
            activity_widget: ActivityWidget::new(default_mascot.clone()),
            //EXAMPLE
            exercises: vec![Exercise::new("Benchpress".to_string()),
                            Exercise::new("Lateral pulldown".to_string()),
                            preacher_curl],
            selected_exercise_id: 2
        }
    }
}
impl App {
    pub fn get_selected_exercise(&self) -> Option<&Exercise> {
        if self.selected_exercise_id >= self.exercises.len() {
            return None
        }
        Some(&self.exercises[self.selected_exercise_id])
    }
}