use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use sqlx::{Row, SqlitePool};
use std::str::FromStr;

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

pub fn mascot_from_string(name: &str) -> Mascot {
    for mascot in Mascot::iter() {
        if mascot.get_name() == name {
            return mascot;
        }
    }
    Mascot::default()
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
