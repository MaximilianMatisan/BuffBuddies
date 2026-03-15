use crate::common::user_mod::user_goals::GoalType;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

pub type Log = Vec<(NaiveDate, f32)>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserLog {
    pub weight_log: Log,
    pub water_log: Log,
    pub step_log: Log,
    pub sleep_log: Log,
}

impl UserLog {
    pub fn get_log_by_goal_type_mut(&mut self, goal_type: &GoalType) -> Option<&mut Log> {
        Some(match goal_type {
            GoalType::Weight => &mut self.weight_log,
            GoalType::Water => &mut self.water_log,
            GoalType::Steps => &mut self.step_log,
            GoalType::Sleep => &mut self.sleep_log,
            GoalType::WeeklyWorkouts => return None,
        })
    }
    pub fn update_log(&mut self, goal_type: &GoalType, new_entry: f32) -> Result<(), &str> {
        let today = Local::now().date_naive();

        let log = self
            .get_log_by_goal_type_mut(goal_type)
            .ok_or("Goal type doesn't have a log")?;

        if let Some((_, value)) = log.iter_mut().find(|(date, _)| *date == today) {
            *value = new_entry;
        } else {
            log.push((today, new_entry)); // push to the end as today should be the most recent day
        }

        Ok(())
    }
}
