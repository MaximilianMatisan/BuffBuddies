use crate::common::exercise_mod::weight::{ExerciseWeight, Kg};
use serde::{Deserialize, Serialize};

pub type Reps = u32;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthSet {
    pub weight: Kg,
    pub reps: Reps,
}
impl StrengthSet {
    pub fn new(weight: ExerciseWeight, reps: Reps) -> Self {
        let kg = weight.to_kg();
        StrengthSet { weight: kg, reps }
    }
    pub fn total_lifted_weight(&self) -> Kg {
        self.weight * self.reps as Kg
    }
}

impl Default for StrengthSet {
    fn default() -> Self {
        StrengthSet {
            weight: 0.0,
            reps: 0,
        }
    }
}
