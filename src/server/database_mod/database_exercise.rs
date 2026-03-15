use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::general_exercise::{
    ExerciseCategory, ExerciseEquipment, ExerciseForce, ExerciseLevel, GeneralExerciseInfo, Id,
    Muscle,
};
use crate::common::exercise_mod::set::StrengthSet;
use crate::common::exercise_mod::weight::Kg;
use crate::server::routes::workout::ExerciseJson;
use chrono::NaiveDate;
use sqlx::{Row, SqlitePool};
use std::collections::BTreeMap;
use std::str::FromStr;
use crate::server::database_mod::database_utils;

pub async fn get_general_exercise_info(
    pool: &SqlitePool,
    exercise_id: i64,
) -> Result<GeneralExerciseInfo, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM exercise WHERE id = ?")
        .bind(exercise_id)
        .fetch_one(pool)
        .await?;

    Ok(GeneralExerciseInfo {
        id: row.get("id"),
        name: row.get("name"),
        force: ExerciseForce::from_str(row.get("exercise_force_name"))
            .unwrap_or(ExerciseForce::Push),
        level: ExerciseLevel::from_str(row.get("exercise_level_name"))
            .unwrap_or(ExerciseLevel::Beginner),
        equipment: ExerciseEquipment::from_str(row.get("exercise_equipment_name"))
            .unwrap_or(ExerciseEquipment::Body),
        primary_muscle: Muscle::from_str(row.get("muscle_name")).unwrap_or(Muscle::Chest),
        instructions: row.get("instructions"),
        category: ExerciseCategory::from_str(row.get("exercise_category_name"))
            .unwrap_or(ExerciseCategory::Strength),
    })
}
#[allow(dead_code)]
pub async fn get_exercise_name(pool: &SqlitePool, exercise_id: i64) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT name FROM exercise WHERE id = ?")
        .bind(exercise_id)
        .fetch_one(pool)
        .await?;

    let exercise_name = row.get("name");
    Ok(exercise_name)
}
#[allow(dead_code)]
pub async fn get_exercise_force_name(
    pool: &SqlitePool,
    exercise_id: i64,
) -> Result<ExerciseForce, sqlx::Error> {
    let row = sqlx::query("SELECT exercise_force_name FROM exercise WHERE id = ?")
        .bind(exercise_id)
        .fetch_one(pool)
        .await?;

    let force_name: String = row.get("exercise_force_name");
    let exercise_force = ExerciseForce::from_str(&force_name).unwrap_or(ExerciseForce::Push);

    Ok(exercise_force)
}

#[allow(dead_code)]
pub async fn get_exercise_level_name(
    pool: &SqlitePool,
    exercise_id: i64,
) -> Result<ExerciseLevel, sqlx::Error> {
    let row = sqlx::query("SELECT exercise_level_name FROM exercise WHERE id = ?")
        .bind(exercise_id)
        .fetch_one(pool)
        .await?;

    let level_name: String = row.get("exercise_level_name");
    let exercise_level = ExerciseLevel::from_str(&level_name).unwrap_or(ExerciseLevel::Beginner);

    Ok(exercise_level)
}
#[allow(dead_code)]
pub async fn get_exercise_equipment_name(
    pool: &SqlitePool,
    exercise_id: i64,
) -> Result<ExerciseEquipment, sqlx::Error> {
    let row = sqlx::query("SELECT exercise_equipment_name FROM exercise WHERE id = ?")
        .bind(exercise_id)
        .fetch_one(pool)
        .await?;

    let equipment_name: String = row.get("exercise_equipment_name");
    let exercise_equipment =
        ExerciseEquipment::from_str(&equipment_name).unwrap_or(ExerciseEquipment::Body);

    Ok(exercise_equipment)
}
#[allow(dead_code)]
pub async fn get_exercise_category_name(
    pool: &SqlitePool,
    exercise_id: i64,
) -> Result<ExerciseCategory, sqlx::Error> {
    let row = sqlx::query("SELECT exercise_category_name FROM exercise WHERE id = ?")
        .bind(exercise_id)
        .fetch_one(pool)
        .await?;

    let category_name: String = row.get("exercise_category_name");
    let exercise_category =
        ExerciseCategory::from_str(&category_name).unwrap_or(ExerciseCategory::Strength);

    Ok(exercise_category)
}
#[allow(dead_code)]
pub async fn get_exercise_instructions(
    pool: &SqlitePool,
    exercise_id: i64,
) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT instructions FROM exercise WHERE id = ?")
        .bind(exercise_id)
        .fetch_one(pool)
        .await?;

    let instructions = row.get("instructions");

    Ok(instructions)
}

