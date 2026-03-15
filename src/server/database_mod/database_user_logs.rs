use chrono::NaiveDate;
use sqlx::SqlitePool;
use crate::common::user_mod::user_goals::GoalType;
use crate::common::user_mod::user_log::Log;
use crate::server::database_mod::database_utils;

pub async fn add_user_log(
    pool: &SqlitePool,
    username: &str,
    weight: f32,
    date: NaiveDate,
    log_type: GoalType,
) -> Result<(), sqlx::Error> {

    if log_type == GoalType::WeeklyWorkouts {
        return Err(sqlx::Error::InvalidArgument("Weekly Workout log doesn't exist".to_string()))
    }

    let formatted_date = database_utils::format_naive_date_for_database(&date);

    sqlx::query(
        "INSERT INTO logs (date, username, value, log_type)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(username, date)
         DO UPDATE SET value = excluded.value;",
    )
        .bind(formatted_date)
        .bind(username)
        .bind(weight)
        .bind(log_type.to_string())
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_user_log(
    pool: &SqlitePool,
    username: &str,
    log_type: GoalType,
) -> Result<Log, sqlx::Error> {

    if log_type == GoalType::WeeklyWorkouts {
        return Err(sqlx::Error::InvalidArgument("Weekly Workout log doesn't exist".to_string()))
    }

    let rows: Vec<(NaiveDate, f32)> = sqlx::query_as("
        SELECT date, value
        FROM logs
        WHERE username = ?
        AND log_type = ?
        ORDER BY date;
    ")
        .bind(username)
        .bind(log_type.to_string())
        .fetch_all(pool)
        .await?;

    Ok(rows)
}
