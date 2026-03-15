use crate::client::server_communication::exercise_communicator::SetJson;
use crate::common::exercise_mod::general_exercise::{ExerciseCategory, ExerciseLevel, Muscle};
pub(crate) use crate::server::database_mod::database_exercise::{
    add_exercise_log, add_workout_to_exercise_log, get_exercises_stats, get_general_exercise_info,
};
use crate::server::database_mod::database_social::{
    add_friend, get_all_friends, get_discovery_users, get_single_foreign_user,
};
use crate::server::database_mod::database_user::{add_user, get_user_information};
use crate::server::routes::workout::ExerciseJson;
use chrono::NaiveDate;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

pub async fn init_pool() -> Result<SqlitePool, sqlx::Error> {
    let options =
        SqliteConnectOptions::from_str("sqlite:database/database.db")?.create_if_missing(true);

    let pool = SqlitePoolOptions::new().connect_with(options).await?;
    Ok(pool)
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
    username TEXT PRIMARY KEY,
    user_password TEXT NOT NULL,
    coin_balance INTEGER NOT NULL,
    weight FLOAT NOT NULL,
    height FLOAT NOT NULL,
    gender TEXT NOT NULL,
    favorite_mascot TEXT NOT NULL,
    selected_mascot TEXT NOT NULL,
    profile_picture TEXT,
    description TEXT
    );",
    )
    .execute(pool)
    .await?; //TODO change height to INTEGER

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS mascot(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mascot_name TEXT NOT NULL UNIQUE,
    description TEXT NULL
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_mascot(
    username TEXT NOT NULL,
    mascot_name TEXT NOT NULL,
    level INTEGER NOT NULL,
    PRIMARY KEY (username, mascot_name),

    FOREIGN KEY (username) REFERENCES users (username),
    FOREIGN KEY (mascot_name) REFERENCES mascot (mascot_name)
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS exercise (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    exercise_force_name TEXT,
    exercise_level_name TEXT NOT NULL,
    exercise_equipment_name TEXT,
    muscle_name TEXT NOT NULL,
    instructions TEXT NOT NULL,
    exercise_category_name TEXT NOT NULL
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        " CREATE TABLE IF NOT EXISTS exerciseLog (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    username TEXT NOT NULL,
    reps INTEGER NOT NULL,
    exercise_id INTEGER NOT NULL,
    weight_in_kg FLOAT NOT NULL,
    workout_id INTEGER NOT NULL,
    FOREIGN KEY (exercise_id) REFERENCES exercise(id),
    FOREIGN KEY (username) REFERENCES users(username)
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS preset (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    preset_name TEXT NOT NULL,
    preset_image TEXT NOT NULL,
    number_of_exercises INTEGER NOT NULL,
    estimated_duration INTEGER NOT NULL,
    description TEXT
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS preset_exercise(
    preset_id INTEGER NOT NULL,
    exercise_name TEXT NOT NULL,

    FOREIGN KEY (preset_id) REFERENCES preset (id)
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_preset(
    username TEXT NOT NULL,
    preset_id INTEGER NOT NULL,
    times_preset_trained INTEGER NOT NULL,
    PRIMARY KEY (username, preset_id),

    FOREIGN KEY (username) REFERENCES users (username),
    FOREIGN KEY (preset_id) REFERENCES preset (id)
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS friendship (
        username TEXT NOT NULL,
        friendname TEXT NOT NULL,
        PRIMARY KEY (username, friendname),

        FOREIGN KEY (username) REFERENCES users(username),
        FOREIGN KEY (friendname) REFERENCES users(username)
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS logs (
        username TEXT NOT NULL,
        date TEXT NOT NULL,
        value FLOAT NOT NULL,
        log_type TEXT NOT NULL,

        PRIMARY KEY(username, date),
        FOREIGN KEY (username) REFERENCES users(username)
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_goals (
    username TEXT NOT NULL,
    weekly_workouts FLOAT NOT NULL,
    weight FLOAT NOT NULL,
    water FLOAT NOT NULL,
    steps FLOAT NOT NULL,
    sleep FLOAT NOT NULL,

    PRIMARY KEY (username),

    FOREIGN KEY (username) REFERENCES users(username)
);",
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[allow(dead_code)]
pub async fn reset_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DROP TABLE IF EXISTS friendship")
        .execute(pool)
        .await?;
    sqlx::query("DROP TABLE IF EXISTS user_preset")
        .execute(pool)
        .await?;
    sqlx::query("DROP TABLE IF EXISTS preset")
        .execute(pool)
        .await?;
    sqlx::query("DROP TABLE IF EXISTS exerciseLog")
        .execute(pool)
        .await?;
    //sqlx::query("DROP TABLE IF EXISTS exercise")
    //.execute(pool)
    //.await?;
    sqlx::query("DROP TABLE IF EXISTS user_goals ")
        .execute(pool)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS user_mascot")
        .execute(pool)
        .await?;
    //sqlx::query("DROP TABLE IF EXISTS mascot")
    //.execute(pool)
    //.await?;
    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(pool)
        .await?;
    sqlx::query("DROP TABLE IF EXISTS weightLog")
        .execute(pool)
        .await?;
    sqlx::query("DROP TABLE IF EXISTS preset_exercise")
        .execute(pool)
        .await?;
    init_db(pool).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn test_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    reset_database(pool).await.expect("database reset failed");

    println!("reseting database was sucecss");

    add_user(pool, "robert", "123").await?;
    add_user(pool, "felix", "456").await?;
    add_user(pool, "maxi", "789").await?;
    add_user(pool, "stefano", "abc").await?;
    add_user(pool, "anna", "anna").await?;
    add_user(pool, "banna", "banna").await?;
    add_user(pool, "canna", "canna").await?;
    add_user(pool, "danna", "danna").await?;
    add_user(pool, "fanna", "fanna").await?;
    add_user(pool, "ganna", "ganna").await?;
    add_user(pool, "hanna", "hanna").await?;

    println!("Adding users was success");

    add_friend(pool, "robert", "felix").await?;
    add_friend(pool, "robert", "maxi").await?;
    add_friend(pool, "robert", "stefano").await?;

    println!("adding friends was success");

    add_exercise_log(
        pool,
        "felix",
        "Internal Rotation with Band",
        100,
        60.0,
        NaiveDate::from_ymd_opt(2025, 10, 10).unwrap(),
        1,
    )
    .await?;

    add_exercise_log(
        pool,
        "felix",
        "Incline Bench Pull",
        100,
        60.0,
        NaiveDate::from_ymd_opt(2025, 10, 10).unwrap(),
        1,
    )
    .await?;

    add_exercise_log(
        pool,
        "felix",
        "Incline Bench Pull",
        100,
        100.0,
        NaiveDate::from_ymd_opt(2025, 10, 10).unwrap(),
        1,
    )
    .await?;

    let felix_foreign_user = get_single_foreign_user(pool, "robert", "felix").await?;

    let discovered_foreign_users = get_discovery_users(pool, "robert", 5).await?;

    let mut discovered_user_list = String::new();

    for discovered_user in discovered_foreign_users {
        let discovered_username = discovered_user.user_information.username;
        discovered_user_list.push_str(&discovered_username);
        discovered_user_list.push_str(" , ");
    }

    println!("These are the discovered users: {}", discovered_user_list);

    assert_eq!(felix_foreign_user.user_information.username, "felix");
    assert!(felix_foreign_user.friends_with_active_user);

    println!("get single_foreign_user was success");

    let friends_robert = get_all_friends(pool, "robert").await?;

    assert_eq!(friends_robert.len(), 3);
    assert_eq!(friends_robert[0].user_information.username, "felix");

    println!("get_all_friends was success");

    let felix_exercises = get_exercises_stats(pool, "felix").await?;

    let exercise_info_from_incline_bench_pull = get_general_exercise_info(pool, 3).await?;
    let user_information_robert = get_user_information(pool, "robert").await?;

    assert_eq!(
        felix_exercises[2].general_exercise_info.name,
        "Incline Bench Pull"
    );

    assert_eq!(
        felix_exercises[2].general_exercise_info.name,
        exercise_info_from_incline_bench_pull.name
    );

    assert_eq!(user_information_robert.username, "robert");

    let mut name_list = String::new();

    for felix_exercise_iterator in felix_exercises.iter() {
        let exercise_name = &felix_exercise_iterator.general_exercise_info.name;

        let mut amount_sets = 0;

        for sets_list in felix_exercise_iterator.sets.values() {
            amount_sets += sets_list.len();
        }
        let entry = format!("{} ({}x), ", exercise_name, amount_sets);
        name_list.push_str(&entry);
    }

    println!(
        "Success: Felix hat folgende Übungen gemacht so oft gemacht: {}",
        name_list
    );

    let incline_bench_pull_id = 3;

    let incline_bench_pull_info = get_general_exercise_info(pool, incline_bench_pull_id).await?;

    assert_eq!(incline_bench_pull_info.name, "Incline Bench Pull");
    println!("Name of general exercise was success!");
    assert_eq!(incline_bench_pull_info.primary_muscle, Muscle::MiddleBack);
    println!("Muscle of general exercise was success!");
    assert_eq!(incline_bench_pull_info.level, ExerciseLevel::Beginner);
    println!("Level of general exercise was success!");
    assert_eq!(incline_bench_pull_info.category, ExerciseCategory::Strength);
    println!("Category of general exercise was success!");

    let leg_day = vec![
        ExerciseJson {
            name: "Smith Machine Bench Press".to_string(),
            sets: vec![
                SetJson {
                    weight: 420.5,
                    reps: 12,
                },
                SetJson {
                    weight: 1000.5,
                    reps: 10,
                },
            ],
        },
        ExerciseJson {
            name: "Reverse Triceps Bench Press".to_string(),
            sets: vec![SetJson {
                weight: 2.0,
                reps: 8,
            }],
        },
    ];

    let test_date = NaiveDate::from_ymd_opt(2006, 4, 26).unwrap();

    add_workout_to_exercise_log(pool, "robert", leg_day, test_date).await?;

    println!("adding leg_day workout to exerciseLog was success");

    Ok(())
}
//moved outside test config so it can be accessed by integration tests
pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    init_db(&pool).await.unwrap();
    pool
}

pub async fn test_values_for_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO mascot (mascot_name, description) VALUES (?, ?)")
        .bind("Duck")
        .bind("test")
        .execute(pool)
        .await
        .expect("Mascot insert failed");

    add_user(pool, "testuser", "123")
        .await
        .expect("adding user failed");

    add_user(pool, "testuser2", "123")
        .await
        .expect("adding user failed");

    add_user(pool, "testuser3", "123")
        .await
        .expect("adding user failed");

    add_user(pool, "testuser4", "123")
        .await
        .expect("adding user failed");

    add_user(pool, "testuser5", "123")
        .await
        .expect("adding user failed");

    sqlx::query("INSERT INTO exercise (name, exercise_level_name, muscle_name, instructions, exercise_category_name)
                 VALUES (?, ?, ?, ?, ?)")
        .bind("Bankdrücken")
        .bind("Beginner")
        .bind("Chest")
        .bind("test")
        .bind("Strength")
        .execute(pool).await.expect("Insert of exercise failed");

    sqlx::query("INSERT INTO exercise (name, exercise_level_name, muscle_name, instructions, exercise_category_name)
                 VALUES (?, ?, ?, ?, ?)")
        .bind("Squat")
        .bind("Expert")
        .bind("Glutes")
        .bind("test2")
        .bind("Strength")
        .execute(pool).await.expect("Insert of exercise failed");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::user_mod::user_goals::UserGoals;
    use crate::common::workout_preset::{PresetImage, WorkoutPreset};
    use crate::server::database_mod::database_preset::{
        add_preset, add_preset_to_user, get_presets_for_user,
    };
    use crate::server::database_mod::database_social::add_friend;
    use crate::server::database_mod::database_user_goals::{get_user_goals, update_user_goals};

    #[tokio::test]
    async fn test_add_and_get_preset() {
        let pool = setup_test_db().await;
        test_values_for_db(&pool)
            .await
            .expect("inserting test values failed");

        let workout = WorkoutPreset {
            name: "test_workout".to_string(),
            image: PresetImage::Bench,
            exercises: vec!["Bankdrücken".to_string(), "Squat".to_string()],
        };

        let preset_id = add_preset(&pool, &workout, 69)
            .await
            .expect("Fehler beim Hinzufügen des Presets");

        add_preset_to_user(&pool, "testuser", preset_id)
            .await
            .expect("add_preset_to_user failed");

        let preset_from_user: Vec<WorkoutPreset> =
            get_presets_for_user(&pool, "testuser").await.unwrap();

        let preset = &preset_from_user[0];

        assert_eq!(preset_from_user.len(), 1);
        assert_eq!(preset.name, "test_workout");
        assert_eq!(preset.image, PresetImage::Bench);
        assert_eq!(
            preset.exercises,
            vec!["Bankdrücken".to_string(), "Squat".to_string()]
        );
    }

    #[tokio::test]
    async fn test_user_goals() {
        let pool = setup_test_db().await;
        test_values_for_db(&pool)
            .await
            .expect("inserting test values failed");

        let test_goals = UserGoals {
            weekly_workouts: 1.0,
            weight: 2.0,
            water: 3.0,
            steps: 4.0,
            sleep: 5.0,
        };

        update_user_goals(&pool, "testuser", test_goals)
            .await
            .expect("updating user goals failed");

        let testuser_goals = get_user_goals(&pool, "testuser")
            .await
            .expect("getting usergoals failed");

        assert_eq!(testuser_goals.weekly_workouts, 1.0);
        assert_eq!(testuser_goals.weight, 2.0);
        assert_eq!(testuser_goals.water, 3.0);
        assert_eq!(testuser_goals.steps, 4.0);
        assert_eq!(testuser_goals.sleep, 5.0);
    }

    #[tokio::test]
    async fn test_friendship_and_discovery() {
        let pool = setup_test_db().await;
        test_values_for_db(&pool)
            .await
            .expect("inserting test values failed");

        add_friend(&pool, "testuser", "testuser2")
            .await
            .expect("adding friend failed");
        add_friend(&pool, "testuser", "testuser3")
            .await
            .expect("adding friend failed");

        let testuser_friends = get_all_friends(&pool, "testuser")
            .await
            .expect("getting friends failed");

        assert_eq!(testuser_friends.len(), 2);
        assert_eq!(testuser_friends[0].user_information.username, "testuser2");
        assert_eq!(testuser_friends[1].owned_mascots.len(), 1);

        let test_discovered_users = get_discovery_users(&pool, "testuser", 3)
            .await
            .expect("get discovered users failed");
        let mut discovered_list: Vec<String> = Vec::new();
        for discovered_user in test_discovered_users {
            discovered_list.push(discovered_user.user_information.username);
        }
        assert!(discovered_list.contains(&"testuser4".to_string()));
        assert!(discovered_list.contains(&"testuser5".to_string()));
    }

    #[tokio::test]
    async fn test_exerciselog() {
        let pool = setup_test_db().await;
        test_values_for_db(&pool)
            .await
            .expect("inserting test values failed");

        add_exercise_log(
            &pool,
            "testuser",
            "Squat",
            100,
            60.0,
            NaiveDate::from_ymd_opt(2025, 10, 10).unwrap(),
            1,
        )
        .await
        .expect("add exerciselog failed");

        add_exercise_log(
            &pool,
            "testuser",
            "Squat",
            100,
            60.0,
            NaiveDate::from_ymd_opt(2025, 10, 10).unwrap(),
            1,
        )
        .await
        .expect("add exerciselog failed");

        let test_day = vec![
            ExerciseJson {
                name: "Bankdrücken".to_string(),
                sets: vec![
                    SetJson {
                        weight: 420.5,
                        reps: 12,
                    },
                    SetJson {
                        weight: 1000.5,
                        reps: 10,
                    },
                ],
            },
            ExerciseJson {
                name: "Squat".to_string(),
                sets: vec![SetJson {
                    weight: 2.0,
                    reps: 8,
                }],
            },
        ];

        let test_date = NaiveDate::from_ymd_opt(2006, 4, 26).unwrap();

        let test_workout_id = add_workout_to_exercise_log(&pool, "testuser", test_day, test_date)
            .await
            .expect("Adding workout was failure");

        let testuser_stats = get_exercises_stats(&pool, "testuser")
            .await
            .expect("Getting stats failed");

        assert_eq!(testuser_stats[0].general_exercise_info.name, "Bankdrücken");
        assert_eq!(testuser_stats[1].general_exercise_info.name, "Squat");
        assert_eq!(testuser_stats.len(), 2);
        assert_eq!(testuser_stats[0].all_time_sets(), 2);
        assert_eq!(test_workout_id, 2);
    }
}
