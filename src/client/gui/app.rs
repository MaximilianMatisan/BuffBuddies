use crate::client::backend::exercise::exercise_manager::{
    ExerciseManager, calculate_activity_data,
};
use crate::client::backend::image::image_manager::ImageManager;
use crate::client::backend::login_state::LoginState;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::mascot_manager::MascotManager;
use crate::client::backend::pop_up_manager::PopUpManager;
use crate::client::backend::user_mod::user_manager::UserManager;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::activity_widget::activity::ActivityWidget;

pub struct App {
    pub loading: bool,
    pub screen: Tab,
    pub activity_widget: ActivityWidget,
    pub login_state: LoginState,
    pub mascot_manager: MascotManager,
    pub exercise_manager: ExerciseManager,
    pub user_manager: UserManager,
    pub image_manager: ImageManager,
    pub pop_up_manager: PopUpManager,
}

impl Default for App {
    fn default() -> Self {
        let exercise_manager = ExerciseManager::default();
        let default_mascot = Mascot::default();
        App {
            loading: false,
            screen: Tab::Home,
            login_state: LoginState::default(),
            activity_widget: ActivityWidget::new(
                default_mascot,
                calculate_activity_data(&exercise_manager.exercise_stats),
            ),
            user_manager: UserManager::new(&exercise_manager.exercise_stats),
            mascot_manager: MascotManager::default(),
            exercise_manager,
            image_manager: ImageManager::default(),
            pop_up_manager: PopUpManager::default(),
        }
    }
}
