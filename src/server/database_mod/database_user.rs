use crate::client::backend::profile_stat_manager::ProfileStatManager;
use crate::common::exercise_mod::weight::Kg;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::user_mod::user::{Gender, UserInformation};
use crate::common::user_mod::user_goals::{GoalType, UserGoals};
use crate::common::user_mod::user_log::UserLog;
use crate::server::database_mod::database::get_exercises_stats;
use crate::server::database_mod::database_mascot::{add_mascot_to_user, mascot_from_string};
use crate::server::database_mod::database_user_goals::get_user_goals;
use crate::server::database_mod::database_user_logs::{add_user_log, get_user_log};
use chrono::Local;
use sqlx::Row;
use sqlx::SqlitePool;
use std::str::FromStr;

pub async fn add_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<(), sqlx::Error> {
    let default_info = UserInformation::default(&Vec::new());
    sqlx::query("INSERT INTO users (username, user_password, coin_balance, weight, height, gender, favorite_mascot, selected_mascot, profile_picture)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(username)
        .bind(password)
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

pub async fn update_user_info_settings(
    pool: &SqlitePool,
    username: &str,
    new_user_info: UserInformation,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users
    SET
    favorite_mascot = ?,
    gender = ?,
    weight = ?,
    height = ?,
    description = ?,
    profile_picture = ?
    WHERE
        username = ?
    ",
    )
    .bind(new_user_info.favorite_mascot.to_string())
    .bind(new_user_info.gender.to_string())
    .bind(new_user_info.weight)
    .bind(new_user_info.height)
    .bind(new_user_info.description)
    .bind(new_user_info.profile_picture_path)
    .bind(username)
    .execute(pool)
    .await?;

    Ok(())
}

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

    add_user_log(
        pool,
        username,
        new_weight,
        Local::now().date_naive(),
        GoalType::Weight,
    )
    .await?;

    Ok(())
}

#[allow(dead_code)]
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
#[allow(dead_code)]
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

pub async fn get_user_information(
    pool: &SqlitePool,
    username: &str,
) -> Result<UserInformation, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    let exercise_stats = get_exercises_stats(pool, username).await?;

    let user_goals = get_user_goals(pool, username).await?;

    let user_logs = UserLog {
        weight_log: get_user_log(pool, username, GoalType::Weight).await?,
        water_log: get_user_log(pool, username, GoalType::Water).await?,
        step_log: get_user_log(pool, username, GoalType::Steps).await?,
        sleep_log: get_user_log(pool, username, GoalType::Sleep).await?,
    };

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
        coin_balance: row.get::<i64, _>("coin_balance") as u32,
        favorite_mascot: mascot_from_string(row.get("favorite_mascot")),
        profile_stat_manager: ProfileStatManager::new(
            &exercise_stats,
            user_goals.weekly_workouts as u32,
        ),
        user_logs,
        user_goals,
    })
}
