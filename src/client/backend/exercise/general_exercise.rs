//Currently only as template for extracting exercise data from the db in future work
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
#[allow(dead_code)]
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

#[allow(dead_code)]
pub enum ExerciseForce {
    Pull,
    Push,
}

#[allow(dead_code)]
pub enum ExerciseLevel {
    Beginner,
    Intermediate,
    Expert,
}

#[allow(dead_code)]
pub enum ExerciseMechanic {
    Compound,
    Isolation,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
