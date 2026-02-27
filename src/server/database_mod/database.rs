use crate::client::backend::profile_stat_manager::ProfileStatManager;
use crate::client::server_communication::server_communicator::SetJson;
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::general_exercise::{
    ExerciseCategory, ExerciseEquipment, ExerciseForce, ExerciseLevel, GeneralExerciseInfo, Id,
    Muscle,
};
use crate::common::exercise_mod::set::StrengthSet;
use crate::common::exercise_mod::weight::Kg;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::user_mod::user::{ForeignUser, Gender, UserInformation};
use crate::common::user_mod::user_goals::UserGoals;
use crate::server::routes::workout::ExerciseJson;
use chrono::NaiveDate;
use sqlx::Row;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::collections::BTreeMap;
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
    weekly_workout_streak INTEGER NOT NULL,
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
    name TEXT NOT NULL,
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
    PresetName TEXT NOT NULL,
    number_of_exercises INTEGER NOT NULL,
    estimated_duration INTEGER NOT NULL,
    description TEXT
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
        "CREATE TABLE IF NOT EXISTS weightLog (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        date TEXT NOT NULL,
        username TEXT NOT NULL,
        weight FLOAT NOT NULL,

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

pub async fn add_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    let default_info = UserInformation::default(&Vec::new());
    sqlx::query("INSERT INTO users (username, user_password, weekly_workout_streak, coin_balance, weight, height, gender, favorite_mascot, selected_mascot, profile_picture)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(username)
        .bind(password)
        .bind(default_info.weekly_workout_streak)
        .bind(default_info.coin_balance)
        .bind(default_info.weight)
        .bind(default_info.height)
        .bind(default_info.gender.to_string())
        .bind(Mascot::default().to_string())
        .bind(Mascot::default().to_string())
        .bind(default_info.profile_picture_path)
        .execute(pool)
        .await?;

    add_mascot_to_user(pool, username, "Duck").await?;

    let default_goals = UserGoals::default();

    sqlx::query(
        "INSERT INTO user_goals(username, weekly_workouts, weight, water, steps, sleep)
        VALUES(?, ?, ?, ?, ?, ?)",
    )
    .bind(username)
    .bind(default_goals.weekly_workouts)
    .bind(default_goals.weight)
    .bind(default_goals.water)
    .bind(default_goals.steps)
    .bind(default_goals.sleep)
    .execute(pool)
    .await?;

    Ok(())
}
pub enum RequestPasswordAnswer {
    UserNotFound,
    Password(String),
}

