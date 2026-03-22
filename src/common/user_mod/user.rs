use crate::client::backend::profile_stat_manager::ProfileStatManager;
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::exercise_mod::weight::Kg;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::profile_picture::ProfilePictureTypes;
use crate::common::user_mod::user_goals::{GoalType, UserGoals};
use crate::common::user_mod::user_log::UserLog;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

pub const MAX_DESCRIPTION_CHARACTERS: usize = 300;
#[derive(
    Display, Clone, EnumString, Eq, PartialEq, Debug, Default, Serialize, Deserialize, EnumIter,
)]
#[strum(ascii_case_insensitive)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub enum UserType {
    /// The currently logged-in user
    Own,
    /// Another non-logged-in user
    Other(String),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInformation {
    pub username: String,
    pub description: String,
    pub profile_picture_path: String,
    pub weight: Kg,
    pub height: u32,
    pub gender: Gender,
    pub coin_balance: u32,
    pub favorite_mascot: Mascot,
    pub user_goals: UserGoals,
    pub user_logs: UserLog,
    //Doesn't include "new" data only for performance, doesn't need to be in db
    pub profile_stat_manager: ProfileStatManager,
}
impl UserInformation {
    pub fn default(exercise_data: &Vec<Exercise>) -> Self {
        let default_user_goals = UserGoals::default();
        UserInformation {
            username: "Default_user".to_string(),
            description: "".to_string(),
            profile_picture_path: ProfilePictureTypes::ManBuff.get_image_path(),
            weight: 60.0,
            height: 170,
            gender: Gender::Male,
            coin_balance: 0,
            favorite_mascot: Mascot::default(),
            profile_stat_manager: ProfileStatManager::new(
                exercise_data,
                default_user_goals.weekly_workouts as u32,
            ),
            user_logs: UserLog::default(),
            user_goals: default_user_goals,
        }
    }
}
/// Necessary information about non-logged-in users for the logged-in user
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForeignUser {
    pub username: String,
    pub description: String,
    pub profile_picture_path: String,
    pub profile_stat_manager: ProfileStatManager,
    pub favorite_mascot: Mascot,
    pub owned_mascots: Vec<Mascot>,
    pub friends_with_active_user: bool,
}
impl Default for ForeignUser {
    fn default() -> Self {
        Self {
            username: "Default_user".to_string(),
            description: "".to_string(),
            profile_picture_path: ProfilePictureTypes::ManBuff.get_image_path(),
            profile_stat_manager: ProfileStatManager::new(
                &vec![],
                GoalType::WeeklyWorkouts.get_default_goal_value() as u32,
            ),
            favorite_mascot: Mascot::default(),
            owned_mascots: vec![Mascot::default()],
            friends_with_active_user: false,
        }
    }
}

pub struct DisplayUserProfileData {
    pub username: String,
    pub description: String,
    pub profile_picture_path: String,
    pub profile_stat_manager: ProfileStatManager,
    pub favorite_mascot: Mascot,
    pub owned_mascots: Vec<Mascot>,
    pub friends_with_active_user: bool,
}
impl Default for DisplayUserProfileData {
    fn default() -> Self {
        Self::from(ForeignUser::default())
    }
}
impl From<ForeignUser> for DisplayUserProfileData {
    fn from(value: ForeignUser) -> Self {
        Self {
            username: value.username,
            description: value.description,
            profile_picture_path: value.profile_picture_path,
            profile_stat_manager: value.profile_stat_manager,
            favorite_mascot: value.favorite_mascot,
            owned_mascots: value.owned_mascots,
            friends_with_active_user: value.friends_with_active_user,
        }
    }
}
impl DisplayUserProfileData {
    pub fn from_logged_in_user(value: UserInformation, owned_mascots: Vec<Mascot>) -> Self {
        Self {
            username: value.username,
            description: value.description,
            profile_picture_path: value.profile_picture_path,
            profile_stat_manager: value.profile_stat_manager,
            favorite_mascot: value.favorite_mascot,
            owned_mascots,
            friends_with_active_user: false,
        }
    }
}

///Only used to store temporary text_input_strings
pub struct UserInformationStrings {
    pub weight: String,
    pub height: String,
}
impl UserInformationStrings {
    pub fn new(weight: String, height: String) -> UserInformationStrings {
        UserInformationStrings { weight, height }
    }
}
