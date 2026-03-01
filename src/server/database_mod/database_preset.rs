use crate::common::workout_preset::WorkoutPreset;
use sqlx::{Row, SqlitePool};

#[allow(dead_code)]
pub async fn add_preset(
    pool: &SqlitePool,
    workout_preset: &WorkoutPreset,
    estimated_duration: i64,
) -> Result<i64, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    let preset_insert = sqlx::query(
        "INSERT INTO preset (preset_name, preset_image, number_of_exercises, estimated_duration)
                        VALUES (?, ?,?, ?)",
    )
    .bind(workout_preset.name.clone())
    .bind(workout_preset.image.to_string())
    .bind(workout_preset.exercises.len() as i64)
    .bind(estimated_duration)
    .execute(&mut *transaction)
    .await?;

    let preset_id = preset_insert.last_insert_rowid();
    for exercises in workout_preset.exercises.iter() {
        sqlx::query(
            "INSERT INTO preset_exercise (preset_id, exercise_name)
                        VALUES (?, ?)",
        )
        .bind(preset_id)
        .bind(exercises)
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(preset_id)
}
#[allow(dead_code)]
pub async fn get_presets_for_user(
    pool: &SqlitePool,
    username: &str,
) -> Result<Vec<WorkoutPreset>, sqlx::Error> {
    let preset_id_rows = sqlx::query("SELECT preset_id FROM user_preset WHERE username = ? ")
        .bind(username)
        .fetch_all(pool)
        .await?;

    let mut workout_presets = Vec::new();

    for preset_id_row in preset_id_rows {
        let temp_id: i64 = preset_id_row.get("preset_id");

        let preset_row = sqlx::query(
            "SELECT preset.preset_name, preset.preset_image, preset_exercise.exercise_name
                FROM preset
                JOIN preset_exercise ON preset.id = preset_exercise.preset_id
                WHERE preset.id = ?",
        )
        .bind(temp_id)
        .fetch_all(pool)
        .await?;

        let temp_preset_name: String = preset_row[0].get("preset_name");
        let temp_image: String = preset_row[0].get("preset_image");

        let mut exercises = Vec::new();

        for exercise_row in preset_row {
            let temp_exercise_name: String = exercise_row.get("exercise_name");
            exercises.push(temp_exercise_name);
        }

        let temp_workout_preset = WorkoutPreset {
            name: temp_preset_name,
            image: temp_image.into(),
            exercises,
        };

        workout_presets.push(temp_workout_preset);
    }

    Ok(workout_presets)
}
#[allow(dead_code)]
pub async fn delete_preset(pool: &SqlitePool, preset_id: i64) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await?;

    sqlx::query("DELETE FROM user_preset WHERE preset_id = ?")
        .bind(preset_id)
        .execute(&mut *transaction)
        .await?;

    sqlx::query("DELETE FROM preset_exercise WHERE preset_id = ?")
        .bind(preset_id)
        .execute(&mut *transaction)
        .await?;

    sqlx::query("DELETE FROM preset WHERE id = ?")
        .bind(preset_id)
        .execute(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn add_preset_to_user(
    pool: &SqlitePool,
    username: &str,
    preset_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_preset (username, preset_id, times_preset_trained)
                VALUES (?, ?,?)",
    )
    .bind(username)
    .bind(preset_id)
    .bind(0)
    .execute(pool)
    .await?;
    Ok(())
}
#[allow(dead_code)]
pub async fn increment_preset_trained_from_user(
    pool: &SqlitePool,
    username: &str,
    preset_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE user_preset
         SET times_preset_trained = times_preset_trained + 1
         WHERE username = ? AND preset_id = ?",
    )
    .bind(username)
    .bind(preset_id)
    .execute(pool)
    .await?;

    Ok(())
}
#[allow(dead_code)]
pub async fn get_preset_trained_from_user(
    pool: &SqlitePool,
    username: &str,
    preset_id: i64,
) -> Result<i64, sqlx::Error> {
    let times_preset_trained_row = sqlx::query(
        "SELECT times_preset_trained FROM user_preset WHERE username = ? AND preset_id = ? ",
    )
    .bind(username)
    .bind(preset_id)
    .fetch_optional(pool)
    .await?;

    Ok(match times_preset_trained_row {
        Some(row) => row.get("times_preset_trained"),
        None => 0,
    })
}
