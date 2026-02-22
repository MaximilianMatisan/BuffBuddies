use crate::common::exercise_mod::general_exercise::GeneralExerciseInfo;
use crate::common::exercise_mod::set::{Reps, StrengthSet};
use crate::common::exercise_mod::weight::{ExerciseWeight, Kg};
use chrono::{Duration, Local, NaiveDate};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

pub type ExerciseDataPoints = Vec<(NaiveDate, Kg)>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Exercise {
    pub general_exercise_info: GeneralExerciseInfo,
    /// The completed sets of an exercise by the logged-in user are stored here.
    /// They are located in a BTreemap, as its insert function allows us to guarantee that the nodes
    /// of the tree are always sorted by primary key when iterating.  
    pub sets: BTreeMap<NaiveDate, Vec<StrengthSet>>,
}

/// Only show the exercise name when printing an exercise
impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.general_exercise_info.name)
    }
}

impl Exercise {
    pub fn new(general_exercise_info: GeneralExerciseInfo) -> Self {
        Self {
            general_exercise_info,
            sets: Default::default(),
        }
    }

    pub fn is_tracked(&self) -> bool {
        !self.sets.is_empty()
    }

    /// This function calculates the maximum weight of a set for each tracked day.
    pub fn calculate_max_weight_per_day(&self) -> ExerciseDataPoints {
        let mut results: ExerciseDataPoints = vec![];
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
    pub fn all_time_lifted_weight(&self) -> Kg {
        let mut total_lifted_weight = 0.0;
        for sets_per_day in self.sets.values() {
            for set in sets_per_day {
                total_lifted_weight += set.total_lifted_weight();
            }
        }
        total_lifted_weight
    }
    /// Calculates the sum of all reps across all strength sets
    pub fn all_time_reps(&self) -> Reps {
        let mut total_reps: Reps = 0;
        for sets_per_day in self.sets.values() {
            for set in sets_per_day {
                total_reps += set.reps;
            }
        }
        total_reps
    }
    /// Calculates the sum of all tracked sets
    pub fn all_time_sets(&self) -> u64 {
        let mut all_time_sets: u64 = 0;
        for sets_per_day in self.sets.values() {
            all_time_sets += sets_per_day.len() as u64
        }
        all_time_sets
    }
    /// Calculates the highest tracked weight of an exercise
    /// Returns 0 if no StrengthSet is tracked
    pub fn weight_personal_record(&self) -> Kg {
        let mut pr = 0.0;
        for sets_per_day in self.sets.values() {
            for set in sets_per_day {
                if set.weight > pr {
                    pr = set.weight;
                }
            }
        }
        pr
    }
    /// Calculates the max of reps * weight across all StrengthSets
    /// Returns (<Date of today>, 0.0) if no StrengthSet is tracked
    pub fn set_with_most_total_lifted_weight(&self) -> (NaiveDate, Kg) {
        let mut heaviest_set: (NaiveDate, Kg) = (Local::now().date_naive(), 0.0);
        for (day, sets_per_day) in &self.sets {
            for set in sets_per_day {
                let cur_total_lifted_weight = set.total_lifted_weight();
                if cur_total_lifted_weight > heaviest_set.1 {
                    heaviest_set.0 = *day;
                    heaviest_set.1 = cur_total_lifted_weight;
                }
            }
        }
        heaviest_set
    }
}
pub fn get_weight_milestones(start_number: u32, end_number: u32, steps: u32) -> Vec<u32> {
    let mut milestones = vec![];

    if steps == 0 || end_number < start_number {
        return milestones;
    }
    if steps == 1 {
        return vec![start_number];
    }
    if steps >= end_number - start_number {
        for i in start_number..=end_number {
            milestones.push(i);
        }
        return milestones;
    }

    let range = end_number - start_number;

    for step in 0..steps {
        milestones.push(start_number + (range * step) / (steps - 1));
    }
    milestones
}

pub fn generate_example_exercise(
    name: GeneralExerciseInfo,
    sets_on_different_days: usize,
    base_weight: Kg,
) -> Exercise {
    let mut exercise = Exercise::new(name);

    let mut cur_day = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let mut weight = base_weight;
    let variation: Kg = 1.1;
    let mut rand = rand::rng();

    for _ in 0..sets_on_different_days {
        let random_number = rand.random_range(-1..=2);
        weight += variation * random_number as Kg;
        exercise.sets.insert(
            cur_day,
            vec![
                StrengthSet::new(ExerciseWeight::Kg(weight), 10),
                StrengthSet::new(ExerciseWeight::Kg(weight), 10),
            ],
        );
        cur_day += Duration::days(1)
    }
    exercise
}
pub fn get_minimum_weight(data_points: &ExerciseDataPoints) -> Option<f32> {
    let temp_min = data_points.iter().map(|(_, kg)| (*kg * 10.0) as u32).min();

    temp_min.map(|value| value as f32 / 10.0)
}
pub fn get_maximum_weight(data_points: &ExerciseDataPoints) -> Option<f32> {
    let temp_min = data_points.iter().map(|(_, kg)| (*kg * 10.0) as u32).max();
    temp_min.map(|value| value as f32 / 10.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::exercise_mod::set::Reps;

    const CUSTOM_TRACKED_DAYS: u32 = 45;
    const CUSTOM_SETS_PER_DAY: u32 = 10;
    const CUSTOM_WEIGHT_PER_SET: Kg = 80.0;
    const CUSTOM_REPS_PER_SET: Reps = 12;
    fn create_custom_exercise(
        first_tracked_day: NaiveDate,
        tracked_days: u32,
        sets_per_day: u32,
        weight_per_set: Kg,
        reps_per_set: Reps,
    ) -> Exercise {
        let mut exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        let mut date = first_tracked_day;
        for _ in 0..tracked_days {
            for _ in 0..sets_per_day {
                exercise
                    .sets
                    .entry(date)
                    .or_default()
                    .push(StrengthSet::new(
                        ExerciseWeight::Kg(weight_per_set),
                        reps_per_set,
                    ))
            }
            date += Duration::days(1);
        }
        exercise
    }
    fn custom_exercise_preset() -> Exercise {
        create_custom_exercise(
            Local::now().date_naive(),
            CUSTOM_TRACKED_DAYS,
            CUSTOM_SETS_PER_DAY,
            CUSTOM_WEIGHT_PER_SET,
            CUSTOM_REPS_PER_SET,
        )
    }

    const MOCK_DATES: [NaiveDate; 2] = [
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
    ];
    const MOCK_DAYS: usize = 2;
    const MOCK_SETS_PER_DAY: usize = 3;
    const MOCK_BEST_WEIGHT_DAY_ONE: Kg = 67.5;
    const MOCK_BEST_WEIGHT_DAY_TWO: Kg = 60.0;
    const MOCK_WEIGHT: [[Kg; MOCK_SETS_PER_DAY]; MOCK_DAYS] = [
        [50.0, 57.5, MOCK_BEST_WEIGHT_DAY_ONE],
        [55.0, MOCK_BEST_WEIGHT_DAY_TWO, 57.5],
    ];
    const MOCK_REPS: [[Reps; MOCK_SETS_PER_DAY]; MOCK_DAYS] = [[13, 12, 10], [15, 15, 11]];
    fn mock_exercise() -> Exercise {
        let mut exercise = Exercise::new(GeneralExerciseInfo::test_obj());

        for day in 0..MOCK_DAYS {
            for set in 0..MOCK_SETS_PER_DAY {
                exercise
                    .sets
                    .entry(MOCK_DATES[day])
                    .or_default()
                    .push(StrengthSet::new(
                        ExerciseWeight::Kg(MOCK_WEIGHT[day][set]),
                        MOCK_REPS[day][set],
                    ));
            }
        }
        exercise
    }

    //TESTS
    #[test]
    fn test_create_custom_exercise() {
        let custom_exercise = custom_exercise_preset();
        assert_eq!(custom_exercise.sets.len() as u32, CUSTOM_TRACKED_DAYS);
        for (_day, sets) in custom_exercise.sets {
            assert_eq!(sets.len() as u32, CUSTOM_SETS_PER_DAY);
        }
        //let mock_exercise = mock_exercise();
        //println!("{:?}",mock_exercise)
    }
    #[test]
    fn test_calculate_max_weight_per_day() {
        let empty_exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        assert_eq!(empty_exercise.calculate_max_weight_per_day(), vec![]);

        let custom_exercise = custom_exercise_preset();
        let custom_exercise_data_points = custom_exercise.calculate_max_weight_per_day();
        assert_eq!(
            custom_exercise_data_points.len() as u32,
            CUSTOM_TRACKED_DAYS
        );
        for (_, kg) in custom_exercise_data_points {
            assert_eq!(kg, CUSTOM_WEIGHT_PER_SET);
        }
        let mock_exercise_data_points = mock_exercise().calculate_max_weight_per_day();

        assert_eq!(
            mock_exercise_data_points,
            vec![
                (MOCK_DATES[0], MOCK_BEST_WEIGHT_DAY_ONE),
                (MOCK_DATES[1], MOCK_BEST_WEIGHT_DAY_TWO)
            ]
        );
    }
    #[test]
    fn test_all_time_lifted_weight() {
        let empty_exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        assert_eq!(empty_exercise.all_time_lifted_weight(), 0.0);

        let custom_exercise = custom_exercise_preset();
        let real_custom_all_time_lifted_weight = CUSTOM_TRACKED_DAYS as Kg
            * CUSTOM_SETS_PER_DAY as Kg
            * CUSTOM_WEIGHT_PER_SET
            * CUSTOM_REPS_PER_SET as Kg;
        assert_eq!(
            custom_exercise.all_time_lifted_weight(),
            real_custom_all_time_lifted_weight
        );

        let mock_exercise = mock_exercise();
        let mut real_mock_all_time_lifted_weight: Kg = 0.0;
        for day in 0..MOCK_DAYS {
            for set in 0..MOCK_SETS_PER_DAY {
                real_mock_all_time_lifted_weight +=
                    MOCK_REPS[day][set] as Kg * MOCK_WEIGHT[day][set]
            }
        }
        assert_eq!(
            mock_exercise.all_time_lifted_weight(),
            real_mock_all_time_lifted_weight
        );
    }
    #[test]
    fn test_all_time_reps() {
        let empty_exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        assert_eq!(empty_exercise.all_time_reps(), 0);

        let custom_exercise = custom_exercise_preset();
        let real_custom_all_time_reps =
            CUSTOM_TRACKED_DAYS * CUSTOM_SETS_PER_DAY * CUSTOM_REPS_PER_SET;
        assert_eq!(custom_exercise.all_time_reps(), real_custom_all_time_reps);

        let mock_exercise = mock_exercise();
        let real_mock_all_time_reps = MOCK_REPS
            .iter()
            .fold(0, |acc, x| acc + (x.iter().sum::<Reps>()));

        assert_eq!(mock_exercise.all_time_reps(), real_mock_all_time_reps)
    }
    #[test]
    fn test_all_time_sets() {
        let empty_exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        assert_eq!(empty_exercise.all_time_sets(), 0);

        let custom_exercise = custom_exercise_preset();
        let real_custom_all_time_sets = (CUSTOM_TRACKED_DAYS * CUSTOM_SETS_PER_DAY) as u64;
        assert_eq!(custom_exercise.all_time_sets(), real_custom_all_time_sets);

        let mock_exercise = mock_exercise();
        let real_mock_exercise_all_time_sets = (MOCK_DAYS * MOCK_SETS_PER_DAY) as u64;
        assert_eq!(
            mock_exercise.all_time_sets(),
            real_mock_exercise_all_time_sets
        );
    }
    #[test]
    fn test_weight_personal_record() {
        let empty_exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        assert_eq!(empty_exercise.weight_personal_record(), 0.0);

        let custom_exercise = custom_exercise_preset();
        assert_eq!(
            custom_exercise.weight_personal_record(),
            CUSTOM_WEIGHT_PER_SET
        );

        let mock_exercise = mock_exercise();
        let real_mock_weight_personal_record =
            MOCK_BEST_WEIGHT_DAY_ONE.max(MOCK_BEST_WEIGHT_DAY_TWO);
        assert_eq!(
            mock_exercise.weight_personal_record(),
            real_mock_weight_personal_record
        );
    }
    #[test]
    fn test_set_with_most_total_lifted_weight() {
        let empty_exercise = Exercise::new(GeneralExerciseInfo::test_obj());
        assert_eq!(
            empty_exercise.set_with_most_total_lifted_weight(),
            (Local::now().date_naive(), 0.0)
        );

        let custom_exercise = custom_exercise_preset();
        let real_custom_most_total_lifted_weight_in_set =
            CUSTOM_WEIGHT_PER_SET * CUSTOM_REPS_PER_SET as Kg;
        assert_eq!(
            custom_exercise.set_with_most_total_lifted_weight(),
            (
                Local::now().date_naive(),
                real_custom_most_total_lifted_weight_in_set
            )
        );

        let mock_exercise = mock_exercise();
        //DAY 1:
        // set1: 650 / set2: 690 / set3: 675
        //DAY 2:
        // set1: 825 / set2: 900 / set3: 632.5
        let real_mock: Kg = 900.0;
        assert_eq!(
            mock_exercise.set_with_most_total_lifted_weight(),
            (MOCK_DATES[1], real_mock)
        );
    }
    #[test]
    fn test_get_weight_milestones() {
        let illegal_range = get_weight_milestones(900, 500, 5);
        assert_eq!(illegal_range, Vec::<u32>::new());

        let start_equals_end = get_weight_milestones(10, 10, 5);
        assert_eq!(start_equals_end, vec![10]);

        let one_step = get_weight_milestones(54, 99, 1);
        assert_eq!(one_step, vec![54]);

        let zero_step = get_weight_milestones(50, 200, 0);
        assert_eq!(zero_step, Vec::<u32>::new());

        let simple_two_step = get_weight_milestones(10, 20, 2);
        assert_eq!(simple_two_step, vec![10, 20]);

        let simple_three_step = get_weight_milestones(0, 100, 3);
        assert_eq!(simple_three_step, vec![0, 50, 100]);

        let simple_ten_step = get_weight_milestones(0, 12393, 85);
        assert_eq!(simple_ten_step.len(), 85);
        assert_eq!(*simple_ten_step.first().unwrap(), 0);
        assert_eq!(*simple_ten_step.last().unwrap(), 12393);
    }
}
