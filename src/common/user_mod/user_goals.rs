use crate::common::exercise_mod::weight::Kg;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGoals {
    pub weekly_workouts: u32,
    pub weight: Kg,
    pub water: f32,
    pub steps: u32,
    pub sleep: f32,
}

impl Default for UserGoals {
    fn default() -> Self {
        UserGoals {
            weekly_workouts: 4,
            weight: 60.0,
            water: 2000.0,
            steps: 10000,
            sleep: 9.0,
        }
    }
}
