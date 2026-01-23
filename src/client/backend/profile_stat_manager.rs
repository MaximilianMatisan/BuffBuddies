use crate::client::backend::exercise_mod::exercise::Exercise;
use crate::client::backend::exercise_mod::exercise_manager::calculate_activity_data;
use crate::client::backend::exercise_mod::set::Reps;
use crate::client::backend::exercise_mod::weight::Kg;
use crate::client::gui::bb_widget::activity_widget::activity::ActivityData;

/// The data included in this struct is only there for performance enhancement purposes
/// so that these values don't have to be calculated with every frame inside the view function
#[derive(Clone)]
pub struct ProfileStatManager {
    pub activity_data: ActivityData,
    pub total_sets: u64,
    pub total_reps: Reps,
    pub total_lifted_weight: Kg,
    pub best_pr: (String, Kg),
}

impl ProfileStatManager {
    pub fn new(exercise_data: &Vec<Exercise>) -> Self {
        ProfileStatManager {
            activity_data: calculate_activity_data(exercise_data),
            total_sets: total_sets(exercise_data),
            total_reps: total_reps(exercise_data),
            total_lifted_weight: total_lifted_weight(exercise_data),
            best_pr: highest_weight_pr(exercise_data),
        }
    }
}
pub fn total_lifted_weight(exercise_data: &Vec<Exercise>) -> Kg {
    let mut result: Kg = 0.0;
    for exercise in exercise_data {
        result += exercise.all_time_lifted_weight();
    }
    result
}
pub fn total_sets(exercise_data: &Vec<Exercise>) -> u64 {
    let mut result: u64 = 0;
    for exercise in exercise_data {
        result += exercise.all_time_sets();
    }
    result
}
pub fn total_reps(exercise_data: &Vec<Exercise>) -> Reps {
    let mut result: Reps = 0;
    for exercise in exercise_data {
        result += exercise.all_time_reps();
    }
    result
}
pub fn highest_weight_pr(exercise_data: &Vec<Exercise>) -> (String, Kg) {
    let mut result: (String, Kg) = ("None".to_string(), 0.0);

    for exercise in exercise_data {
        let current_pr = exercise.weight_personal_record();
        if current_pr > result.1 {
            result = (exercise.general_exercise_info.name.clone(), current_pr)
        }
    }
    result
}
