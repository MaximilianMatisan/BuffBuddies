use crate::client::backend::exercise_create::{
    ExerciseCreate, ExerciseCreateString, StrengthSetCreate, WorkoutCreate,
};
use crate::client::backend::profile_stat_manager::ProfileStatManager;
use crate::client::backend::recent_workouts::{
    RecentWorkoutVisualization, get_up_to_three_most_recent_workout_exercise_names,
};
use crate::client::gui::bb_tab::workout_creation::ExerciseNumber;
use crate::client::gui::bb_theme::combo_box::{
    get_combo_box_all_exercises_state, get_combo_box_tracked_exercise_state,
};
use crate::common::exercise_mod::exercise::{
    Exercise, ExerciseDataPoints, generate_example_exercise,
};
use crate::common::exercise_mod::general_exercise::{
    ExerciseCategory, ExerciseEquipment, ExerciseForce, ExerciseLevel, GeneralExerciseInfo, Id,
    Muscle,
};
use crate::common::exercise_mod::set::{Reps, StrengthSet};
use crate::common::exercise_mod::weight::Kg;
use crate::common::user_mod::user::UserInformation;
use crate::common::workout_preset::WorkoutPreset;
use chrono::{Local, NaiveDate};
use iced::widget::combo_box;
use std::collections::HashSet;

///coins you receive each day you have done a workout
const DAILY_COIN_REWARD: u32 = 5;

pub enum CreateWorkoutError {
    WorkoutAlreadyInCreation,
}

pub struct ExerciseManager {
    /// Every exercise data, including general info and tracked StrengthSets
    pub exercises: Vec<Exercise>,

    ///Show further general infos for these exercise_ids in the gui
    pub extended_general_exercise_infos: HashSet<u32>,

    /// Selection options for a combo_box. Only containing tracked exercises
    pub tracked_exercise_state: combo_box::State<String>,
    /// Selection options for a combo_box. Containing all exercises
    pub all_exercise_state: combo_box::State<String>,

    /// Not necessarily a valid exercise name
    pub selected_exercise_name: String,

    //STATS OF SELECTED EXERCISE
    ///representing the heaviest weight used in a set per tracked day
    pub data_points: ExerciseDataPoints,
    pub all_time_lifted_weight: Kg,
    pub all_time_reps: Reps,
    pub all_time_sets: u64,
    pub weight_personal_record: Kg,
    pub set_with_most_total_lifted_weight: (NaiveDate, Kg),
    /// Needed for exercise creation menu
    pub workout_in_creation: Option<WorkoutCreate>,
    /// shows which exercise is being edited during workout creation
    pub exercise_in_edit_number: Option<ExerciseNumber>,
    /// used for iced to be able to show and edit the current workout in creation
    pub exercise_in_edit_strings: Option<ExerciseCreateString>,
    /// Contains the exercise data for the visualization of up to three most recent workouts
    pub recent_workouts: Vec<RecentWorkoutVisualization>,
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
            recent_workouts: get_up_to_three_most_recent_workout_exercise_names(&exercises),
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

        exercise_manager.all_exercise_state =
            get_combo_box_all_exercises_state(&exercise_manager.exercises);
        exercise_manager.tracked_exercise_state =
            get_combo_box_tracked_exercise_state(&exercise_manager.exercises);

        exercise_manager.update_selected_exercise(selected_exercise_name);

        exercise_manager
    }
}
impl ExerciseManager {
    pub fn update_exercise_manager_on_login(
        &mut self,
        exercises: Vec<Exercise>,
        selected_exercise: String,
    ) {
        self.exercises = exercises;
        self.selected_exercise_name = selected_exercise.clone();
        self.tracked_exercise_state = get_combo_box_tracked_exercise_state(&self.exercises);
        self.all_exercise_state = get_combo_box_all_exercises_state(&self.exercises);
        self.recent_workouts = get_up_to_three_most_recent_workout_exercise_names(&self.exercises);

        self.update_selected_exercise(selected_exercise)
    }
    /// Returns a reference to an Exercise, if the name inside
    /// ExerciseManager.selected_exercise_name exists
    /// If the exercise name doesn't exist it returns None
    pub fn get_selected_exercise(&self) -> Option<&Exercise> {
        self.exercises.iter().find(|ex| {
            ex.general_exercise_info
                .name
                .eq_ignore_ascii_case(&self.selected_exercise_name)
        })
    }
    /// Updates the selected_exercise and stats according to the new_exercise_name
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

