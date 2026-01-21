use crate::client::backend::exercise::exercise_stats::{ExerciseStat, generate_example_exercise};
use crate::client::backend::exercise::general_exercise::{
    ExerciseCategory, ExerciseEquipment, ExerciseForce, ExerciseLevel, GeneralExerciseInfo, Muscle,
};
use crate::client::backend::exercise::set::Reps;
use crate::client::backend::exercise::weight::Kg;
use crate::client::gui::bb_widget::activity_widget::activity::AmountOfSets;
use chrono::NaiveDate;
use iced::widget::combo_box;
use std::collections::{HashMap, HashSet};

pub struct ExerciseManager {
    //TODO get general_exercise_info and exercise_stats from db
    pub general_exercise_info: Vec<GeneralExerciseInfo>,
    ///Show further general infos for these exercise_ids in the gui
    pub extended_general_exercise_infos: HashSet<u32>,

    pub exercise_stats: Vec<ExerciseStat>,

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
        let bench_press = generate_example_exercise("Bench press".to_string(), 200, 60.0);
        let barbell_row = generate_example_exercise("Barbell row".to_string(), 1, 80.0);
        let lateral_pulldown = ExerciseStat::new("Lateral pulldown".to_string());

        let selected_exercise_name = "Bench press".to_string();
        let test_exercise_info_0 = GeneralExerciseInfo {
            id: 0,
            name: "Preacher Curl".to_string(),
            force: Some(ExerciseForce::Pull),
            level: ExerciseLevel::Beginner,
            equipment: Some(ExerciseEquipment::Barbell),
            primary_muscle: Muscle::Biceps,
            instructions: "To perform this movement you will need a preacher bench and an E-Z bar. Grab the E-Z curl bar at the close inner handle (either have someone hand you the bar which is preferable or grab the bar from the front bar rest provided by most preacher benches). The palm of your hands should be facing forward and they should be slightly tilted inwards due to the shape of the bar.
With the upper arms positioned against the preacher bench pad and the chest against it, hold the E-Z Curl Bar at shoulder length. This will be your starting position.
As you breathe in, slowly lower the bar until your upper arm is extended and the biceps is fully stretched.
As you exhale, use the biceps to curl the weight up until your biceps is fully contracted and the bar is at shoulder height. Squeeze the biceps hard and hold this position for a second.
Repeat for the recommended amount of repetitions.".to_string(),
            category: ExerciseCategory::Strength,
        };
        let test_exercise_info_1 = GeneralExerciseInfo {
            id: 1,
            name: "Bench press".to_string(),
            force: Some(ExerciseForce::Push),
            level: ExerciseLevel::Beginner,
            equipment: Some(ExerciseEquipment::Barbell),
            primary_muscle: Muscle::Chest,
            instructions: "Lie back on a flat bench. Using a medium width grip (a grip that creates a 90-degree angle in the middle of the movement between the forearms and the upper arms), lift the bar from the rack and hold it straight over you with your arms locked. This will be your starting position.
From the starting position, breathe in and begin coming down slowly until the bar touches your middle chest.
After a brief pause, push the bar back to the starting position as you breathe out. Focus on pushing the bar using your chest muscles. Lock your arms and squeeze your chest in the contracted position at the top of the motion, hold for a second and then start coming down slowly again. Tip: Ideally, lowering the weight should take about twice as long as raising it.
Repeat the movement for the prescribed amount of repetitions.
When you are done, place the bar back in the rack.".to_string(),
            category: ExerciseCategory::Strength,
        };
        let mut exercise_manager = ExerciseManager {
            general_exercise_info: vec![test_exercise_info_0, test_exercise_info_1],
            extended_general_exercise_infos: HashSet::new(),
            exercise_stats: vec![preacher_curl, bench_press, barbell_row, lateral_pulldown],
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
                .exercise_stats
                .iter()
                .map(|ex| ex.name.clone())
                .collect(),
        );
        exercise_manager.update_selected_exercise(selected_exercise_name);

        exercise_manager
    }
}
impl ExerciseManager {
    pub fn get_selected_exercise(&self) -> Option<&ExerciseStat> {
        self.exercise_stats
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

pub fn calculate_activity_data(
    exercise_data: &Vec<ExerciseStat>,
) -> HashMap<NaiveDate, AmountOfSets> {
    let mut map: HashMap<NaiveDate, AmountOfSets> = HashMap::new();

    for exercise in exercise_data {
        for (date, set) in &exercise.sets {
            map.entry(*date)
                .and_modify(|entry| *entry += set.len() as AmountOfSets)
                .or_insert(set.len() as AmountOfSets);
        }
    }
    map
}
