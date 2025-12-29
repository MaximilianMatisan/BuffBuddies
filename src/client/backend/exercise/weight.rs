use std::cmp::Ordering;

pub type Kg = f64;
pub enum ExerciseWeight {
    Bodyweight,
    BodyweightPlusKg(Kg),
    Kg(Kg),
}
impl PartialEq for ExerciseWeight {
    fn eq(&self, other: &Self) -> bool {
        self.to_kg() == other.to_kg()
    }
}
impl PartialOrd for ExerciseWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_kg().partial_cmp(&other.to_kg())
    }
}
impl ExerciseWeight {
    pub fn to_kg(&self) -> Kg {
        let user_weight = 0.0; //TODO get user weight
        match self {
            ExerciseWeight::Bodyweight => round_to_two_decimals(user_weight),
            ExerciseWeight::BodyweightPlusKg(kg) => round_to_two_decimals(user_weight + kg),
            ExerciseWeight::Kg(kg) => round_to_two_decimals(*kg),
        }
    }
}
pub fn round_to_two_decimals(weight: Kg) -> Kg {
    (weight * 100.0).round() / 100.0
}
