use std::collections::HashMap;
use crate::client::backend::exercise_mod::exercise::{Exercise, generate_example_exercise};
use crate::client::backend::exercise_mod::set::Reps;
use crate::client::backend::exercise_mod::weight::Kg;
use chrono::{NaiveDate};
use iced::widget::combo_box;
use crate::client::gui::bb_widget::activity_widget::activity::AmountOfSets;

pub mod exercise;
pub mod set;
pub mod weight;

pub struct ExerciseManager {
    pub exercises: Vec<Exercise>,

    /// Not necessarily a valid exercise_mod name
    pub selected_exercise_name: String,
    pub owned_exercise_state: combo_box::State<String>,
    //STATS OF SELECTED EXERCISE
    ///representing the heaviest weight used in a set per tracked day
    pub data_points: Vec<(NaiveDate, Kg)>,
    pub all_time_lifted_weight: Kg,
    pub all_time_reps: Reps,
    pub all_time_sets: u64,
    pub weight_personal_record: Kg,
    pub set_with_most_total_lifted_weight: (NaiveDate, Kg),
}
impl Default for ExerciseManager {
    fn default() -> Self {
        let preacher_curl = generate_example_exercise("Preacher curl".to_string(), 50, 40.0);
        let bench_press = generate_example_exercise("Benchpress".to_string(), 200, 60.0);
        let barbell_row = generate_example_exercise("Barbell row".to_string(), 1, 80.0);
        let lateral_pulldown = Exercise::new("Lateral pulldown".to_string());

        let selected_exercise_name = "Benchpress".to_string();
        let mut exercise_manager = ExerciseManager {
            exercises: vec![preacher_curl, bench_press, barbell_row, lateral_pulldown],
            selected_exercise_name: selected_exercise_name.clone(),
            owned_exercise_state: combo_box::State::new(vec![]),
            data_points: vec![],
            all_time_lifted_weight: 0.0,
            all_time_reps: 0,
            all_time_sets: 0,
            weight_personal_record: 0.0,
            set_with_most_total_lifted_weight: (Default::default(), 0.0),
        };

        exercise_manager.owned_exercise_state = combo_box::State::new(
            exercise_manager
                .exercises
                .iter()
                .map(|ex| ex.name.clone())
                .collect(),
        );
        exercise_manager.update_selected_exercise(selected_exercise_name);

        exercise_manager
    }
}
impl ExerciseManager {
    pub fn get_selected_exercise(&self) -> Option<&Exercise> {
        self.exercises
            .iter()
            .find(|ex| ex.name.eq_ignore_ascii_case(&self.selected_exercise_name))
    }
    pub fn update_selected_exercise(&mut self, new_exercise_name: String) {
        self.selected_exercise_name = new_exercise_name;
        let option_selected_exercise = self.get_selected_exercise();

        if let Some(exercise) = option_selected_exercise {
            let all_time_lifted_weight = exercise.all_time_lifted_weight();
            let all_time_reps = exercise.all_time_reps();
            let all_time_sets = exercise.all_time_sets();
            let weight_personal_record = exercise.weight_personal_record();
            let set_with_most_total_lifted_weight = exercise.set_with_most_total_lifted_weight();

            self.data_points = exercise.calculate_max_weight_per_day();
            self.all_time_lifted_weight = all_time_lifted_weight;
            self.all_time_reps = all_time_reps;
            self.all_time_sets = all_time_sets;
            self.weight_personal_record = weight_personal_record;
            self.set_with_most_total_lifted_weight = set_with_most_total_lifted_weight;
        } else {
            self.data_points = vec![];
            self.all_time_lifted_weight = 0.0;
            self.all_time_reps = 0;
            self.all_time_sets = 0;
            self.weight_personal_record = 0.0;
            self.set_with_most_total_lifted_weight = (NaiveDate::default(), 0.0);
        }
    }
}

pub fn calculate_activity_data(exercise_data: &Vec<Exercise>) -> HashMap<NaiveDate, AmountOfSets> {
    let mut map: HashMap<NaiveDate, AmountOfSets> = HashMap::new();

    for exercise in exercise_data {
        for (date, set) in &exercise.sets {
            map
                .entry(*date)
                .and_modify(|entry| *entry += set.len() as AmountOfSets)
                .or_insert(set.len() as AmountOfSets);
        }
    }
    map
}