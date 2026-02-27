use crate::common::exercise_mod::set::{Reps, StrengthSet};
use crate::common::exercise_mod::weight::{ExerciseWeight, Kg};

pub type WorkoutCreate = Vec<ExerciseCreate>;
#[derive(Debug, Clone)]
pub struct ExerciseCreate {
    pub name: String,
    pub sets: Vec<StrengthSetCreate>,
}

impl ExerciseCreate {
    pub fn new(name: String) -> Self {
        ExerciseCreate {
            name,
            sets: Vec::new(),
        }
    }
    pub fn test_case(number: usize) -> Self {
        ExerciseCreate {
            name: "TestExerciseName".to_string(),
            sets: vec![
                StrengthSetCreate::new(ExerciseWeight::Kg(number as Kg), number as u32),
                StrengthSetCreate::new(ExerciseWeight::Kg(60 as Kg), 100),
                StrengthSetCreate::new(ExerciseWeight::Kg(60 as Kg), 100),
                StrengthSetCreate::new(ExerciseWeight::Kg(60 as Kg), 100),
            ],
        }
    }
}

pub struct ExerciseCreateString {
    pub sets: Vec<StrengthSetString>,
}

impl From<ExerciseCreate> for ExerciseCreateString {
    fn from(exercise_create: ExerciseCreate) -> Self {
        let mut resulting_sets = Vec::new();
        for creation_set in exercise_create.sets {
            resulting_sets.push(creation_set.into())
        }
        ExerciseCreateString {
            sets: resulting_sets,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StrengthSetCreate {
    pub reps: Reps,
    pub weight: Kg,
}
impl StrengthSetCreate {
    pub fn new(weight: ExerciseWeight, reps: Reps) -> Self {
        let kg = weight.to_kg();
        StrengthSetCreate { weight: kg, reps }
    }
}
pub struct StrengthSetString {
    pub reps: String,
    pub kg: String,
}

impl From<StrengthSetCreate> for StrengthSetString {
    fn from(strength_set: StrengthSetCreate) -> Self {
        StrengthSetString {
            reps: strength_set.reps.to_string(),
            kg: strength_set.weight.to_string(),
        }
    }
}

impl From<StrengthSet> for StrengthSetCreate {
    fn from(val: StrengthSet) -> Self {
        StrengthSetCreate {
            reps: val.reps,
            weight: val.weight,
        }
    }
}
