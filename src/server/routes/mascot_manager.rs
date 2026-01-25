use axum::Json;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MascotJson {
    name: String,
}
pub async fn save_mascot(Json(mascot): Json<MascotJson>) {
    println!("User purchased {}", mascot.name)
}
