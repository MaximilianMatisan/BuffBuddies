use std::cmp::Ordering;

pub type Kg = f32;
pub fn kg_to_string(kg: Kg) -> String {
    format!("{kg} kg")
}
pub enum ExerciseWeight {
    Bodyweight,
    BodyweightPlusKg(Kg),
    Kg(Kg)
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
            ExerciseWeight::Bodyweight => user_weight,
            ExerciseWeight::BodyweightPlusKg(kg) => user_weight + kg,
            ExerciseWeight::Kg(kg) => *kg
        }
    }
}