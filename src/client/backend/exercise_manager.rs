use crate::client::backend::exercise_create::{ExerciseCreate, ExerciseCreateString};
use crate::client::gui::bb_tab::workout_creation::ExerciseNumber;
use crate::client::gui::bb_widget::activity_widget::activity::calculate_activity_data;
use crate::common::exercise_mod::exercise::{
    Exercise, ExerciseDataPoints, generate_example_exercise,
};
use crate::common::exercise_mod::general_exercise::{
    ExerciseCategory, ExerciseEquipment, ExerciseForce, ExerciseLevel, GeneralExerciseInfo, Muscle,
};
use crate::common::exercise_mod::set::{Reps, StrengthSet};
use crate::common::exercise_mod::weight::Kg;
use crate::common::user_mod::user::UserInformation;
use chrono::{Local, NaiveDate};
use iced::widget::combo_box;
use std::collections::HashSet;
use crate::client::gui::bb_theme::combo_box::{get_combo_box_all_exercises_state, get_combo_box_tracked_exercise_state};

pub struct ExerciseManager {
    //TODO get exercises from db
    pub exercises: Vec<Exercise>,

    ///Show further general infos for these exercise_ids in the gui
    pub extended_general_exercise_infos: HashSet<u32>,

    /// Selection options for a combo_box. Only containing tracked exercises
    pub tracked_exercise_state: combo_box::State<String>, //TODO update after tracking a workout
    /// Selection options for a combo_box. Containing all exercises
    pub all_exercise_state: combo_box::State<String>,

