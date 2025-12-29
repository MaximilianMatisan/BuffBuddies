use crate::client::backend::exercise::weight::{ExerciseWeight, Kg};

pub type Reps = u32;
#[derive(Debug, Clone)]
pub struct StrengthSet {
    pub(crate) weight: Kg,
    pub(crate) reps: Reps,
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
