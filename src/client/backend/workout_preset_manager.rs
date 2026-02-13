use crate::common::workout_preset::WorkoutPreset;

#[derive(Default)]
pub struct WorkoutPresetManager {
    pub presets: Vec<WorkoutPreset>,
}
