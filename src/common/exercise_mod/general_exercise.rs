use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub type Id = u32;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GeneralExerciseInfo {
    pub id: Id,
    pub name: String,
    pub force: ExerciseForce,
    /// Difficulty of the exercise
    pub level: ExerciseLevel,
    pub equipment: ExerciseEquipment,
    /// Primarily targeted muscle
    pub primary_muscle: Muscle,
    pub instructions: String,
    pub category: ExerciseCategory,
}
impl GeneralExerciseInfo {
    pub fn test_obj() -> Self {
        GeneralExerciseInfo {
            id: 0,
            name: "Test exercise".to_string(),
            force: ExerciseForce::Push,
            level: ExerciseLevel::Beginner,
            equipment: ExerciseEquipment::Body,
            primary_muscle: Muscle::Abdominals,
            instructions: "This is a test exercise!".to_string(),
            category: ExerciseCategory::Strength,
        }
    }
}
#[derive(Display, EnumString, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum Muscle {
    Abdominals,
    Hamstrings,
    Calves,
    Shoulders,
    Adductors,
    Glutes,
    Quadriceps,
    Biceps,
    Forearms,
    Abductors,
    Triceps,
    #[default]
    Chest,
    #[strum(serialize = "lower back")]
    LowerBack,
    Traps,
    #[strum(serialize = "middle back")]
    MiddleBack,
    Lats,
    Neck,
}

#[derive(Debug, Display, EnumString, Default, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum ExerciseForce {
    #[default]
    Pull,
    Push,
    Static,
}

#[derive(Display, EnumString, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum ExerciseLevel {
    #[default]
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Debug, Display, EnumString, Default, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum ExerciseEquipment {
    #[default]
    #[strum(serialize = "body only")]
    Body,
    Machine,
    Kettlebells,
    Dumbbell,
    Cable,
    Barbell,
    Bands,
    #[strum(serialize = "medicine ball")]
    MedicineBall,
    #[strum(serialize = "exercise ball")]
    ExerciseBall,
    #[strum(serialize = "e-z curl bar")]
    EzCurlBar,
    FoamRoll,
    Other,
}

#[derive(Display, EnumString, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
#[strum(ascii_case_insensitive)]
pub enum ExerciseCategory {
    #[default]
    Strength,
    Stretching,
    Plyometrics,
    Strongman,
    Powerlifting,
    Cardio,
    OlympicWeightlifting,
    Crossfit,
    WeightedBodyweight,
    AssistedBodyweight,
}
