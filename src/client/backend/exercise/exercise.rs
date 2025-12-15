use std::collections::BTreeMap;
use chrono::NaiveDate;
use crate::client::backend::exercise::set::StrengthSet;
use crate::client::backend::exercise::weight::{ExerciseWeight, Kg};

pub struct Exercise {
    name: String,
    sets: BTreeMap<NaiveDate, Vec<StrengthSet>>
}
impl Exercise {
    pub fn new(name: String) -> Self {
        Self {
            name,
            sets: Default::default()
        }
    }

    fn all_time_lifted_weight(&self) -> Kg {
        let mut total_lifted_weight = 0.0;
        for (_day, sets_per_day) in &self.sets {
            for set in sets_per_day {
                total_lifted_weight += set.total_lifted_weight();
            }
        }
        total_lifted_weight
    }
    fn all_time_reps(&self) -> u64 {
        let mut total_reps: u64 = 0;
        for (_day, sets_per_day) in &self.sets {
            for set in sets_per_day {
                total_reps += set.reps;
            }
        }
        total_reps
    }
    fn all_time_sets(&self) -> u64 {
        let mut all_time_sets: u64 = 0;
        for (_day, sets_per_day) in &self.sets {
            all_time_sets += sets_per_day.len() as u64
        }
        all_time_sets
    }
    fn weight_personal_record(&self) -> Kg {
        let mut pr = 0.0;
        for (_day, sets_per_day) in &self.sets {
            for set in sets_per_day {
                if set.weight.to_kg() > pr {
                    pr = set.weight.to_kg();
                }
            }
        }
        pr
    }
    fn set_with_most_total_lifted_weight(&self) -> (Option<NaiveDate>, &StrengthSet) {
        let mut heaviest_set: (Option<NaiveDate>, &StrengthSet) =
            (None,
             &StrengthSet {
                 weight: ExerciseWeight::Kg(0.0),
                 reps: 0,
             });
        for (day, sets_per_day) in &self.sets {
            for set in sets_per_day {
                if set.total_lifted_weight() > heaviest_set.1.total_lifted_weight() {
                    heaviest_set.0 = Some(*day);
                    heaviest_set.1 = set;
                }
            }
        }
        heaviest_set
    }
}