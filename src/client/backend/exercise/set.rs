use crate::client::backend::exercise::weight::{ExerciseWeight, Kg};

pub struct StrengthSet {
    pub(crate) weight: ExerciseWeight,
    pub(crate) reps: u64
}
impl StrengthSet {
    pub fn new(weight: ExerciseWeight, reps: u64) -> Self {
        StrengthSet {
            weight,
            reps
        }
    }
    pub fn total_lifted_weight(&self) -> Kg {
        self.weight.to_kg() * self.reps as Kg
    }
}
