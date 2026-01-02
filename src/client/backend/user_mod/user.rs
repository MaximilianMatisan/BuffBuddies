use crate::client::backend::exercise_mod::exercise::Exercise;
use crate::client::backend::exercise_mod::weight::Kg;
use crate::client::backend::mascot_mod::mascot::Mascot;

pub enum Gender{
    Male,
    Female
}
pub struct User {
    pub username: String,
    pub description: String,
    pub profile_picture_handle: String,
    pub weight: Kg,
    pub height: u32,
    pub gender: Gender,
    pub exercise_stats: Vec<Exercise>,
    pub weekly_workout_goal: u32,
    pub weekly_workout_streak: u32,
    pub coin_balance: u32,
    pub favorite_mascot: Mascot,
    pub selected_mascot: Mascot,
    pub friends_with_active_user: bool
}
impl Default for User {
    fn default() -> Self {
        User {
            username: "Default_user".to_string(),
            description: "Hi, I am a user and I like to work out!".to_string(),
            profile_picture_handle: "assets/images/profile_picture.png".to_string(),
            weight: 60.0,
            height: 170,
            gender: Gender::Male,
            exercise_stats: vec![],
            weekly_workout_goal: 4,
            weekly_workout_streak: 0,
            coin_balance: 0,
            favorite_mascot: Default::default(),
            selected_mascot: Default::default(),
            friends_with_active_user: false
        }
    }
}