#[allow(dead_code)]
pub async fn get_exercise_muscle_name(
    pool: &SqlitePool,
    exercise_id: i64,
) -> Result<Muscle, sqlx::Error> {
    let row = sqlx::query("SELECT muscle_name FROM exercise WHERE id = ?")
        .bind(exercise_id)
        .fetch_one(pool)
        .await?;

    let muscle_name: String = row.get("muscle_name");
    let exercise_muscle = Muscle::from_str(&muscle_name).unwrap_or(Muscle::Chest);

    Ok(exercise_muscle)
}

pub async fn get_exercises_stats(
    pool: &SqlitePool,
    username: &str,
) -> Result<Vec<Exercise>, sqlx::Error> {
    let exercise_row_for_user = sqlx::query(
        "SELECT date,reps,weight_in_kg,exercise_id,workout_id FROM exerciseLog WHERE username = ? ",
    )
    .bind(username)
    .fetch_all(pool)
    .await?;

    let all_exercises = sqlx::query("SELECT id, name FROM exercise")
        .fetch_all(pool)
        .await?;

    let mut exercises = Vec::new();
    let mut exercise_ids: Vec<i64> = Vec::new();

    for all_exercise_counter in all_exercises {
        let id = all_exercise_counter.get("id");
        let exercise_info = get_general_exercise_info(pool, id).await?;
        exercises.push(Exercise {
            general_exercise_info: exercise_info,
            sets: BTreeMap::new(),
        });
        exercise_ids.push(id);
    }

    for exercise_log_counter in exercise_row_for_user {
        let exercise_id: i64 = exercise_log_counter.get("exercise_id");
        let workout_id: Id = exercise_log_counter.get("workout_id");
        let reps: u32 = exercise_log_counter.get("reps");
        let weight: Kg = exercise_log_counter.get("weight_in_kg");
        let date: &str = exercise_log_counter.get("date");

        let real_date = database_utils::database_date_string_to_naive_date(date).unwrap();

        for i in 0..exercise_ids.len() {
            if exercise_ids[i] == exercise_id {
                let exercise = &mut exercises[i];
                exercise.sets.entry(real_date).or_insert_with(Vec::new);
                let set = StrengthSet {
                    workout_id,
                    weight,
                    reps,
                };
                //exercise.name = real_name;
                exercise.sets.get_mut(&real_date).unwrap().push(set);
            }
        }
    }
    Ok(exercises)
}

pub async fn add_workout_to_exercise_log(
    pool: &SqlitePool,
    username: &str,
    workout: Vec<ExerciseJson>,
    date: NaiveDate,
) -> Result<Id, sqlx::Error> {
    let mut transaction = pool.begin().await?;

    let max_id_row = sqlx::query("SELECT MAX(workout_id) as max_id FROM exerciseLog")
        .fetch_optional(&mut *transaction)
        .await?;

    let string_date = database_utils::format_naive_date_for_database(&date);

    let mut next_id: i64 = match max_id_row {
        Some(r) => r.get("max_id"),
        None => 1,
    };

    next_id += 1;

    for exercises in workout {
        let exercise_id_row = sqlx::query("SELECT id FROM exercise WHERE name = ?")
            .bind(&exercises.name)
            .fetch_one(pool)
            .await?;
        let exercise_id: i64 = exercise_id_row.get("id");

        for set in exercises.sets {
            sqlx::query(
                "INSERT INTO exerciseLog (date, username, reps, exercise_id, weight_in_kg, workout_id)
             VALUES (?, ?, ?, ?, ?, ?)"
            )
                .bind(&string_date)
                .bind(username.to_string())
                .bind(set.reps)
                .bind(exercise_id)
                .bind(set.weight)
                .bind(next_id)
                .execute(&mut *transaction)
                .await?;
        }
    }

    transaction.commit().await?;

    Ok(next_id as Id)
}

pub async fn add_exercise_log(
    pool: &SqlitePool,
    username: &str,
    exercise_name: &str,
    reps: i64,
    weight: f32,
    date: NaiveDate,
    workout_id: u32,
) -> Result<(), sqlx::Error> {
    let row = sqlx::query("SELECT id from exercise WHERE name = ?")
        .bind(exercise_name)
        .fetch_one(pool)
        .await?;

    let id: i64 = row.get("id");
    let date_str = database_utils::format_naive_date_for_database(&date);
    sqlx::query(
        "INSERT INTO exerciseLog (date, username, reps, exercise_id, weight_in_kg, workout_id)
            VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(date_str)
    .bind(username)
    .bind(reps)
    .bind(id)
    .bind(weight)
    .bind(workout_id)
    .execute(pool)
    .await?;

    Ok(())
}
