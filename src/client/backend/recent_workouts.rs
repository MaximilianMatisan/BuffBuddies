use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::general_exercise::Id;
use chrono::NaiveDate;

/// This struct contains the necessary data to display exercise data
#[derive(Debug, PartialEq)]
pub struct RecentWorkoutVisualization {
    pub date: NaiveDate,
    pub exercise_names: Vec<String>,
}

pub fn get_up_to_three_most_recent_workout_exercise_names(
    exercises: &Vec<Exercise>,
) -> Vec<RecentWorkoutVisualization> {
    let mut workouts: Vec<RecentWorkoutVisualization> = Vec::new();
    let ids = get_up_to_three_biggest_workout_ids(exercises);

    for id in ids {
        let recent_workout = get_exercise_names_in_workout_and_date_by_workout_id(exercises, id);
        workouts.push(recent_workout);
    }
    workouts
}
/// Returns the date the workout with given id was tracked and
/// the names of the exercises the workout contained
fn get_exercise_names_in_workout_and_date_by_workout_id(
    exercises: &Vec<Exercise>,
    workout_id: Id,
) -> RecentWorkoutVisualization {
    let mut date_of_workout: Option<NaiveDate> = None;
    let mut exercises_in_workout = Vec::new();
    for exercise in exercises {
        if !exercise.contains_set_with_workout_id(workout_id) {
            continue;
        }
        if date_of_workout.is_none() {
            date_of_workout = exercise.get_date_of_workout_id(workout_id);
        }
        exercises_in_workout.push(exercise.general_exercise_info.name.clone());
    }

    RecentWorkoutVisualization {
        date: date_of_workout.unwrap_or_default(),
        exercise_names: exercises_in_workout,
    }
}
fn get_up_to_three_biggest_workout_ids(exercises: &Vec<Exercise>) -> Vec<Id> {
    let mut ids: Vec<Id> = Vec::new();
    for exercise in exercises {
        ids.extend(exercise.get_up_to_three_largest_workout_ids().iter())
    }
    ids.sort_by(|a, b| b.cmp(a));
    ids.dedup();
    ids.truncate(3);

    ids
}

#[cfg(test)]
mod tests {
    use crate::client::backend::recent_workouts::{
        RecentWorkoutVisualization, get_exercise_names_in_workout_and_date_by_workout_id,
        get_up_to_three_biggest_workout_ids, get_up_to_three_most_recent_workout_exercise_names,
    };
    use crate::common::exercise_mod::exercise::Exercise;
    use crate::common::exercise_mod::exercise::tests::{
        CUSTOM_TRACKED_DAYS, MOCK_DATES, custom_exercise_preset, mock_exercise,
    };
    use crate::common::exercise_mod::general_exercise::GeneralExerciseInfo;
    use crate::common::exercise_mod::general_exercise::Id;
    use chrono::{Duration, Local};
    use std::collections::BTreeMap;

    #[test]
    fn get_two_of_three_biggest_workout_ids() {
        let empty_exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        let mock_exercise = mock_exercise();

        let exercises = vec![empty_exercise, mock_exercise];
        assert_eq!(vec![1, 0], get_up_to_three_biggest_workout_ids(&exercises));
    }
    #[test]
    fn get_zero_of_three_biggest_workout_ids() {
        let exercises = vec![
            Exercise::new(GeneralExerciseInfo::test_obj()),
            Exercise::new(GeneralExerciseInfo::test_obj()),
        ];
        assert_eq!(
            Vec::<Id>::new(),
            get_up_to_three_biggest_workout_ids(&exercises)
        );
    }
    #[test]
    fn get_three_largest_workout_ids_of_example_data() {
        let exercises = vec![
            mock_exercise(),
            custom_exercise_preset(),
            Exercise {
                general_exercise_info: GeneralExerciseInfo::test_obj(),
                sets: BTreeMap::new(),
            },
        ];
        let expected_data = vec![
            CUSTOM_TRACKED_DAYS - 1,
            CUSTOM_TRACKED_DAYS - 2,
            CUSTOM_TRACKED_DAYS - 3,
        ];
        assert_eq!(
            expected_data,
            get_up_to_three_biggest_workout_ids(&exercises)
        );
    }
    #[test]
    fn get_exercise_names_and_date_of_workout_id_mock() {
        let exercises = vec![mock_exercise()];

        for (i, date) in MOCK_DATES.iter().enumerate() {
            let expected_data_for_date = RecentWorkoutVisualization {
                date: *date,
                exercise_names: vec![mock_exercise().general_exercise_info.name],
            };
            assert_eq!(
                expected_data_for_date,
                get_exercise_names_in_workout_and_date_by_workout_id(&exercises, i as Id)
            )
        }
    }
    #[test]
    fn get_up_to_three_recent_workout_visualization_data() {
        let exercises = vec![custom_exercise_preset()];

        let mut most_recent_workouts = Vec::new();

        for i in 0..=2 {
            let expected_data_for_date = RecentWorkoutVisualization {
                date: Local::now().date_naive()
                    + Duration::days((CUSTOM_TRACKED_DAYS - 1 - i) as i64),
                exercise_names: vec![custom_exercise_preset().general_exercise_info.name],
            };
            most_recent_workouts.push(expected_data_for_date)
        }
        assert_eq!(
            get_up_to_three_most_recent_workout_exercise_names(&exercises),
            most_recent_workouts
        );
    }
}
