use strum_macros::{Display, EnumString};

pub struct ExerciseInfo {
    name: String,
    force: ExerciseForce,
    level: ExerciseLevel,
    equipment: Option<ExerciseEquipment>,
    primary_muscle: Muscle,
    instructions: String,
    category: ExerciseCategory,
}
#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
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
    Chest,
    LowerBack,
    Traps,
    MiddleBack,
    Lats,
    Neck,
}

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ExerciseForce {
    Pull,
    Push,
}

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ExerciseLevel {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ExerciseMechanic {
    Compound,
    Isolation,
}

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ExerciseEquipment {
    Body,
    Machine,
    Kettlebells,
    Dumbbell,
    Cable,
    Barbell,
    Bands,
    MedicineBall,
    ExerciseBall,
    EzCurlBar,
    FoamRoll,
}

#[derive(EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ExerciseCategory {
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