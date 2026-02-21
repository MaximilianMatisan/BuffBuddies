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
    pub fn get_max_goal(&self) -> f32 {
        match self {
            GoalType::WeeklyWorkouts => 20.0,
            GoalType::Weight => 200.0,
            GoalType::Water => 10.0,
            GoalType::Steps => 100000.0,
            GoalType::Sleep => 20.0,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserGoals {
    pub weekly_workouts: f32,
    pub weight: Kg,
    pub water: f32,
    pub steps: f32,
    pub sleep: f32,
}

impl Default for UserGoals {
    fn default() -> Self {
        UserGoals {
            weekly_workouts: 4.0,
            weight: 60.0,
            water: 3.0,
            steps: 10000.0,
            sleep: 9.0,
        }
    }
}

impl UserGoals {
    pub fn get_goal_by_type_mut(&mut self, goal_type: &GoalType) -> &mut f32 {
        match goal_type {
            GoalType::WeeklyWorkouts => &mut self.weekly_workouts,
            GoalType::Weight => &mut self.weight,
            GoalType::Water => &mut self.water,
            GoalType::Steps => &mut self.steps,
            GoalType::Sleep => &mut self.sleep,
        }
    }
    pub fn update_user_goals(&mut self, goal_type: &GoalType, increment: bool) {
        let value = self.get_goal_by_type_mut(goal_type);
        let step = goal_type.get_increment_decrement_step();
        let min = goal_type.get_increment_decrement_step();
        let max = goal_type.get_max_goal();

        safe_increment_decrement_f32(value, step, min, max, increment);
    }
}

pub fn safe_increment_decrement_f32(
    value: &mut f32,
    step: f32,
    min: f32,
    max: f32,
    increment: bool,
) {
    if increment {
        *value = (*value + step).min(max);
    } else {
        *value = (*value - step).max(min);
    }
}