    /// Saves the workout supplied as an input and the current date.
    /// As a side effect gives the user DAILY_COIN_REWARD coins if it's their
    /// first workout of the day.
    /// It also updates data after workout is safed
    pub fn save_workout(
        &mut self,
        workout: &WorkoutCreate,
        workout_id: Id,
        user_info: &mut UserInformation,
    ) {
        let local_time = Local::now().date_naive();
        let first_workout_today: bool =
            !self.is_set_tracked_on_date(&local_time) && !workout.is_empty();
        for exercise_data in &mut self.exercises {
            for exercise_create in workout {
                if exercise_create.name == exercise_data.general_exercise_info.name
                    && !exercise_create.sets.is_empty()
                {
                    let workout_sets: Vec<StrengthSet> = exercise_create
                        .sets
                        .iter()
                        .map(|set_create| {
                            StrengthSet::from_strength_set_create(set_create, workout_id)
                        })
                        .collect();

                    for workout_set in workout_sets {
                        exercise_data
                            .sets
                            .entry(local_time)
                            .or_default()
                            .push(workout_set);
                    }
                }
            }
        }
        self.update_app_data_after_save_workout(user_info, first_workout_today);
    }

    fn update_app_data_after_save_workout(
        &mut self,
        user_info: &mut UserInformation,
        first_workout_today: bool,
    ) {
        if first_workout_today {
            user_info.coin_balance += DAILY_COIN_REWARD;
        }
        self.update_selected_exercise(self.selected_exercise_name.clone());
        user_info.profile_stat_manager =
            ProfileStatManager::new(&self.exercises, user_info.user_goals.weekly_workouts as u32);
        self.tracked_exercise_state = get_combo_box_tracked_exercise_state(&self.exercises);
        self.recent_workouts = get_up_to_three_most_recent_workout_exercise_names(&self.exercises);
    }

