use crate::client::server_communication::server_communicator::SetJson;
use crate::common::exercise_mod::weight::Kg;
use crate::common::mascot_mod::epic_mascot::EpicMascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::user_mod::user_goals::UserGoals;
use crate::server::database_mod::database;
use crate::server::routes::workout::ExerciseJson;
use chrono::{Duration, Local, NaiveDate};
use rand::Rng;
use sqlx::SqlitePool;
use std::string::ToString;

#[allow(dead_code)]
const DEMO_USERNAME: &str = "User";

/// CURRENT PREREQUISITE user with `username` exists in database
/// Configures demo account in database
#[allow(dead_code)]
pub async fn add_demo_data_to_user(pool: &SqlitePool, username: &str) -> Result<(), sqlx::Error> {
    //register_demo_user(pool).await?;
    configure_users_table_for_demo_user(pool, username).await?;
    configure_owned_mascots_for_demo_user(pool, username).await?;
    configure_goals_for_demo_user(pool, username).await?;
    configure_presets_for_demo_user(pool, username).await?;
    configure_exercise_log_for_demo_user(pool, username).await?;

    Ok(())
}

async fn configure_users_table_for_demo_user(
    pool: &SqlitePool,
    username: &str,
) -> Result<(), sqlx::Error> {
    database::update_user_description(pool, username, "I love BuffBuddies :)").await?;
    database::update_user_coin_balance(pool, username, 650).await?;

    Ok(())
}

async fn configure_owned_mascots_for_demo_user(
    pool: &SqlitePool,
    username: &str,
) -> Result<(), sqlx::Error> {
    database::add_mascot_to_user(pool, username, EpicMascot::Capybara.get_name()).await?;
    Ok(())
}
async fn configure_goals_for_demo_user(
    pool: &SqlitePool,
    username: &str,
) -> Result<(), sqlx::Error> {
    let goals = UserGoals {
        weekly_workouts: 2.0,
        ..Default::default()
    };
    database::update_user_goals(pool, username, goals).await?;
    Ok(())
}

async fn configure_presets_for_demo_user(
    _pool: &SqlitePool,
    _username: &str,
) -> Result<(), sqlx::Error> {
    //TODO add some presets if functions are available
    Ok(())
}
async fn configure_exercise_log_for_demo_user(
    pool: &SqlitePool,
    username: &str,
) -> Result<(), sqlx::Error> {
    let mut date_iterator = NaiveDate::from_ymd_opt(2025, 6, 12).unwrap();
    let end_date = Local::now().date_naive();

    let mut rng = rand::rng();

    let weight_step: Kg = 1.0;

    // Exercise names and their base weight
    let mut exercise_names: [(&str, Kg); 5] = [
        ("Wide-Grip Barbell Bench Press", 40.0),
        ("Butterfly", 30.0),
        ("Wide-Grip Lat Pulldown", 50.0),
        ("Triceps Pushdown - Rope Attachment", 10.0),
        ("Leg Extensions", 60.0),
    ];

    while date_iterator <= end_date {
        let sets_tracked_on_this_day = rng.random_bool(0.5);

        if !sets_tracked_on_this_day {
            date_iterator += Duration::days(1);
            continue;
        }

        let mut exercise_data_of_this_day = Vec::new();
        for (exercise_name, base_weight) in &mut exercise_names {
            let weight_step_factor = rng.random_range(-1..=2);

            *base_weight += weight_step_factor as Kg * weight_step;

            let exercise_json = ExerciseJson {
                name: exercise_name.to_string(),
                sets: vec![SetJson {
                    weight: *base_weight,
                    reps: 5,
                }],
            };
            exercise_data_of_this_day.push(exercise_json)
        }

        database::add_workout_to_exercise_log(
            pool,
            username,
            exercise_data_of_this_day,
            date_iterator,
        )
        .await?;

        date_iterator += Duration::days(1);
    }
    Ok(())
}
