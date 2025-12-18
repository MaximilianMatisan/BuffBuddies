use crate::client::backend::exercise::exercise::{generate_example_exercise, Exercise};

pub mod weight;
pub mod set;
pub mod exercise;

pub struct ExerciseManager {
    pub exercises: Vec<Exercise>,
    /// Not necessarily a valid exercise name
    pub selected_exercise_name: String,
}
impl Default for ExerciseManager {
    fn default() -> Self {

        let preacher_curl =
            generate_example_exercise("Preacher curl".to_string(), 50, 40.0);
        let bench_press =
            generate_example_exercise("Benchpress".to_string(), 200, 60.0);
        let barbell_row =
            generate_example_exercise("Barbell row".to_string(), 1, 80.0);
        ExerciseManager {
            exercises: vec![
                preacher_curl,
                bench_press,
                barbell_row
            ],
            selected_exercise_name: "".to_string(),
        }
    }
}
impl ExerciseManager {
    pub fn get_selected_exercise(&self) -> Option<&Exercise> {
        self.exercises.iter()
            .find(|ex| ex.name.eq_ignore_ascii_case(&self.selected_exercise_name))
    }
}