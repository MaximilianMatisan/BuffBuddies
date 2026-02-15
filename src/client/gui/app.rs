use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::backend::image_manager::ImageManager;
use crate::client::backend::login_state::LoginState;
use crate::client::backend::mascot_manager::MascotManager;
use crate::client::backend::pop_up_manager::PopUpManager;
use crate::client::backend::user_manager::UserManager;
use crate::client::backend::widget_state_manager::WidgetManager;
use crate::client::backend::workout_preset_manager::WorkoutPresetManager;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::server_communication::request_data::LoginServerRequestData;
use crate::common::mascot_mod::mascot::Mascot;

pub struct App {
    /// Every connection to the server after the login has to contain this jwt in its json header
    /// to qualify as a valid request. E.g. see user_communicator::get_user_information_from_server
    pub jsonwebtoken: Option<String>,
    pub screen: Tab,
    pub widget_manager: WidgetManager,
    pub login_state: LoginState,
    pub mascot_manager: MascotManager,
    pub exercise_manager: ExerciseManager,
    pub workout_preset_manager: WorkoutPresetManager,
    pub user_manager: UserManager,
    pub image_manager: ImageManager,
    pub pop_up_manager: PopUpManager,
}

impl Default for App {
    fn default() -> Self {
        let exercise_manager = ExerciseManager::default();
        let _default_mascot = Mascot::default();
        App {
            jsonwebtoken: None,
            screen: Tab::Home,
            login_state: LoginState::default(),
            user_manager: UserManager::new(&exercise_manager.exercises),
            mascot_manager: MascotManager::default(),
            widget_manager: WidgetManager::new(),
            exercise_manager,
            workout_preset_manager: WorkoutPresetManager::default(),
            image_manager: ImageManager::default(),
            pop_up_manager: PopUpManager::default(),
        }
    }
}
impl App {
    pub fn update_app_on_login(&mut self, data: LoginServerRequestData) {
        self.user_manager.user_info = data.user_information;
        self.user_manager.loaded_users = data.foreign_users;
        self.mascot_manager
            .update_mascot_manager_on_login(data.mascot_data);
        self.exercise_manager.update_exercise_manager_on_login(
            data.exercises,
            self.user_manager
                .user_info
                .profile_stat_manager
                .best_pr
                .0
                .clone(),
        );
        self.widget_manager.activity_widget.update_data(
            self.mascot_manager.selected_mascot,
            self.user_manager
                .user_info
                .profile_stat_manager
                .activity_data
                .clone(),
        );
    }
}
