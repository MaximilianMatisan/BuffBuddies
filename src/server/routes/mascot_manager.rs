use crate::common::mascot_mod::mascot_data_transfer::MascotDataServerClientTransfer;
use crate::server::database_mod::database;
use crate::server::jwt::user_authentication_request_path::UserAuthenticationRequestPath;
use crate::server::server_main::ApiError;
use axum::Json;
use axum::extract::State;
use sqlx::SqlitePool;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MascotJson {
    name: String,
}
pub async fn save_mascot(Json(mascot): Json<MascotJson>) {
    println!("User purchased {}", mascot.name)
}
pub async fn get_mascot_data(
    State(pool): State<SqlitePool>,
    user_authentication: UserAuthenticationRequestPath,
) -> Result<Json<MascotDataServerClientTransfer>, ApiError> {
    let selected_mascot =
        database::get_user_selected_mascot(&pool, &user_authentication.username).await?;
    let owned_mascots =
        database::get_mascots_from_user(&pool, &user_authentication.username).await?;

    let mascot_data = MascotDataServerClientTransfer {
        selected_mascot,
        owned_mascots,
    };

    Ok(Json(mascot_data))
}
