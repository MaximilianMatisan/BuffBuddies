use crate::client::backend::exercise_mod::exercise::Exercise;
use crate::client::backend::exercise_mod::weight::Kg;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::profile_stat_manager::ProfileStatManager;
use strum_macros::{Display, EnumIter};

pub const MAX_DESCRIPTION_CHARACTERS: usize = 300;
#[derive(Debug, Clone, Display, EnumIter)]
pub enum Gender {
    Male,
    Female,
}
#[derive(Debug, Clone)]
pub enum UserType {
    Own,
    Other(String),
}
#[derive(Clone)]
pub struct UserInformation {
    pub username: String,
    pub description: String,
    pub profile_picture_handle: String,
    pub weight: Kg,
    pub height: u32,
    pub gender: Gender,
    pub weekly_workout_goal: u32,
    pub weekly_workout_streak: u32,
    pub coin_balance: u32,
    pub favorite_mascot: Mascot,
    //Doesn't include "new" data only for performance, doesn't need to be in db
    pub profile_stat_manager: ProfileStatManager,
}
impl UserInformation {
    pub fn default(exercise_data: &Vec<Exercise>) -> Self {
        UserInformation {
            username: "Default_user".to_string(),
            description: "".to_string(),
            profile_picture_handle: "assets/images/profile_picture.png".to_string(),
            weight: 60.0,
            height: 170,
            gender: Gender::Male,
            weekly_workout_goal: 4,
            weekly_workout_streak: 0,
            coin_balance: 400,
            favorite_mascot: Mascot::default(),
            profile_stat_manager: ProfileStatManager::new(exercise_data),
        }
    }
}
pub struct ForeignUser {
    pub user_information: UserInformation,
    pub selected_mascot: Mascot,
    pub owned_mascots: Vec<Mascot>,
    pub friends_with_active_user: bool,
}
impl Default for ForeignUser {
    fn default() -> Self {
        let exercise_data = vec![];
        ForeignUser {
            user_information: UserInformation::default(&exercise_data),

            selected_mascot: Default::default(),
            owned_mascots: vec![Mascot::default()],
            friends_with_active_user: false,
        }
    }
}

///Only used to store temporary text_input_strings
pub struct UserInformationStrings {
    pub weight: String,
    pub height: String,
    pub weekly_workout_goal: String,
}
impl UserInformationStrings {
    pub fn new(
        weight: String,
        height: String,
        weekly_workout_goal: String,
    ) -> UserInformationStrings {
        UserInformationStrings {
            weight,
            height,
            weekly_workout_goal,
        }
    }
}
