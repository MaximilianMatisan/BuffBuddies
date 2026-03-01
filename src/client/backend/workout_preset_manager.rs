use crate::common::workout_preset::{PresetCreation, WorkoutPreset};

#[derive(Debug, PartialEq)]
pub enum PresetSafeError {
    NoPresetToSafe,
    NameAlreadyExists,
    NameEmpty,
    PresetEmpty,
}

#[derive(Default)]
pub struct WorkoutPresetManager {
    pub presets: Vec<WorkoutPreset>,
    /// used for preset creation
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_preset_creation_none() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        workout_preset_manager.start_preset_creation();
        assert_eq!(
            workout_preset_manager.preset_in_creation,
            Some(PresetCreation::default())
        );
    }

    #[test]
    fn start_preset_creation_some() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        let preset_creation = PresetCreation {
            workout_preset: WorkoutPreset {
                name: "Preset1".to_string(),
                image: Default::default(),
                exercises: vec![],
            },
            edit_title: false,
            edit_image: false,
        };
        workout_preset_manager.preset_in_creation = Some(preset_creation.clone());
        workout_preset_manager.start_preset_creation();
        assert_eq!(
            workout_preset_manager.preset_in_creation,
            Some(preset_creation)
        );
    }

    #[test]
    fn add_preset_none() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        workout_preset_manager.add_preset();
        assert!(workout_preset_manager.presets.is_empty())
    }

    #[test]
    fn add_preset_some() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        let preset_creation = PresetCreation {
            workout_preset: WorkoutPreset {
                name: "Preset1".to_string(),
                image: Default::default(),
                exercises: vec![],
            },
            edit_title: false,
            edit_image: false,
        };
        workout_preset_manager.preset_in_creation = Some(preset_creation.clone());
        workout_preset_manager.add_preset();
        assert!(workout_preset_manager.preset_in_creation.is_none());
        assert!(!workout_preset_manager.presets.is_empty());
    }

    #[test]
    fn check_preset_preset_empty() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        workout_preset_manager.start_preset_creation();
        assert_eq!(
            workout_preset_manager.check_preset(),
            Err(PresetSafeError::PresetEmpty)
        );
    }
    #[test]
    fn check_preset_name_empty() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        let preset_creation = PresetCreation {
            workout_preset: WorkoutPreset {
                name: "".to_string(),
                image: Default::default(),
                exercises: vec!["Exercise".to_string()],
            },
            edit_title: false,
            edit_image: false,
        };
        workout_preset_manager.preset_in_creation = Some(preset_creation.clone());
        assert_eq!(
            workout_preset_manager.check_preset(),
            Err(PresetSafeError::NameEmpty)
        );
    }

    #[test]
    fn check_preset_empty() {
        let workout_preset_manager = WorkoutPresetManager::default();
        assert_eq!(
            workout_preset_manager.check_preset(),
            Err(PresetSafeError::NoPresetToSafe)
        );
    }

    #[test]
    fn check_preset_name_already_exists() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        let preset_creation = PresetCreation {
            workout_preset: WorkoutPreset {
                name: "123".to_string(),
                image: Default::default(),
                exercises: vec!["Exercise".to_string()],
            },
            edit_title: false,
            edit_image: false,
        };
        workout_preset_manager.preset_in_creation = Some(preset_creation.clone());
        workout_preset_manager.presets = vec![preset_creation.workout_preset];
        assert_eq!(
            workout_preset_manager.check_preset(),
            Err(PresetSafeError::NameAlreadyExists)
        )
    }

    #[test]
    fn check_preset_ok() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        let preset_creation = PresetCreation {
            workout_preset: WorkoutPreset {
                name: "123".to_string(),
                image: Default::default(),
                exercises: vec!["Exercise".to_string()],
            },
            edit_title: false,
            edit_image: false,
        };
        workout_preset_manager.preset_in_creation = Some(preset_creation.clone());
        assert!(workout_preset_manager.check_preset().is_ok());
    }

    #[test]
    fn move_preset_to_front() {
        let mut workout_preset_manager = WorkoutPresetManager::default();
        let preset_creation1 = PresetCreation {
            workout_preset: WorkoutPreset {
                name: "123".to_string(),
                image: Default::default(),
                exercises: vec!["Exercise".to_string()],
            },
            edit_title: false,
            edit_image: false,
        };
        let preset_creation2 = PresetCreation {
            workout_preset: WorkoutPreset {
                name: "123".to_string(),
                image: Default::default(),
                exercises: vec!["Exercise".to_string()],
            },
            edit_title: false,
            edit_image: false,
        };
        workout_preset_manager.presets = vec![
            preset_creation1.workout_preset,
            preset_creation2.workout_preset.clone(),
        ];
        workout_preset_manager.move_preset_to_front(&preset_creation2.workout_preset);
        assert_eq!(
            workout_preset_manager.presets[0],
            preset_creation2.workout_preset
        );
    }
}