    ///Filters out the sets with 0 weight or reps in current workout_in_creation
    pub fn filter_workout_creation(&mut self) {
        let mut workout_filtered: WorkoutCreate = Vec::new();
        if let Some(workout) = &self.workout_in_creation {
            for exercise in workout {
                let filtered_sets: Vec<StrengthSetCreate> = exercise
                    .sets
                    .iter()
                    .filter_map(|strength_set| {
                        if strength_set.reps > 0 && strength_set.weight > 0.0 {
                            Some(strength_set.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                workout_filtered.push(ExerciseCreate {
                    name: exercise.name.clone(),
                    sets: filtered_sets,
                })
            }
        }
        self.workout_in_creation = Some(workout_filtered);
    }

    /// Returns whether a set of any exercise was tracked on the given day or not
    pub fn is_set_tracked_on_date(&self, date: &NaiveDate) -> bool {
        let mut first_workout_today = false;
        for exercise_data in &self.exercises {
            if exercise_data.sets.contains_key(date) {
                first_workout_today = true;
            }
        }
        first_workout_today
    }

    /// clears the current workout in creation
    pub fn clear_workout(&mut self) {
        self.workout_in_creation = None;
        self.exercise_in_edit_strings = None;
        self.exercise_in_edit_number = None;
    }

    /// creates the data needed for workout creation when it is started
    pub fn start_workout(&mut self) {
        if self.workout_in_creation.is_none() {
            self.workout_in_creation = Some(Vec::new());
        }
    }

    /// setup workout creation with exercises of a preset already being added
    pub fn start_workout_with_preset(
        &mut self,
        preset: &WorkoutPreset,
    ) -> Result<(), CreateWorkoutError> {
        if self.workout_in_creation.is_none() {
            self.force_workout_with_preset(preset);
            Ok(())
        } else {
            Err(CreateWorkoutError::WorkoutAlreadyInCreation)
        }
    }

    /// makes new workout with preset even if the old one gets replaced
    pub fn force_workout_with_preset(&mut self, preset: &WorkoutPreset) {
        let mut exercises = Vec::new();
        for exercise in &preset.exercises {
            exercises.push(ExerciseCreate::new(exercise.clone()))
        }
        self.workout_in_creation = Some(exercises);
    }

    /// Checks the historically latest done set for an exercise.
    /// Used when you create a new set during workout creation
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

    use crate::client::backend::exercise_create::{
        ExerciseCreate, StrengthSetCreate, WorkoutCreate,
    };
    use crate::client::backend::exercise_manager::{DAILY_COIN_REWARD, ExerciseManager};
    use crate::common::exercise_mod::exercise::tests::{MOCK_DATES, mock_exercise};
    use crate::common::exercise_mod::{
        exercise::Exercise, general_exercise::GeneralExerciseInfo, set::StrengthSet,
        weight::ExerciseWeight,
    };
    use crate::common::user_mod::user::UserInformation;
    use chrono::{Local, NaiveDate};

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
            vec![StrengthSet::new(0, ExerciseWeight::Kg(10.0), 1)],
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

    #[test]
    fn adding_two_exercises_only_one_daily_reward() {
        let mut ex_manager = ExerciseManager::default();
        let mut exercise_create_example = ExerciseCreate::test_case(0);
        exercise_create_example
            .sets
            .push(StrengthSetCreate::new(ExerciseWeight::Kg(50.0), 10));
        let mut user_information =
            UserInformation::default(&vec![Exercise::new(GeneralExerciseInfo::test_obj())]);
        ex_manager.save_workout(
            &vec![exercise_create_example.clone()],
            0,
            &mut user_information,
        );
        assert_eq!(
            UserInformation::default(&Vec::new()).coin_balance + DAILY_COIN_REWARD,
            user_information.coin_balance
        );
    }

    #[test]
    fn empty_exercise_not_saved() {
        let mut ex_manager = ExerciseManager::default();
        let exercise_create_example = ExerciseCreate::test_case(0);
        let mut user_information =
            UserInformation::default(&vec![Exercise::new(GeneralExerciseInfo::test_obj())]);
        ex_manager.save_workout(&vec![exercise_create_example], 0, &mut user_information);
        for exercise in ex_manager.exercises {
            assert_ne!(
                exercise.general_exercise_info.name,
                "Test Exercise".to_string()
            )
        }
    }

    #[test]
    fn first_workout_of_day_false() {
        let ex_manager = ExerciseManager {
            exercises: vec![mock_exercise()],
            extended_general_exercise_infos: Default::default(),
            tracked_exercise_state: Default::default(),
            all_exercise_state: Default::default(),
            selected_exercise_name: "".to_string(),
            data_points: vec![],
            all_time_lifted_weight: 0.0,
            all_time_reps: 0,
            all_time_sets: 0,
            weight_personal_record: 0.0,
            set_with_most_total_lifted_weight: (Default::default(), 0.0),
            workout_in_creation: None,
            exercise_in_edit_number: None,
            exercise_in_edit_strings: None,
            recent_workouts: Vec::new(),
        };
        assert!(ex_manager.is_set_tracked_on_date(&MOCK_DATES[0]));
    }
    #[test]
    fn first_workout_of_day_true() {
        let ex_manager = ExerciseManager::default();
        assert!(!ex_manager.is_set_tracked_on_date(&Local::now().date_naive()));
    }

    #[test]
    fn clear_workout() {
        let mut ex_manager = ExerciseManager {
            exercises: vec![mock_exercise()],
            extended_general_exercise_infos: Default::default(),
            tracked_exercise_state: Default::default(),
            all_exercise_state: Default::default(),
            selected_exercise_name: "".to_string(),
            data_points: vec![],
            all_time_lifted_weight: 0.0,
            all_time_reps: 0,
            all_time_sets: 0,
            weight_personal_record: 0.0,
            set_with_most_total_lifted_weight: (Default::default(), 0.0),
            workout_in_creation: Some(WorkoutCreate::default()),
            exercise_in_edit_number: None,
            exercise_in_edit_strings: None,
            recent_workouts: Vec::new(),
        };
        ex_manager.clear_workout();
        assert_eq!(ex_manager.workout_in_creation, None);
    }
    #[test]
    fn start_workout() {
        let workout = vec![ExerciseCreate::new("Name".to_string())];
        let mut ex_manager = ExerciseManager {
            exercises: vec![mock_exercise()],
            extended_general_exercise_infos: Default::default(),
            tracked_exercise_state: Default::default(),
            all_exercise_state: Default::default(),
            selected_exercise_name: "".to_string(),
            data_points: vec![],
            all_time_lifted_weight: 0.0,
            all_time_reps: 0,
            all_time_sets: 0,
            weight_personal_record: 0.0,
            set_with_most_total_lifted_weight: (Default::default(), 0.0),
            workout_in_creation: Some(workout.clone()),
            exercise_in_edit_number: None,
            exercise_in_edit_strings: None,
            recent_workouts: Vec::new(),
        };
        ex_manager.start_workout();
        assert_eq!(ex_manager.workout_in_creation, Some(workout));
    }
}
