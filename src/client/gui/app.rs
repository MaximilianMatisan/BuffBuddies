use crate::client::backend::login_state::LoginState;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_widget::activity::activity::ActivityWidget;
use crate::client::gui::mascots::Mascot;

pub struct App {
    pub loading: bool,
    pub screen: Tab,
    pub active_mascot: Mascot,
    pub activity_widget: ActivityWidget,
    pub login_state: LoginState,
}
impl Default for App {
    fn default() -> Self {
        let default_mascot = Mascot::default();
        App {
            loading: false,
            screen: Tab::Home,
            active_mascot: default_mascot.clone(),
            activity_widget: ActivityWidget::new(default_mascot),
            login_state: LoginState::default(),
        }
    }
}