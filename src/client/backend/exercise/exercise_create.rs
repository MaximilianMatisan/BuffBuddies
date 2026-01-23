use crate::client::backend::exercise::set::StrengthSet;
use crate::client::backend::exercise::weight::{ExerciseWeight, Kg};

#[derive(Clone)]
pub struct ExerciseCreate {
    pub name: String,
    pub sets: Vec<StrengthSet>,
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
                StrengthSet::new(ExerciseWeight::Kg(number as Kg), number as u32),
                StrengthSet::new(ExerciseWeight::Kg(60 as Kg), 100),
                StrengthSet::new(ExerciseWeight::Kg(60 as Kg), 100),
                StrengthSet::new(ExerciseWeight::Kg(60 as Kg), 100),
            ],
        }
    }
}

pub struct ExerciseCreateString {
    pub sets: Vec<StrengthSetString>,
}

impl From<ExerciseCreate> for ExerciseCreateString {
    fn from(exercise_create: ExerciseCreate) -> Self {
        let mut sets = Vec::new();
        for set in exercise_create.sets {
            sets.push(set.into())
        }
        ExerciseCreateString { sets }
    }
}

pub struct StrengthSetString {
    pub reps: String,
    pub kg: String,
}

impl From<StrengthSet> for StrengthSetString {
    fn from(strength_set: StrengthSet) -> Self {
        StrengthSetString {
            reps: strength_set.reps.to_string(),
            kg: strength_set.weight.to_string(),
        }
    }
}
