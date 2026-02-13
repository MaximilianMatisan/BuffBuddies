use serde::{Deserialize, Serialize};

use crate::common::exercise_mod::general_exercise::Id;


#[derive(Debug, Serialize, Deserialize)]
pub struct WorkoutPreset {
    pub exercises: Vec<Id>,
}
