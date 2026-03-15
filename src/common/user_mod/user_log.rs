use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub type Log = Vec<(NaiveDate, f32)>;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserLog {
    pub weight_log: Log,
    pub water_log: Log,
    pub step_log: Log,
    pub sleep_log: Log,
}