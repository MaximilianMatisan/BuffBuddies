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
        App {
            screen: Tab::Home,
            active_mascot: Mascot::Duck,
            activity_widget: ActivityWidget::default()
        }
    }
}