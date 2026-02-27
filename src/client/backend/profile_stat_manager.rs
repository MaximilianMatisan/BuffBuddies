use crate::client::gui::bb_widget::activity_widget::activity::{ActivityData, AmountOfSets};
use crate::client::gui::bb_widget::activity_widget::date_utils;
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::general_exercise::Id;
use crate::common::exercise_mod::set::Reps;
use crate::common::exercise_mod::weight::Kg;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// The data included in this struct is only there for performance enhancement purposes
/// so that these values don't have to be calculated with every frame inside the view function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStatManager {
    pub activity_data: ActivityData,
    pub total_sets: u64,
    pub total_reps: Reps,
    pub total_lifted_weight: Kg,
    pub best_pr: (String, Kg),
    pub workouts_this_week: u32,
}

impl ProfileStatManager {
    pub fn new(exercise_data: &Vec<Exercise>) -> Self {
        let local_today = Local::now().date_naive();
        ProfileStatManager {
            activity_data: calculate_activity_data(exercise_data),
            total_sets: total_sets(exercise_data),
            total_reps: total_reps(exercise_data),
            total_lifted_weight: total_lifted_weight(exercise_data),
            best_pr: highest_weight_pr(exercise_data),
            workouts_this_week: amount_of_workouts_in_week(exercise_data, local_today),
        }
    }
}

pub fn calculate_activity_data(exercise_data: &Vec<Exercise>) -> ActivityData {
    let mut map: ActivityData = HashMap::new();

    for exercise in exercise_data {
        for (date, set) in &exercise.sets {
            map.entry(*date)
                .and_modify(|entry| *entry += set.len() as AmountOfSets)
                .or_insert(set.len() as AmountOfSets);
        }
    }
    map
}
/// Calculate the total amount of sets across all exercises
pub fn total_sets(exercise_data: &Vec<Exercise>) -> u64 {
    let mut result: u64 = 0;
    for exercise in exercise_data {
        result += exercise.all_time_sets();
    }
    result
}
/// Calculates the total amount of reps across all exercises
pub fn total_reps(exercise_data: &Vec<Exercise>) -> Reps {
    let mut result: Reps = 0;
    for exercise in exercise_data {
        result += exercise.all_time_reps();
    }
    result
}

/// Calculates the total amount of reps * weight on every StrengthSet across all exercises
pub fn total_lifted_weight(exercise_data: &Vec<Exercise>) -> Kg {
    let mut result: Kg = 0.0;
    for exercise in exercise_data {
        result += exercise.all_time_lifted_weight();
    }
    result
}
/// Calculates the exercise name and weight of the set with
/// the highest tracked weight across all exercises <br>
/// Returns ("None",0) if no StrengthSet is tracked
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
/// Calculates the amount of workouts tracked in the corresponding <br>
/// week from Monday until Sunday the given `date` falls in
pub fn amount_of_workouts_in_week(exercise_data: &Vec<Exercise>, date: NaiveDate) -> u32 {
    let dates_of_local_todays_week = date_utils::get_dates_of_week_belonging_to_date(date);

    let mut seen_workout_ids_this_week: HashSet<Id> = HashSet::new();

    for exercise in exercise_data {
        for (date_of_set, vec_of_sets) in &exercise.sets {
            if !dates_of_local_todays_week.contains(date_of_set) {
                continue;
            }
            for set in vec_of_sets {
                seen_workout_ids_this_week.insert(set.workout_id);
            }
        }
    }

    seen_workout_ids_this_week.len() as u32
}

#[cfg(test)]
mod tests {
    use crate::client::backend::profile_stat_manager::amount_of_workouts_in_week;
    use crate::common::exercise_mod::exercise;

    #[test]
    fn test_amount_of_workouts_in_two_mock_exercise() {
        let first_tracked_day_of_mock_exercise = exercise::tests::MOCK_DATES[0];
        // Same ids across both exercises
        let mock_exercise1 = exercise::tests::mock_exercise();
        let mock_exercise2 = exercise::tests::mock_exercise();

        let number_of_distinct_workouts = 2;

        let exercises = vec![mock_exercise1, mock_exercise2];

        assert_eq!(
            amount_of_workouts_in_week(&exercises, first_tracked_day_of_mock_exercise),
            number_of_distinct_workouts
        );
    }
    #[test]
    fn test_amount_of_workouts_in_one_mock_exercise() {
        let first_tracked_day_of_mock_exercise = exercise::tests::MOCK_DATES[0];
        let mock_exercise1 = exercise::tests::mock_exercise();

        let number_of_distinct_workouts = 2;

        let exercises = vec![mock_exercise1];

        assert_eq!(
            amount_of_workouts_in_week(&exercises, first_tracked_day_of_mock_exercise),
            number_of_distinct_workouts
        );
    }
}
