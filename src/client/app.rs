use crate::client::bb_tab::tab::Tab;
use crate::client::bb_widget::activity::activity::ActivityWidget;
use crate::client::mascots::Mascot;

pub struct App {
    pub screen: Tab,
    pub active_mascot: Mascot,
    pub activity_widget: ActivityWidget,
}
impl Default for App {
    fn default() -> Self {
        let default_mascot = Mascot::default();
        App {
            screen: Tab::Home,
            active_mascot: default_mascot.clone(),
            activity_widget: ActivityWidget::new(default_mascot)
        }
    }
}