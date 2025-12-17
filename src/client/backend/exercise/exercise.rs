use crate::client::backend::exercise::set::StrengthSet;
use crate::client::backend::exercise::weight::{ExerciseWeight, Kg};
use chrono::{Duration, NaiveDate};
use std::collections::BTreeMap;
use rand::Rng;

pub struct Exercise {
    pub name: String,
    pub sets: BTreeMap<NaiveDate, Vec<StrengthSet>>
}
impl Exercise {
    pub fn new(name: String) -> Self {
        Self {
            name,
            sets: Default::default()
        }
    }
    pub fn calculate_max_weight_per_day(&self) -> Vec<(NaiveDate, Kg)> {
        let mut results: Vec<(NaiveDate, Kg)> = vec![];
        for (date, sets) in &self.sets {
            let mut current_best_weight = 0.0;
            for set in sets {
                if set.weight > current_best_weight {
                   current_best_weight = set.weight;
                }
           }
            results.push((*date, current_best_weight))
        }
        results
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
    pub fn all_time_sets(&self) -> u64 {
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
                if set.weight > pr {
                    pr = set.weight;
                }
            }
        }
        pr
    }
    fn set_with_most_total_lifted_weight(&self) -> (Option<NaiveDate>, &StrengthSet) {
        let mut heaviest_set: (Option<NaiveDate>, &StrengthSet) =
            (None,
             &StrengthSet {
                 weight: 0.0,
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

pub fn generate_example_exercise(name: String, sets_on_different_days: usize, base_weight: Kg) -> Exercise {

    let mut exercise = Exercise::new(name);

    let mut cur_day = NaiveDate::from_ymd_opt(2025,1,1).unwrap();
    let mut weight = base_weight;
    let variation: f32 = 1.0;
    let mut rand = rand::rng();

    for _ in 0..sets_on_different_days{
        let random_number = rand.random_range(-1..=2);
         weight += variation * random_number as f32;
        exercise.sets.insert(cur_day,
                                  vec![
                                      StrengthSet::new(ExerciseWeight::Kg(weight), 6),
                                      StrengthSet::new(ExerciseWeight::Kg(weight-5.0), 10)
                                  ]);
        cur_day += Duration::days(1)
    };

    exercise
}