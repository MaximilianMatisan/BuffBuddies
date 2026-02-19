use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutPreset {
    pub name: String,
    pub exercises: Vec<String>,
}

impl Default for WorkoutPreset {
    fn default() -> Self {
        WorkoutPreset {
            name: "Preset Title".to_string(),
            exercises: Vec::default(),
        }
    }
}

#[derive(Default)]
pub struct PresetCreation {
    pub workout_preset: WorkoutPreset,
    pub edit_title: bool,
}
