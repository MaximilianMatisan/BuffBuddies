use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::backend::image_manager::ImageManager;
use crate::client::backend::login_state::LoginState;
use crate::client::backend::mascot_manager::MascotManager;
use crate::client::backend::pop_up_manager::PopUpManager;
use crate::client::backend::user_manager::UserManager;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::activity_widget::activity::{
    ActivityWidget, calculate_activity_data,
};
use crate::client::gui::bb_widget::graph::GraphWidgetState;
use crate::common::mascot_mod::mascot::Mascot;

pub struct App {
    pub loading: bool,
    pub screen: Tab,
    pub activity_widget: ActivityWidget,
    pub graph_widget_state: GraphWidgetState,
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
                calculate_activity_data(&exercise_manager.exercises),
            ),
            user_manager: UserManager::new(&exercise_manager.exercises),
            mascot_manager: MascotManager::default(),
            exercise_manager,
            graph_widget_state: GraphWidgetState::new(),
            image_manager: ImageManager::default(),
            pop_up_manager: PopUpManager::default(),
        }
    }
}
