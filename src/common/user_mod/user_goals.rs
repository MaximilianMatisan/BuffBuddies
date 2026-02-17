use crate::client::gui::bb_theme::text_format::{hours_to_string, kg_to_string, liter_to_string};
use crate::common::exercise_mod::weight::Kg;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Display, EnumIter)]
pub enum GoalType {
    #[strum(to_string = "Weekly workouts")]
    WeeklyWorkouts,

    Weight,
    Water,
    Steps,
    Sleep,
}
impl GoalType {
    pub fn get_formatted_user_goal_strings(&self, user_goals: &UserGoals) -> String {
        match self {
            GoalType::WeeklyWorkouts => user_goals.weekly_workouts.to_string(),
            GoalType::Weight => kg_to_string(user_goals.weight),
            GoalType::Water => liter_to_string(user_goals.water),
            GoalType::Steps => user_goals.steps.to_string(),
            GoalType::Sleep => hours_to_string(user_goals.sleep),
        }
    }
    pub fn get_increment_decrement_step(&self) -> f32 {
        match self {
            GoalType::WeeklyWorkouts => 1.0,
            GoalType::Weight => 0.5,
            GoalType::Water => 0.5,
            GoalType::Steps => 500.0,
            GoalType::Sleep => 0.5,
        }
    }
}
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
