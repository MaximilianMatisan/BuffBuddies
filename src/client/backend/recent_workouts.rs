use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::general_exercise::Id;
use chrono::NaiveDate;

/// This struct contains the necessary data to display exercise data
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
    use crate::client::backend::recent_workouts::get_up_to_three_biggest_workout_ids;
    use crate::common::exercise_mod::exercise::Exercise;
    use crate::common::exercise_mod::exercise::tests::mock_exercise;
    use crate::common::exercise_mod::general_exercise::GeneralExerciseInfo;
    use crate::common::exercise_mod::general_exercise::Id;

    #[test]
    fn test_get_two_of_three_biggest_workout_ids() {
        let empty_exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        let mock_exercise = mock_exercise();

        let exercises = vec![empty_exercise, mock_exercise];
        assert_eq!(vec![1, 0], get_up_to_three_biggest_workout_ids(&exercises));
    }
    #[test]
    fn test_get_zero_of_three_biggest_workout_ids() {
        let exercises = vec![
            Exercise::new(GeneralExerciseInfo::test_obj()),
            Exercise::new(GeneralExerciseInfo::test_obj()),
        ];
        assert_eq!(Vec::<Id>::new(), get_up_to_three_biggest_workout_ids(&exercises));
    }
}
