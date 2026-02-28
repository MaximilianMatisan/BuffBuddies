use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutPreset {
    pub name: String,
    pub image: PresetImage,
    pub exercises: Vec<String>,
}

impl Default for WorkoutPreset {
    fn default() -> Self {
        WorkoutPreset {
            name: "Preset Title".to_string(),
            image: PresetImage::Default,
            exercises: Vec::default(),
        }
    }
}

#[derive(Default)]
pub struct PresetCreation {
    pub workout_preset: WorkoutPreset,
    pub edit_title: bool,
    pub edit_image: bool,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, EnumIter)]
pub enum PresetImage {
    #[default]
    Default,
    Bench,
    Pullup,
    Squats,
    Running,
}

impl PresetImage {
    pub fn get_file_path(&self, mascot: &Mascot) -> String {
        //this leave the option to add different images for different mascots later, it's just set to default for now tho
        let mascot_name_lowered = mascot.get_name().to_lowercase();
        let base_path = "assets/images/preset_images/";
        let image_endings = match self {
            PresetImage::Default => "default_preset.png",
            PresetImage::Bench => "_bench.png",
            PresetImage::Pullup => "_pullup.png",
            PresetImage::Squats => "_squat.png",
            PresetImage::Running => "_run.png",
        };
        if let PresetImage::Default = self {
            format!("{base_path}{image_endings}")
        } else {
            format!("{base_path}{mascot_name_lowered}{image_endings}")
        }
    }
}
