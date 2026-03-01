use crate::common::exercise_mod::set::{Reps, StrengthSet};
use crate::common::exercise_mod::weight::{ExerciseWeight, Kg};

pub type WorkoutCreate = Vec<ExerciseCreate>;
///struct that is used to save a workout during creation
#[derive(Debug, Clone, PartialEq)]
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
///needed by iced to show and edit the current workout in creation
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

///struct that is used to represent strength sets during creation of a workout
#[derive(Debug, Clone, PartialEq)]
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
///needed for iced to show and edit sets
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_strength_set() {
        let strength_set_string = StrengthSetString::from(StrengthSetCreate {
            weight: 50.0,
            reps: 10,
        });
        assert_eq!(strength_set_string.kg, "50".to_string());
        assert_eq!(strength_set_string.reps, "10".to_string())
    }
}
