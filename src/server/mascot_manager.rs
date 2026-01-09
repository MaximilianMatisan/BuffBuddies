use axum::Json;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Mascot {
    name: String,
}
pub async fn save_mascot(Json(mascot): Json<Mascot>) {
    println!("User purchased {}", mascot.name)
}
