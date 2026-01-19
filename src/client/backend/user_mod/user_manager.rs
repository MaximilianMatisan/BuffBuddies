use crate::client::backend::exercise::exercise_stats::{ExerciseStat, generate_example_exercise};
use crate::client::backend::mascot_mod::epic_mascot::EpicMascot;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::rare_mascot::RareMascot;
use crate::client::backend::profile_stat_manager::ProfileStatManager;
use crate::client::backend::user_mod::user::{
    ForeignUser, Gender, UserInformation, UserInformationStrings, UserType,
};
use iced::widget::combo_box;
use strum::IntoEnumIterator;

pub struct UserManager {
    /// Contains general information about the currently logged-in user.
    pub user_info: UserInformation,
    /// Contains information if the user is currently inside of edit mode of the user_info in the settings
    pub pending_user_info_changes: Option<(UserInformation, UserInformationStrings)>,

    pub loaded_users: Vec<ForeignUser>,
    pub most_recently_viewed_user: UserType,

    //Utils
    pub gender_combo_box_state: combo_box::State<Gender>,
}
impl UserManager {
    pub fn new(exercise_data: &Vec<ExerciseStat>) -> Self {
        //TODO delete examples
        let user1_exercises = vec![generate_example_exercise(
            "Benchpress".to_string(),
            49,
            80.0,
        )];
        let user1_mascots = vec![Mascot::Rare(RareMascot::Chameleon)];
        let test_user1 = ForeignUser {
            user_information: UserInformation {
                username: "Felix".to_string(),
                description: "The boss".to_string(),
                profile_picture_handle: "assets/images/profile_picture.png".to_string(),
                weight: 75.0,
                height: 187,
                gender: Gender::Male,
                weekly_workout_goal: 5,
                weekly_workout_streak: 12,
                coin_balance: 12381,
                favorite_mascot: Mascot::Rare(RareMascot::Chameleon),
                profile_stat_manager: ProfileStatManager::new(&user1_exercises),
            },
            exercise_stats: user1_exercises,
            selected_mascot: Mascot::Rare(RareMascot::Chameleon),
            owned_mascots: user1_mascots,
            friends_with_active_user: false,
        };
        let user2_exercises = vec![generate_example_exercise(
            "Preacher curl".to_string(),
            180,
            40.0,
        )];
        let user2_mascots = vec![
            Mascot::Rare(RareMascot::Whale),
            Mascot::Rare(RareMascot::Duck),
        ];
        let test_user2 = ForeignUser {
            user_information: UserInformation {
                username: "Stefano".to_string(),
                description: "The beast".to_string(),
                profile_picture_handle: "assets/images/profile_picture.png".to_string(),
                weight: 70.0,
                height: 178,
                gender: Gender::Male,
                weekly_workout_goal: 7,
                weekly_workout_streak: 19,
                coin_balance: 2972,
                favorite_mascot: Mascot::Rare(RareMascot::Whale),
                profile_stat_manager: ProfileStatManager::new(&user2_exercises),
            },
            exercise_stats: user2_exercises,
            selected_mascot: Mascot::Rare(RareMascot::Duck),
            owned_mascots: user2_mascots,
            friends_with_active_user: false,
        };
        let user3_exercises = vec![];
        let user3_mascots = vec![
            Mascot::Epic(EpicMascot::Capybara),
            Mascot::Rare(RareMascot::Dog),
        ];
        let test_user3 = ForeignUser {
            user_information: UserInformation {
                username: "Robert".to_string(),
                description: "The titan".to_string(),
                profile_picture_handle: "assets/images/profile_picture.png".to_string(),
                weight: 68.0,
                height: 188,
                gender: Gender::Male,
                weekly_workout_goal: 5,
                weekly_workout_streak: 9,
                coin_balance: 90,
                favorite_mascot: Mascot::Epic(EpicMascot::Capybara),
                profile_stat_manager: ProfileStatManager::new(&user3_exercises),
            },
            exercise_stats: user3_exercises,
            selected_mascot: Mascot::Rare(RareMascot::Dog),
            owned_mascots: user3_mascots,
            friends_with_active_user: false,
        };
        let user4_exercises = vec![];
        let user4_mascots = vec![
            Mascot::Epic(EpicMascot::Shark),
            Mascot::Rare(RareMascot::Duck),
        ];
        let test_user4 = ForeignUser {
            user_information: UserInformation {
                username: "JohnP".to_string(),
                description: "always on my phone".to_string(),
                profile_picture_handle: "assets/images/profile_picture.png".to_string(),
                weight: 100.0,
                height: 150,
                gender: Gender::Male,
                weekly_workout_goal: 1,
                weekly_workout_streak: 2,
                coin_balance: 200,
                favorite_mascot: Mascot::Epic(EpicMascot::Shark),
                profile_stat_manager: ProfileStatManager::new(&user4_exercises),
            },
            exercise_stats: user4_exercises,
            selected_mascot: Mascot::Rare(RareMascot::Duck),
            owned_mascots: user4_mascots,
            friends_with_active_user: false,
        };
        UserManager {
            user_info: UserInformation::default(exercise_data),
            pending_user_info_changes: None,
            loaded_users: vec![test_user1, test_user2, test_user3, test_user4],
            most_recently_viewed_user: UserType::Own,
            gender_combo_box_state: combo_box::State::new(Gender::iter().collect()),
        }
    }
}

impl UserManager {
    pub fn get_user_by_username(&self, username: &str) -> Option<&ForeignUser> {
        self.loaded_users.iter().find(|user| {
            user.user_information
                .username
                .eq_ignore_ascii_case(username)
        })
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
    pub fn add_user_as_friend(&mut self, username: &str) {
        let user_opt = self.loaded_users.iter_mut().find(|user| {
            user.user_information
                .username
                .eq_ignore_ascii_case(username)
        });

        if let Some(user) = user_opt {
            user.friends_with_active_user = true;
        }
    }
}
