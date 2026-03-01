use crate::client::gui::bb_widget::activity_widget::activity::{ActivityData, AmountOfSets};
use crate::client::gui::bb_widget::activity_widget::date_utils;
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::general_exercise::Id;
use crate::common::exercise_mod::set::Reps;
use crate::common::exercise_mod::weight::Kg;
use chrono::{Duration, Local, NaiveDate};
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
    pub weekly_workout_streak: u32,
    pub workouts_this_week: u32,
}

impl ProfileStatManager {
    pub fn new(exercise_data: &Vec<Exercise>, weekly_workout_goal: u32) -> Self {
        let local_today = Local::now().date_naive();
        ProfileStatManager {
            activity_data: calculate_activity_data(exercise_data),
            total_sets: total_sets(exercise_data),
            total_reps: total_reps(exercise_data),
            total_lifted_weight: total_lifted_weight(exercise_data),
            best_pr: highest_weight_pr(exercise_data),
            weekly_workout_streak: calculate_weekly_workout_streak(
                exercise_data,
                weekly_workout_goal,
            ),
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
/// Calculates the weekly workout streak based on a weekly workout streak goal.
/// Doesn't take previous streak with different goal into account
pub fn calculate_weekly_workout_streak(exercises: &Vec<Exercise>, weekly_workout_goal: u32) -> u32 {
    if weekly_workout_goal == 0 {
        return 0;
    }

    let mut today = Local::now().date_naive();

    let mut streak = 0;

    while amount_of_workouts_in_week(exercises, today) >= weekly_workout_goal {
        streak += 1;
        today -= Duration::days(7);
    }

    streak
}

#[cfg(test)]
mod tests {
    use crate::client::backend::profile_stat_manager::{
        amount_of_workouts_in_week, calculate_weekly_workout_streak,
    };
    use crate::common::exercise_mod::exercise;
    use crate::common::exercise_mod::exercise::Exercise;
    use crate::common::exercise_mod::general_exercise::{GeneralExerciseInfo, Id};
    use crate::common::exercise_mod::set::StrengthSet;
    use crate::common::exercise_mod::weight::ExerciseWeight;
    use chrono::{Duration, Local, NaiveDate};
    use std::collections::BTreeMap;

    #[test]
    fn weekly_workout_streak_with_streak_goal_zero() {
        let mock_exercise1 = exercise::tests::mock_exercise();
        let mock_exercise2 = exercise::tests::mock_exercise();

        let exercises = vec![mock_exercise1, mock_exercise2];

        assert_eq!(calculate_weekly_workout_streak(&exercises, 0), 0);
    }
    #[test]
    fn weekly_workout_streak_with_streak_goal_upper_bound() {
        let mock_exercise1 = exercise::tests::mock_exercise();
        let mock_exercise2 = exercise::tests::mock_exercise();

        let exercises = vec![mock_exercise1, mock_exercise2];
        assert_eq!(calculate_weekly_workout_streak(&exercises, u32::MAX), 0);
    }

    #[test]
    fn weekly_workout_streak_with_no_tracked_sets() {
        let exercises: Vec<Exercise> = Vec::new();

        assert_eq!(calculate_weekly_workout_streak(&exercises, 1), 0);
    }
    #[test]
    fn test_weekly_workout_streak_out_of_range() {
        let mock_exercise = exercise::tests::mock_exercise();

        let weekly_workout_goal = 2;

        let exercises = vec![mock_exercise];

        assert_eq!(
            calculate_weekly_workout_streak(&exercises, weekly_workout_goal),
            0
        );
    }

    #[test]
    fn test_weekly_workout_streak_20() {
        let number_of_workouts = 20;
        let today = Local::now().date_naive();
        let mut exercises: Vec<Exercise> = Vec::new();

        for i in 0..number_of_workouts {
            let mut sets = BTreeMap::new();
            sets.insert(
                today - Duration::weeks(i),
                vec![StrengthSet::new(i as Id, ExerciseWeight::Kg(10.0), 5)],
            );

            exercises.push(Exercise {
                general_exercise_info: GeneralExerciseInfo::test_obj(),
                sets,
            })
        }
        let weekly_workout_goal = 1;

        assert_eq!(
            calculate_weekly_workout_streak(&exercises, weekly_workout_goal),
            number_of_workouts as u32
        );
    }
    #[test]
    fn test_amount_of_workouts_in_week_one_mock_exercise() {
        let first_tracked_day_of_mock_exercise = exercise::tests::MOCK_DATES[0];
        let mock_exercise1 = exercise::tests::mock_exercise();

        let number_of_distinct_workouts = 2;

        let exercises = vec![mock_exercise1];

        assert_eq!(
            amount_of_workouts_in_week(&exercises, first_tracked_day_of_mock_exercise),
            number_of_distinct_workouts
        );
    }

    #[test]
    fn test_amount_of_workouts_in_week_two_mock_exercise() {
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
    fn test_amount_of_workouts_in_week_for_empty_exercises() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 3).unwrap();
        let exercise1 = Exercise::new(GeneralExerciseInfo::test_obj());
        let exercise2 = Exercise::new(GeneralExerciseInfo::test_obj());

        let exercises = vec![exercise1, exercise2];
        assert_eq!(amount_of_workouts_in_week(&exercises, date), 0);
    }
}
