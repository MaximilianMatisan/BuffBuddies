use serde::Deserialize;
use sqlx::{Sqlite, SqlitePool, Transaction};

#[derive(Deserialize)]
pub struct ExerciseJson {
    name: String,
    force: Option<String>,
    level: String,
    equipment: Option<String>,
    #[serde(rename = "primaryMuscles")]
    primary_muscle: Vec<String>,
    instructions: Vec<String>,
    category: String,
}

/// Only used for initial filling of the exercise table
#[allow(dead_code)]
pub async fn import_exercises(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await.expect("Transaction err");

    let mut exercise_folders = tokio::fs::read_dir("assets/exercises")
        .await
        .expect("exercise folder err");

    while let Some(exercise_folder) = exercise_folders
        .next_entry()
        .await
        .expect("next exercise err")
    {
        let exercise_json_path = exercise_folder.path().join("exercise.json");
        if !exercise_json_path.exists() {
            continue;
        }
        let exercise_json = tokio::fs::read_to_string(&exercise_json_path)
            .await
            .expect("json to string err");
        let exercise_info: ExerciseJson =
            serde_json::from_str(&exercise_json).expect("deserialize err");

        if exercise_info.category == "strength" {
            insert_exercise_in_db(&mut transaction, &exercise_info)
                .await
                .expect("insert in db err");
        }
    }
    transaction.commit().await.expect("commit err");
    Ok(())
}

/// Only used for initial filling of the exercise table
#[allow(dead_code)]
async fn insert_exercise_in_db(
    transaction: &mut Transaction<'_, Sqlite>,
    exercise_json: &ExerciseJson,
) -> Result<(), sqlx::Error> {
    let instructions = exercise_json.instructions.join(" ");
    let muscle = exercise_json
        .primary_muscle
        .first()
        .map(|mus| mus.as_str())
        .unwrap_or("None"); //Sollte normalerweise nicht passieren

    sqlx::query(
        "INSERT INTO exercise (name,exercise_force_name,exercise_level_name,exercise_equipment_name, muscle_name, instructions, exercise_category_name)
    VALUES (?,?,?,?,?,?,?) ",
    )
        .bind(&exercise_json.name)
        .bind(&exercise_json.force)
        .bind(&exercise_json.level)
        .bind(&exercise_json.equipment)
        .bind(muscle)
        .bind(&instructions)
        .bind(&exercise_json.category)
        .execute(&mut **transaction)
        .await?;

    Ok(())
}
