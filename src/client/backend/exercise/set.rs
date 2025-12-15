use crate::client::backend::exercise::weight::{ExerciseWeight, Kg};

pub struct StrengthSet {
    pub(crate) weight: Kg,
    pub(crate) reps: u64
}
impl StrengthSet {
    pub fn new(weight: ExerciseWeight, reps: u64) -> Self {
        let kg = weight.to_kg();
        StrengthSet {
            weight: kg,
            reps
        }
    }
    pub fn total_lifted_weight(&self) -> Kg {
        self.weight * self.reps as Kg
    }
}
