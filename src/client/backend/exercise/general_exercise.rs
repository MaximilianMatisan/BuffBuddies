//Currently only as template for extracting exercise data from the db in future work
#[allow(dead_code)]
pub struct GeneralExerciseInfo {
    name: String,
    force: Option<ExerciseForce>,
    level: ExerciseLevel,
    equipment: Option<ExerciseEquipment>,
    primary_muscle: Muscle,
    instructions: String,
    category: ExerciseCategory,
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