#[allow(dead_code)]
pub async fn get_password(
    pool: &SqlitePool,
    username: &str,
) -> Result<RequestPasswordAnswer, sqlx::Error> {
    let user = sqlx::query("SELECT user_password FROM users WHERE username == ? ")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    match user {
        None => Ok(RequestPasswordAnswer::UserNotFound),
        Some(row) => Ok(RequestPasswordAnswer::Password(row.get("user_password"))),
    }
}
#[allow(dead_code)]
pub async fn get_all_usernames(pool: &SqlitePool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query("SELECT * from users").fetch_all(pool).await?;
    let mut names: Vec<String> = Vec::new();
    for row in rows {
        let user: String = row.get("username");
        names.push(user.to_string());
    }
    Ok(names)
}
#[allow(dead_code)]
pub async fn update_user_goals(
    pool: &SqlitePool,
    username: &str,
    user_goals: UserGoals,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE user_goals SET weekly_workouts = ? , weight = ? , water = ?, steps = ? , sleep = ? WHERE username = ?")
        .bind(user_goals.weekly_workouts)
        .bind(user_goals.weight)
        .bind(user_goals.water)
        .bind(user_goals.steps)
        .bind(user_goals.sleep)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user_goals(pool: &SqlitePool, username: &str) -> Result<UserGoals, sqlx::Error> {
    let user_goal_row = sqlx::query("SELECT * FROM user_goals WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(UserGoals {
        weekly_workouts: user_goal_row.get("weekly_workouts"),
        weight: user_goal_row.get::<Kg, _>("weight"),
        water: user_goal_row.get("water"),
        steps: user_goal_row.get("steps"),
        sleep: user_goal_row.get("sleep"),
    })
}
#[allow(dead_code)]
pub async fn update_user_weight(
    pool: &SqlitePool,
    username: &str,
    new_weight: Kg,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET weight = ? WHERE username = ?")
        .bind(new_weight)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn update_user_profile_picture(
    pool: &SqlitePool,
    username: &str,
    new_profile_picture_path: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET profile_picture = ? WHERE username = ?")
        .bind(new_profile_picture_path)
        .bind(username)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn update_user_height(
    pool: &SqlitePool,
    username: &str,
    new_height: u32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET height = ? WHERE username = ?")
        .bind(new_height)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_user_gender(
    pool: &SqlitePool,
    username: &str,
    new_gender: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET gender = ? WHERE username = ?")
        .bind(new_gender)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}
#[allow(dead_code)]
pub async fn update_user_weekly_workout_streak(
    pool: &SqlitePool,
    username: &str,
    new_streak: u32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET weekly_workout_streak = ? WHERE username = ?")
        .bind(new_streak)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn update_user_coin_balance(
    pool: &SqlitePool,
    username: &str,
    new_balance: u32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET coin_balance = ? WHERE username = ?")
        .bind(new_balance)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_user_description(
    pool: &SqlitePool,
    username: &str,
    new_description: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET description = ? WHERE username = ?")
        .bind(new_description)
        .bind(username)
        .execute(pool)
        .await?;

    Ok(())
}
#[allow(dead_code)]
pub async fn add_weight_log(
    pool: &SqlitePool,
    username: &str,
    weight: f32,
    date: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO weightLog (date, username, weight)
         VALUES (?, ?, ?)",
    )
    .bind(date)
    .bind(username)
    .bind(weight)
    .execute(pool)
    .await?;

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

pub async fn add_friend(
    pool: &SqlitePool,
    username: &str,
    friendname: &str,
) -> Result<(), sqlx::Error> {
    if username != friendname {
        sqlx::query("INSERT OR IGNORE INTO friendship (username, friendname) VALUES (?,?)")
            .bind(username)
            .bind(friendname)
            .execute(pool)
            .await?;
    }
    Ok(())
}
pub async fn remove_friend(
    pool: &SqlitePool,
    username: &str,
    friendname: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM friendship WHERE username = ? AND friendname = ?")
        .bind(username)
        .bind(friendname)
        .execute(pool)
        .await?;
    Ok(())
}
pub async fn add_mascot_to_user(
    pool: &SqlitePool,
    username: &str,
    mascot_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_mascot (username, mascot_name, level)
                VALUES (?, ?,?)",
    )
    .bind(username)
    .bind(mascot_name)
    .bind(1)
    .execute(pool)
    .await?;

    Ok(())
}
pub async fn get_mascots_from_user(
    pool: &SqlitePool,
    username: &str,
) -> Result<Vec<Mascot>, sqlx::Error> {
    let mascot_rows = sqlx::query("SELECT mascot_name FROM user_mascot WHERE username = ?")
        .bind(username)
        .fetch_all(pool)
        .await?;
    let mut mascots = Vec::new();

    for mascot_row in mascot_rows {
        let mascot_name = mascot_row.get("mascot_name");
        let mascot = Mascot::from_str(mascot_name).unwrap_or(Mascot::default());
        mascots.push(mascot);
    }
    Ok(mascots)
}
pub async fn update_user_favorite_mascot(
    pool: &SqlitePool,
    username: &str,
    new_favorite_mascot: &Mascot,
) -> Result<(), sqlx::Error> {
    let new_fav_mascot_string = new_favorite_mascot.to_string();
    sqlx::query("UPDATE users SET favorite_mascot = ? WHERE username = ?")
        .bind(new_fav_mascot_string)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}
#[allow(dead_code)]
pub async fn get_user_favorite_mascot(
    pool: &SqlitePool,
    username: &str,
) -> Result<Mascot, sqlx::Error> {
    let favorite_mascot_row = sqlx::query("SELECT favorite_mascot FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    let favorite_mascot_string: String = favorite_mascot_row.unwrap().get("favorite_mascot");
    let favorite_mascot = Mascot::from_str(&favorite_mascot_string).unwrap_or(Mascot::default());

    Ok(favorite_mascot)
}
pub async fn update_user_selected_mascot(
    pool: &SqlitePool,
    username: &str,
    new_selected_mascot: &Mascot,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET selected_mascot = ? WHERE username = ?")
        .bind(new_selected_mascot.to_string())
        .bind(username)
        .execute(pool)
        .await?;

    Ok(())
}
pub async fn get_user_selected_mascot(
    pool: &SqlitePool,
    username: &str,
) -> Result<Mascot, sqlx::Error> {
    let selected_mascot_row = sqlx::query("SELECT selected_mascot FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;

    let selected_mascot_string: String = selected_mascot_row.unwrap().get("selected_mascot");
    let selected_mascot = Mascot::from_str(&selected_mascot_string).unwrap_or(Mascot::default());

    Ok(selected_mascot)
}
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
    init_db(pool).await?;
    Ok(())
}
#[allow(dead_code)]
pub async fn get_user_weight(pool: &SqlitePool, username: &str) -> Result<f32, sqlx::Error> {
    let row = sqlx::query("SELECT weight FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(row.get("weight"))
}
#[allow(dead_code)]
pub async fn get_user_height(pool: &SqlitePool, username: &str) -> Result<f32, sqlx::Error> {
    let row = sqlx::query("SELECT height FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(row.get("height"))
}
#[allow(dead_code)]
pub async fn get_user_gender(pool: &SqlitePool, username: &str) -> Result<Gender, sqlx::Error> {
    let row = sqlx::query("SELECT gender FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    let gender_string: String = row.get("gender");
    let exercise_force = Gender::from_str(&gender_string).unwrap_or(Gender::default());

    Ok(exercise_force)
}
#[allow(dead_code)]
pub async fn get_user_weekly_workout_streak(
    pool: &SqlitePool,
    username: &str,
) -> Result<u32, sqlx::Error> {
    let row = sqlx::query("SELECT weekly_workout_streak FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(row.get("weekly_workout_streak"))
}
pub async fn get_user_coin_balance(pool: &SqlitePool, username: &str) -> Result<u32, sqlx::Error> {
    let row = sqlx::query("SELECT coin_balance FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(row.get("coin_balance"))
}
#[allow(dead_code)]
pub async fn get_user_description(
    pool: &SqlitePool,
    username: &str,
) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT description FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(row.get("description"))
}
#[allow(dead_code)]
pub async fn get_user_profile_picture(
    pool: &SqlitePool,
    username: &str,
) -> Result<String, sqlx::Error> {
    let row = sqlx::query("SELECT profile_picture FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    Ok(row.get("profile_picture"))
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
    let date_str = date.format("%d.%m.%y").to_string();
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

    let string_date = date.format("%d.%m.%y").to_string();

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

        let real_date = NaiveDate::parse_from_str(date, "%d.%m.%y").unwrap();

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
pub async fn get_single_foreign_user(
    pool: &SqlitePool,
    active_user: &str,
    target_username: &str,
) -> Result<ForeignUser, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM users WHERE username = ?")
        .bind(target_username)
        .fetch_one(pool)
        .await?;

    let exercise_stats = get_exercises_stats(pool, target_username).await?;

    let friend = sqlx::query("SELECT 1 FROM friendship WHERE username = ? AND friendname = ?")
        .bind(active_user)
        .bind(target_username)
        .fetch_optional(pool)
        .await?;

    let is_friend = friend.is_some();

    let owned_mascot_rows = sqlx::query("SELECT mascot_name FROM user_mascot WHERE username = ?")
        .bind(target_username)
        .fetch_all(pool)
        .await?;

    let mut owned_mascots = Vec::new();
    for mascot_row in owned_mascot_rows {
        let mascot_name: String = mascot_row.get("mascot_name");
        owned_mascots.push(mascot_from_string(&mascot_name));
    }

    Ok(ForeignUser {
        user_information: UserInformation {
            username: row.get("username"),
            description: row.get("description"),
            profile_picture_path: row.get("profile_picture"),
            weight: row.get::<Kg, _>("weight"),
            height: row.get::<f64, _>("height") as u32,
            gender: match row.get("gender") {
                "Female" => Gender::Female,
                _ => Gender::Male,
            },
            weekly_workout_streak: row.get::<i64, _>("weekly_workout_streak") as u32,
            coin_balance: row.get::<i64, _>("coin_balance") as u32,
            favorite_mascot: mascot_from_string(row.get("favorite_mascot")),
            user_goals: get_user_goals(pool, target_username).await?,
            profile_stat_manager: ProfileStatManager::new(&exercise_stats),
        },
        selected_mascot: mascot_from_string(row.get("selected_mascot")),
        owned_mascots,
        friends_with_active_user: is_friend,
    })
}
pub fn mascot_from_string(name: &str) -> Mascot {
    for mascot in Mascot::iter() {
        if mascot.get_name() == name {
            return mascot;
        }
    }
    Mascot::default()
}

pub async fn get_all_friends(
    pool: &SqlitePool,
    active_user: &str,
) -> Result<Vec<ForeignUser>, sqlx::Error> {
    let mut friends = Vec::new();

    let all_friend_rows = sqlx::query("SELECT friendname FROM friendship WHERE username = ?")
        .bind(active_user)
        .fetch_all(pool)
        .await?;

    for friend_row in all_friend_rows {
        let name: String = friend_row.get("friendname");

        if let Ok(user) = get_single_foreign_user(pool, active_user, &name).await {
            friends.push(user);
        }
    }
    Ok(friends)
}

pub async fn get_discovery_users(
    pool: &SqlitePool,
    active_user: &str,
    limit: i64,
) -> Result<Vec<ForeignUser>, sqlx::Error> {
    let discovered_users_rows = sqlx::query(
        "SELECT username FROM users
         WHERE username != ?
         AND username NOT IN (SELECT friendname FROM friendship WHERE username = ?)
         ORDER BY RANDOM()
         LIMIT ?",
    )
    .bind(active_user)
    .bind(active_user)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    let mut discovery_users = Vec::new();

    for row in discovered_users_rows {
        let discovered_username: String = row.get("username");
        if let Ok(user) = get_single_foreign_user(pool, active_user, &discovered_username).await {
            discovery_users.push(user);
        }
    }

    Ok(discovery_users)
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
pub async fn get_user_information(
    pool: &SqlitePool,
    username: &str,
) -> Result<UserInformation, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    let exercise_stats = get_exercises_stats(pool, username).await?;

    Ok(UserInformation {
        username: row.get("username"),
        description: row.get("description"),
        profile_picture_path: row.get("profile_picture"),
        weight: row.get::<Kg, _>("weight"),
        height: row.get::<f64, _>("height") as u32,
        gender: match row.get("gender") {
            "Female" => Gender::Female,
            _ => Gender::Male,
        },
        weekly_workout_streak: row.get::<i64, _>("weekly_workout_streak") as u32,
        coin_balance: row.get::<i64, _>("coin_balance") as u32,
        favorite_mascot: mascot_from_string(row.get("favorite_mascot")),
        user_goals: get_user_goals(pool, username).await?,
        profile_stat_manager: ProfileStatManager::new(&exercise_stats),
    })
}
