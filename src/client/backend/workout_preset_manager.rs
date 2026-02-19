use crate::common::workout_preset::{PresetCreation, WorkoutPreset};

pub enum PresetSafeError {
    NoPresetToSafe,
    NameAlreadyExists,
    NameEmpty,
    PresetEmpty,
}

#[derive(Default)]
pub struct WorkoutPresetManager {
    pub presets: Vec<WorkoutPreset>,
    //preset creation
    pub preset_in_creation: Option<PresetCreation>,
}

impl WorkoutPresetManager {
    pub fn start_preset_creation(&mut self) {
        if self.preset_in_creation.is_none() {
            self.preset_in_creation = Some(PresetCreation::default())
        }
    }

    pub fn add_preset(&mut self) {
        if let Some(new_preset) = &self.preset_in_creation {
            //add preset to front, so the newest created is the first to be shown
            self.presets.insert(0, new_preset.workout_preset.clone());
            self.preset_in_creation = None;
        }
    }

    pub fn check_preset(&self) -> Result<(), PresetSafeError> {
        if let Some(new_preset) = &self.preset_in_creation {
            if new_preset.workout_preset.exercises.is_empty() {
                return Err(PresetSafeError::PresetEmpty);
            }
            if new_preset.workout_preset.name.is_empty() {
                return Err(PresetSafeError::NameEmpty);
            }
            for preset in &self.presets {
                if preset.name == new_preset.workout_preset.name {
                    return Err(PresetSafeError::NameAlreadyExists);
                }
            }
            Ok(())
        } else {
            Err(PresetSafeError::NoPresetToSafe)
        }
    }

    pub fn move_preset_to_front(&mut self, preset: &WorkoutPreset) {
        if let Some(index) = self
            .presets
            .iter()
            .position(|preset_in_vec| -> bool { preset.name == preset_in_vec.name })
        {
            self.presets.remove(index);
            self.presets.insert(0, preset.clone())
        }
    }
}
