use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_data_transfer::MascotDataServerClientTransfer;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::server::database_mod::database_mascot::add_mascot_to_user;
use crate::server::database_mod::database_user::{get_user_coin_balance, update_user_coin_balance};
use crate::server::database_mod::{database_mascot, database_user};
use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;
use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

pub async fn buy_mascot(
    user_authentication: UserAuthenticationRequestPath,
    State(pool): State<SqlitePool>,
    Json(mascot): Json<Mascot>,
) -> Result<(), ApiError> {
    add_mascot_to_user(&pool, &user_authentication.username, &mascot.to_string()).await?;
    let current_coins = get_user_coin_balance(&pool, &user_authentication.username).await?;
    update_user_coin_balance(
        &pool,
        &user_authentication.username,
        current_coins - mascot.get_prize(),
    )
    .await?;
    Ok(())
}
pub async fn get_mascot_data(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
) -> Result<Json<MascotDataServerClientTransfer>, ApiError> {
    let selected_mascot =
        database_mascot::get_user_selected_mascot(&pool, &user_authentication.username).await?;
    let owned_mascots =
        database_mascot::get_mascots_from_user(&pool, &user_authentication.username).await?;

    let mascot_data = MascotDataServerClientTransfer {
        selected_mascot,
        owned_mascots,
    };

    println!("{}: Fetching Mascot Data!", user_authentication.username);

    Ok(Json(mascot_data))
}

pub async fn select_mascot(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
    Json(mascot): Json<Mascot>,
) -> Result<(), ApiError> {
    database_user::update_user_selected_mascot(&pool, &user_authentication.username, &mascot)
        .await?;

    println!(
        "{}: Updated selected mascot to {}!",
        user_authentication.username, mascot
    );

    Ok(())
}