    /// Not necessarily a valid exercise_mod name
    pub selected_exercise_name: String,
    //STATS OF SELECTED EXERCISE
    ///representing the heaviest weight used in a set per tracked day
    pub data_points: ExerciseDataPoints,
    pub all_time_lifted_weight: Kg,
    pub all_time_reps: Reps,
    pub all_time_sets: u64,
    pub weight_personal_record: Kg,
    pub set_with_most_total_lifted_weight: (NaiveDate, Kg),
    // Needed for exercise creation menu
    pub workout_in_creation: Option<Vec<ExerciseCreate>>,
    pub exercise_in_edit_number: Option<ExerciseNumber>,
    pub exercise_in_edit_strings: Option<ExerciseCreateString>,
}
impl Default for ExerciseManager {
    fn default() -> Self {
        //TODO delete examples
        let general_info_preacher_curl = GeneralExerciseInfo {
            id: 0,
            name: "Preacher Curl".to_string(),
            force: ExerciseForce::Pull,
            level: ExerciseLevel::Beginner,
            equipment: ExerciseEquipment::Barbell,
            primary_muscle: Muscle::Biceps,
            instructions: "To perform this movement you will need a preacher bench and an E-Z bar. Grab the E-Z curl bar at the close inner handle (either have someone hand you the bar which is preferable or grab the bar from the front bar rest provided by most preacher benches). The palm of your hands should be facing forward and they should be slightly tilted inwards due to the shape of the bar.
With the upper arms positioned against the preacher bench pad and the chest against it, hold the E-Z Curl Bar at shoulder length. This will be your starting position.
As you breathe in, slowly lower the bar until your upper arm is extended and the biceps is fully stretched.
As you exhale, use the biceps to curl the weight up until your biceps is fully contracted and the bar is at shoulder height. Squeeze the biceps hard and hold this position for a second.
Repeat for the recommended amount of repetitions.".to_string(),
            category: ExerciseCategory::Strength,
        };
        let general_info_bench_press = GeneralExerciseInfo {
            id: 1,
            name: "Bench press".to_string(),
            force: ExerciseForce::Push,
            level: ExerciseLevel::Beginner,
            equipment: ExerciseEquipment::Barbell,
            primary_muscle: Muscle::Chest,
            instructions: "Lie back on a flat bench. Using a medium width grip (a grip that creates a 90-degree angle in the middle of the movement between the forearms and the upper arms), lift the bar from the rack and hold it straight over you with your arms locked. This will be your starting position.
From the starting position, breathe in and begin coming down slowly until the bar touches your middle chest.
After a brief pause, push the bar back to the starting position as you breathe out. Focus on pushing the bar using your chest muscles. Lock your arms and squeeze your chest in the contracted position at the top of the motion, hold for a second and then start coming down slowly again. Tip: Ideally, lowering the weight should take about twice as long as raising it.
Repeat the movement for the prescribed amount of repetitions.
When you are done, place the bar back in the rack.".to_string(),
            category: ExerciseCategory::Strength,
        };
        let general_info_barbell_row = GeneralExerciseInfo {
            id: 2,
            name: "Bent over barbell row".to_string(),
            force: ExerciseForce::Pull,
            level: ExerciseLevel::Beginner,
            equipment: ExerciseEquipment::Barbell,
            primary_muscle: Muscle::MiddleBack,
            instructions: "Holding a barbell with a pronated grip (palms facing down), bend your knees slightly and bring your torso forward, by bending at the waist, while keeping the back straight until it is almost parallel to the floor. Tip: Make sure that you keep the head up. The barbell should hang directly in front of you as your arms hang perpendicular to the floor and your torso. This is your starting position.
Now, while keeping the torso stationary, breathe out and lift the barbell to you. Keep the elbows close to the body and only use the forearms to hold the weight. At the top contracted position, squeeze the back muscles and hold for a brief pause.
Then inhale and slowly lower the barbell back to the starting position.
Repeat for the recommended amount of repetitions.".to_string(),
            category: ExerciseCategory::Strength
        };

        let preacher_curl = generate_example_exercise(general_info_preacher_curl, 50, 40.0);
        let bench_press = generate_example_exercise(general_info_bench_press, 200, 60.0);
        let barbell_row = generate_example_exercise(general_info_barbell_row, 1, 80.0);

        let exercises = vec![preacher_curl, bench_press, barbell_row];

        let selected_exercise_name = "Bench press".to_string();
        let mut exercise_manager = ExerciseManager {
            exercises,
            extended_general_exercise_infos: HashSet::new(),
            selected_exercise_name: selected_exercise_name.clone(),
            tracked_exercise_state: combo_box::State::new(vec![]),
            all_exercise_state: combo_box::State::new(vec![]),
            data_points: vec![],
            all_time_lifted_weight: 0.0,
            all_time_reps: 0,
            all_time_sets: 0,
            weight_personal_record: 0.0,
            set_with_most_total_lifted_weight: (Default::default(), 0.0),
            workout_in_creation: None,
            exercise_in_edit_number: None,
            exercise_in_edit_strings: None,
        };

        exercise_manager.all_exercise_state = get_combo_box_all_exercises_state(&exercise_manager.exercises);
        exercise_manager.tracked_exercise_state = get_combo_box_tracked_exercise_state(&exercise_manager.exercises);

        exercise_manager.update_selected_exercise(selected_exercise_name);

        exercise_manager
    }
}
impl ExerciseManager {

