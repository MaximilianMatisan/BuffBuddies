use crate::client::backend::exercise_mod::exercise::generate_example_exercise;
use crate::client::backend::mascot_mod::epic_mascot::EpicMascot;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::rare_mascot::RareMascot;
use crate::client::backend::user_mod::user::{Gender, User};

pub struct UserManager {
    pub loaded_users: Vec<User>,
}
impl Default for UserManager {
    fn default() -> Self {
        //TODO delete examples
        let test_user1 = User {
            username: "Felix".to_string(),
            description: "The boss".to_string(),
            profile_picture_handle: "assets/images/profile_picture.png".to_string(),
            weight: 75.0,
            height: 187,
            gender: Gender::Male,
            exercise_stats: vec![generate_example_exercise("Benchpress".to_string(), 49, 80.0)],
            weekly_workout_goal: 5,
            weekly_workout_streak: 12,
            coin_balance: 12381,
            favorite_mascot: Mascot::Rare(RareMascot::Chameleon),
            selected_mascot: Mascot::Rare(RareMascot::Chameleon),
            friends_with_active_user: true,
        };
        let test_user2 = User {
            username: "Stefano".to_string(),
            description: "The beast".to_string(),
            profile_picture_handle: "assets/images/profile_picture.png".to_string(),
            weight: 70.0,
            height: 178,
            gender: Gender::Male,
            exercise_stats: vec![generate_example_exercise("Preacher curl".to_string(), 180, 40.0)],
            weekly_workout_goal: 7,
            weekly_workout_streak: 19,
            coin_balance: 2972,
            favorite_mascot: Mascot::Rare(RareMascot::Whale),
            selected_mascot: Mascot::Rare(RareMascot::Duck),
            friends_with_active_user: true,
        };
        let test_user3 = User {
            username: "Robert".to_string(),
            description: "The titan".to_string(),
            profile_picture_handle: "assets/images/profile_picture.png".to_string(),
            weight: 68.0,
            height: 188,
            gender: Gender::Male,
            exercise_stats: vec![],
            weekly_workout_goal: 5,
            weekly_workout_streak: 9,
            coin_balance: 90,
            favorite_mascot: Mascot::Epic(EpicMascot::Capybara),
            selected_mascot: Mascot::Rare(RareMascot::Dog),
            friends_with_active_user: true,
        };
        let test_user4 = User {
            username: "JohnP".to_string(),
            description: "always on my phone".to_string(),
            profile_picture_handle: "assets/images/profile_picture.png".to_string(),
            weight: 100.0,
            height: 150,
            gender: Gender::Male,
            exercise_stats: vec![],
            weekly_workout_goal: 1,
            weekly_workout_streak: 2,
            coin_balance: 200,
            favorite_mascot: Mascot::Epic(EpicMascot::Shark),
            selected_mascot: Mascot::Rare(RareMascot::Duck),
            friends_with_active_user: false,
        };
        UserManager {
            loaded_users: vec![test_user1, test_user2, test_user3, test_user4]
        }
    }
}

impl UserManager {
    pub fn get_user_by_username(&self, username: &str) -> Option<&User> {
        self.loaded_users
            .iter()
            .find(|user| user.username.eq_ignore_ascii_case(username))
    }
    pub fn get_friends(&self) -> Vec<&User> {
        self.loaded_users.iter().filter(|user| user.friends_with_active_user).collect()
    }
    pub fn get_non_friend_users(&self) -> Vec<&User> {
        self.loaded_users.iter().filter(|user| !user.friends_with_active_user).collect()
    }
    pub fn add_user_as_friend(&mut self, username: &str) {
        let user_opt =
            self.loaded_users
                .iter_mut()
                .find(|user| user.username.eq_ignore_ascii_case(username));

        
        match user_opt {
            Some(user) => user.friends_with_active_user = true,
            None => return
        }
    }
}