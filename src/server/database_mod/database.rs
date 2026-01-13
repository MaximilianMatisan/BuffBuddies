use sqlx::Row;
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
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    user_password TEXT NOT NULL,
    weekly_workout_goal INTEGER NOT NULL,
    weekly_workout_streak INTEGER NOT NULL,
    coin_balance INTEGER NOT NULL,
    weight FLOAT NOT NULL,
    height FLOAT NOT NULL,
    gender TEXT NOT NULL,
    profile_picture TEXT,
    description TEXT
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS mascot(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NULL
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_mascot(
    user_id INTEGER NOT NULL,
    mascot_id INTEGER NOT NULL,
    level INTEGER NOT NULL,
    PRIMARY KEY (user_id, mascot_id),

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (mascot_id) REFERENCES mascot (id)
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS exercise (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    exercise_name TEXT NOT NULL,
    description TEXT
    );",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        " CREATE TABLE IF NOT EXISTS exerciseLog (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    username TEXT NOT NULL,
    reps INTEGER NOT NULL,
    exercise_id INTEGER NOT NULL,
    weight_in_kg FLOAT NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username),
    FOREIGN KEY (exercise_id) REFERENCES exercise(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
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
    user_id INTEGER NOT NULL,
    preset_id INTEGER NOT NULL,
    times_preset_trained INTEGER NOT NULL,
    PRIMARY KEY (user_id, preset_id),

    FOREIGN KEY (user_id) REFERENCES users (id),
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

    Ok(())
}

#[allow(dead_code)]
pub async fn add_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
    gender: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (username, user_password, weekly_workout_goal, weekly_workout_streak, coin_balance, weight, height, gender, profile_picture)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(username)
        .bind(password)
        .bind(0)
        .bind(0)
        .bind(0)
        .bind(0)
        .bind(0)
        .bind(gender)
        .bind("")
        .execute(pool)
        .await?;

    Ok(())
}
pub enum ValidUser {
    UserNotFound,
    WrongPassword,
    Valid,
}
#[allow(dead_code)]
pub async fn check_user(
    pool: &SqlitePool,
    username: &str,
    passwort: &str,
) -> Result<ValidUser, sqlx::Error> {
    let user = sqlx::query("SELECT user_password FROM users WHERE username == ? ")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    match user {
        None => Ok(ValidUser::UserNotFound),
        Some(row) => {
            let saved_passwort: String = row.get("user_password");
            if saved_passwort == passwort {
                Ok(ValidUser::Valid)
            } else {
                Ok(ValidUser::WrongPassword)
            }
        }
    }
}
#[allow(dead_code)]
pub async fn print_all_users(pool: &SqlitePool) -> Result<String, sqlx::Error> {
    let rows = sqlx::query("SELECT * from users").fetch_all(pool).await?;
    let mut namen: String = String::from("User: ");
    for row in rows {
        let user: String = row.get("username");
        namen.push_str(&user);
        namen.push_str(", ");
    }
    Ok(namen)
}
#[allow(dead_code)]
pub async fn update_user_weight(
    pool: &SqlitePool,
    username: &str,
    new_weight: f32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET weight = ? WHERE username = ?")
        .bind(new_weight)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}
#[allow(dead_code)]
pub async fn update_user_height(
    pool: &SqlitePool,
    username: &str,
    new_height: f32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET height = ? WHERE username = ?")
        .bind(new_height)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}
#[allow(dead_code)]
pub async fn update_user_weekly_workout_goal(
    pool: &SqlitePool,
    username: &str,
    new_goal: f32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET weekly_workout_goal = ? WHERE username = ?")
        .bind(new_goal)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}
#[allow(dead_code)]
pub async fn get_user_id(pool: &SqlitePool, name: &str) -> Result<i64, sqlx::Error> {
    let user_id = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(name)
        .fetch_optional(pool)
        .await?;
    Ok(user_id.expect("getting id failed ").get("id"))
}
#[allow(dead_code)]
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
    new_streak: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET weekly_workout_streak = ? WHERE username = ?")
        .bind(new_streak)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}
#[allow(dead_code)]
pub async fn update_user_coin_balance(
    pool: &SqlitePool,
    username: &str,
    new_balance: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET coin_balance = ? WHERE username = ?")
        .bind(new_balance)
        .bind(username)
        .execute(pool)
        .await?;
    Ok(())
}

#[allow(dead_code)]
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