    pub fn update_exercise_manager_on_login(&mut self, exercises: Vec<Exercise>) {
        let most_recently_tracked_exercise = "".to_string(); //TODO get
        self.exercises = exercises;
        self.selected_exercise_name = most_recently_tracked_exercise.clone();
        self.tracked_exercise_state = get_combo_box_tracked_exercise_state(&self.exercises);
        self.all_exercise_state = get_combo_box_all_exercises_state(&self.exercises);

        self.update_selected_exercise(most_recently_tracked_exercise)
    }
    pub fn get_selected_exercise(&self) -> Option<&Exercise> {
        self.exercises.iter().find(|ex| {
            ex.general_exercise_info
                .name
                .eq_ignore_ascii_case(&self.selected_exercise_name)
        })
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

    pub fn save_workout(
        &mut self,
        workout: &Vec<ExerciseCreate>,
        user_info: &mut UserInformation,
    ) -> Result<(), ()> {
        let mut first_workout_today: bool = true;
        let local_time = Local::now().date_naive();
        for exercise_data in &mut self.exercises {
            if exercise_data.sets.contains_key(&local_time) {
                first_workout_today = false;
            }
            for exercise in workout {
                if exercise.name == exercise_data.general_exercise_info.name
                    && !exercise.sets.is_empty()
                {
                    match exercise_data.sets.get(&local_time) {
                        None => {
                            exercise_data.sets.insert(local_time, exercise.sets.clone());
                        }
                        Some(old_sets_same_day) => {
                            let mut new_vec = old_sets_same_day.clone();
                            for set in &exercise.sets {
                                new_vec.push(set.clone());
                            }
                            exercise_data.sets.remove(&local_time);
                            exercise_data.sets.insert(local_time, new_vec);
                        }
                    }
                }
            }
        }
        if first_workout_today {
            user_info.coin_balance += 5;
        }
        self.update_selected_exercise(self.selected_exercise_name.clone());
        user_info.profile_stat_manager.activity_data = calculate_activity_data(&self.exercises);
        self.tracked_exercise_state = get_combo_box_tracked_exercise_state(&self.exercises);
        Ok(())
    }

    pub fn clear_workout(&mut self) {
        self.workout_in_creation = None;
        self.exercise_in_edit_strings = None;
        self.exercise_in_edit_number = None;
    }

    pub fn start_workout(&mut self) {
        self.workout_in_creation = Some(Vec::new());
    }

    pub fn get_last_done_set(&self, exercise: &String) -> Option<StrengthSet> {
        let mut set = None;
        for exercise_data in &self.exercises {
            if exercise_data.general_exercise_info.name == *exercise {
                if let Some((_date, sets)) = exercise_data.sets.iter().next_back() {
                    if !sets.is_empty() {
                        set = Some(sets[0].clone())
                    }
                }
            }
        }
        set
    }
}
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::client::backend::exercise_manager::ExerciseManager;
    use crate::common::exercise_mod::{
        exercise::Exercise, general_exercise::GeneralExerciseInfo, set::StrengthSet,
        weight::ExerciseWeight,
    };
    use chrono::NaiveDate;

    #[test]
    fn select_invalid_exercise() {
        let mut ex_manager = ExerciseManager::default();
        ex_manager.update_selected_exercise("a".to_string());

        assert_eq!(ex_manager.data_points, vec![]);
        assert_eq!(ex_manager.all_time_lifted_weight, 0.0);
        assert_eq!(ex_manager.all_time_reps, 0);
        assert_eq!(ex_manager.all_time_sets, 0);
        assert_eq!(ex_manager.weight_personal_record, 0.0);
        assert_eq!(
            ex_manager.set_with_most_total_lifted_weight,
            (NaiveDate::default(), 0.0)
        );
    }

    #[test]
    fn select_valid_exercise() {
        let mut ex_manager = ExerciseManager::default();
        let mock_exercise_name = "Mock exercise".to_string();
        let mut test_stats: BTreeMap<NaiveDate, Vec<StrengthSet>> = BTreeMap::new();
        test_stats.insert(
            NaiveDate::default(),
            vec![StrengthSet::new(ExerciseWeight::Kg(10.0), 1)],
        );
        let mock_exercise = Exercise {
            general_exercise_info: GeneralExerciseInfo {
                name: mock_exercise_name.clone(),
                ..Default::default()
            },
            sets: test_stats,
        };
        ex_manager.exercises.push(mock_exercise);
        ex_manager.update_selected_exercise(mock_exercise_name);

        assert_eq!(ex_manager.data_points, vec![(NaiveDate::default(), 10.0)]);
        assert_eq!(ex_manager.all_time_lifted_weight, 10.0);
        assert_eq!(ex_manager.all_time_reps, 1);
        assert_eq!(ex_manager.all_time_sets, 1);
        assert_eq!(ex_manager.weight_personal_record, 10.0);
        assert_eq!(
            ex_manager.set_with_most_total_lifted_weight,
            (NaiveDate::default(), 10.0)
        );
    }
}
