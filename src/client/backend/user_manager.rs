use crate::common::exercise_mod::exercise::Exercise;
use crate::common::user_mod::user::{
    ForeignUser, Gender, UserInformation, UserInformationStrings, UserType,
};
use iced::widget::combo_box;
use strum::IntoEnumIterator;

pub struct UserManager {
    /// Contains general information about the currently logged-in user.
    pub user_info: UserInformation,
    /// Contains information if the user is currently inside of edit mode of the user_info in the settings
    pub pending_user_info_changes: Option<(UserInformation, UserInformationStrings)>,

    /// Info about all loaded non-logged-in users
    pub loaded_users: Vec<ForeignUser>,
    pub most_recently_viewed_user: UserType,

    //Utils
    pub gender_combo_box_state: combo_box::State<Gender>,
}
impl UserManager {
    pub fn new(exercise_data: &Vec<Exercise>) -> Self {
        UserManager {
            user_info: UserInformation::default(exercise_data),
            pending_user_info_changes: None,
            loaded_users: vec![],
            most_recently_viewed_user: UserType::Own,
            gender_combo_box_state: combo_box::State::new(Gender::iter().collect()),
        }
    }
}

impl UserManager {
    pub fn get_user_by_username(&self, username: &str) -> Option<&ForeignUser> {
        self.loaded_users
            .iter()
            .find(|user| user.username.eq_ignore_ascii_case(username))
    }
    pub fn get_user_by_username_mut(&mut self, username: &str) -> Option<&mut ForeignUser> {
        self.loaded_users
            .iter_mut()
            .find(|user| user.username.eq_ignore_ascii_case(username))
    }
    pub fn get_friends(&self) -> Vec<&ForeignUser> {
        self.loaded_users
            .iter()
            .filter(|user| user.friends_with_active_user)
            .collect()
    }
    pub fn get_non_friend_users(&self) -> Vec<&ForeignUser> {
        self.loaded_users
            .iter()
            .filter(|user| !user.friends_with_active_user)
            .collect()
    }

    /// Returns whether adding a friend was successful or not
    pub fn add_user_as_friend(&mut self, username: &str) -> bool {
        let user_opt = self.get_user_by_username_mut(username);

        if let Some(user) = user_opt {
            user.friends_with_active_user = true;
            true
        } else {
            false
        }
    }

    /// Returns whether deletion was successful or not
    pub fn remove_user_as_friend(&mut self, username: &str) -> bool {
        let user_opt = self.get_user_by_username_mut(username);

        if let Some(user) = user_opt {
            user.friends_with_active_user = false;
            true
        } else {
            false
        }
    }
}
