use crate::client::backend::exercise_mod::ExerciseManager;
use crate::client::backend::image::image_manager::ImageManager;
use crate::client::backend::login_state::LoginState;
use crate::client::backend::mascot_mod::mascot::Mascot;
use crate::client::backend::mascot_mod::mascot_manager::MascotManager;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::activity_widget::activity::ActivityWidget;

pub struct App {
    pub loading: bool,
    pub screen: Tab,
    pub activity_widget: ActivityWidget,
    pub login_state: LoginState,
    pub mascot_manager: MascotManager,
    pub exercise_manager: ExerciseManager,
    pub image_manager: ImageManager
}

impl Default for App {
    fn default() -> Self {
        let default_mascot = Mascot::default();
        App {
            loading: false,
            screen: Tab::Home,
            login_state: LoginState::default(),
            activity_widget: ActivityWidget::new(default_mascot),
            mascot_manager: MascotManager::default(),
            exercise_manager: ExerciseManager::default(),
            image_manager: ImageManager::default()
        }
    }
}
