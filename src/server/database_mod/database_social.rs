use crate::client::backend::profile_stat_manager::ProfileStatManager;
use crate::common::user_mod::user::ForeignUser;
use crate::server::database_mod::database::get_exercises_stats;
use crate::server::database_mod::database_mascot::mascot_from_string;
use crate::server::database_mod::database_user_goals::get_user_goals;
use sqlx::{Row, SqlitePool};

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
#[allow(dead_code)]
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
    let user_goals = get_user_goals(pool, target_username).await?;

    Ok(ForeignUser {
        username: row.get("username"),
        description: row.get("description"),
        profile_picture_path: row.get("profile_picture"),
        favorite_mascot: mascot_from_string(row.get("favorite_mascot")),
        profile_stat_manager: ProfileStatManager::new(
            &exercise_stats,
            user_goals.weekly_workouts as u32,
        ),
        owned_mascots,
        friends_with_active_user: is_friend,
    })
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
