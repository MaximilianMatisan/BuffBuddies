use strum_macros::Display;

pub struct GeneralExerciseInfo {
    pub id: u32,
    pub name: String,
    pub force: Option<ExerciseForce>,
    pub level: ExerciseLevel,
    pub equipment: Option<ExerciseEquipment>,
    pub primary_muscle: Muscle,
    pub instructions: String,
    pub category: ExerciseCategory,
}
#[derive(Display)]
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

#[derive(Display)]
pub enum ExerciseForce {
    Pull,
    Push,
}

#[derive(Display)]
pub enum ExerciseLevel {
    Beginner,
    Intermediate,
    Expert,
}

#[derive(Display)]
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

#[derive(Display)]
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
