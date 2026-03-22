use crate::common::exercise_mod::weight::Kg;
use crate::common::user_mod::user_goals::UserGoals;
use sqlx::{Row, SqlitePool};

pub async fn update_user_goals(
    pool: &SqlitePool,
    username: &str,
    user_goals: &UserGoals,
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
