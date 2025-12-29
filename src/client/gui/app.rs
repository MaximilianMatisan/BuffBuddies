use crate::client::backend::exercise::ExerciseManager;
use crate::client::backend::login_state::LoginState;
use crate::client::backend::mascot::mascot::Mascot;
use crate::client::backend::mascot::mascot_manager::MascotManager;
pub(crate) use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::activity::activity::ActivityWidget;

pub struct App {
    pub loading: bool,
    pub screen: Tab,
    pub activity_widget: ActivityWidget,
    pub login_state: LoginState,
    pub mascot_manager: MascotManager,
    pub exercise_manager: ExerciseManager,
}

impl Default for App {
    fn default() -> Self {
        let default_mascot = Mascot::default();
        App {
            loading: false,
            screen: Tab::Home,
            login_state: LoginState::default(),
            activity_widget: ActivityWidget::new(default_mascot.clone()),
            mascot_manager: MascotManager::default(),
            exercise_manager: ExerciseManager::default(),
        }
    }
}
