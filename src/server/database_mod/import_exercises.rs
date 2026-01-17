use serde::Deserialize;
use sqlx::{Sqlite, SqlitePool, Transaction};

//TODO use enums for force, level, equipment, primary_muscle, category
#[derive(Deserialize)]
pub struct ExerciseInfo {
    name: String,
    force: Option<String>,
    level: String,
    equipment: Option<String>,
    #[serde(rename = "primaryMuscles")]
    primary_muscle: Vec<String>,
    instructions: Vec<String>,
    category: String,
}
pub enum Muscle {
    Abdominals,
    Hamstrings,
    Calves,
    Shoulders,
    Adductors,
    Glutes,
    Quadriceps,
    Biceps,
    Forearms,
    Abductors,
    Triceps,
    Chest,
    LowerBack,
    Traps,
    MiddleBack,
    Lats,
    Neck,
}

pub enum ExerciseForce {
    Pull,
    Push,
}

pub enum ExerciseLevel {
    Beginner,
    Intermediate,
    Expert,
}

pub enum ExerciseMechanic {
    Compound,
    Isolation,
}

pub enum ExerciseEquipment {
    Body,
    Machine,
    Kettlebells,
    Dumbbell,
    Cable,
    Barbell,
    Bands,
    MedicineBall,
    ExerciseBall,
    EzCurlBar,
    FoamRoll,
}

pub enum ExerciseCategory {
    Strength,
    Stretching,
    Plyometrics,
    Strongman,
    Powerlifting,
    Cardio,
    OlympicWeightlifting,
    Crossfit,
    WeightedBodyweight,
    AssistedBodyweight,
}

/// Only used for initial filling of the exercise table
#[allow(dead_code)]
pub async fn import_exercises(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut transaction = pool.begin().await.expect("Transaction err");

    let mut exercise_folders = tokio::fs::read_dir("assets/exercises")
        .await
        .expect("exercise folder err");

    while let Some(exercise_folder) = exercise_folders.next_entry().await.expect("next err") {
        let exercise_json_path = exercise_folder.path().join("exercise.json");
        if !exercise_json_path.exists() {
            continue;
        }
        let exercise_json = tokio::fs::read_to_string(&exercise_json_path)
            .await
            .expect("json to string err");
        let exercise_info: ExerciseInfo =
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
async fn insert_exercise_in_db<'a>(
    transaction: &mut Transaction<'a, Sqlite>,
    exercise_json: &ExerciseInfo,
) -> Result<(), sqlx::Error> {
    let instructions = exercise_json.instructions.join("\n");
    let muscle = exercise_json
        .primary_muscle
        .get(0)
        .map(|mus| mus.as_str())
        .unwrap_or("None");

    sqlx::query(
        "INSERT INTO exercise (name,exercise_force_name,exercise_level_name,exercise_equipment_name, muscle_name, instructions, exercise_category_name)
    VALUES (?,?,?,?,?,?,?) ",
    )
        .bind(&exercise_json.name)
        .bind(&exercise_json.force)
        .bind(&exercise_json.level)
        .bind(&exercise_json.equipment)
        .bind(&muscle)
        .bind(&instructions)
        .bind(&exercise_json.category)
        .execute(&mut **transaction)
        .await?;

    Ok(())
}
