use chrono::NaiveDate;
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::general_exercise::Id;

/// This struct contains the necessary data to display exercise data
pub struct RecentWorkoutVisualization {
    pub date: NaiveDate,
    pub exercise_names: Vec<String>
}

pub fn get_up_to_three_most_recent_workout_exercise_names(exercises: &Vec<Exercise>) -> Vec<RecentWorkoutVisualization> {
    let mut workouts: Vec<RecentWorkoutVisualization> = Vec::new();
    let ids = get_up_to_three_biggest_workout_ids(exercises);

    for id in ids {
        let recent_workout = get_exercises_in_workout_by_workout_id(exercises, id);
        workouts.push(recent_workout);
    }
    workouts
}
pub fn get_exercises_in_workout_by_workout_id(exercises: &Vec<Exercise>, workout_id: Id) -> RecentWorkoutVisualization {
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
pub fn get_up_to_three_biggest_workout_ids(exercises: &Vec<Exercise>) -> Vec<Id> {
    let mut ids: Vec<Id> = Vec::new();
    for exercise in exercises {
        ids.extend(exercise.get_up_to_three_largest_workout_ids().iter())
    }
    ids.sort_by(|a,b| b.cmp(a));
    ids.dedup();
    ids.truncate(3);

    ids
}
