use crate::client::backend::exercise_create::StrengthSetCreate;
use crate::common::exercise_mod::general_exercise::Id;
use crate::common::exercise_mod::weight::{ExerciseWeight, Kg};
use serde::{Deserialize, Serialize};

pub type Reps = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrengthSet {
    pub workout_id: Id,
    pub weight: Kg,
    pub reps: Reps,
}
impl StrengthSet {
    pub fn new(workout_id: Id, weight: ExerciseWeight, reps: Reps) -> Self {
        let kg = weight.to_kg();
        StrengthSet {
            workout_id,
            weight: kg,
            reps,
        }
    }
    pub fn from_strength_set_create(
        strength_set_create: &StrengthSetCreate,
        workout_id: Id,
    ) -> Self {
        StrengthSet {
            workout_id,
            weight: strength_set_create.weight,
            reps: strength_set_create.reps,
        }
    }
    pub fn total_lifted_weight(&self) -> Kg {
        self.weight * self.reps as Kg
    }
}

impl Default for StrengthSet {
    fn default() -> Self {
        StrengthSet {
            workout_id: 0,
            weight: 0.0,
            reps: 0,
        }
    }
}
