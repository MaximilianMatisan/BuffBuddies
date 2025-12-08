use crate::client::bb_tab::tab::Tab;
use crate::client::mascots::Mascot;

pub struct App {
    pub screen: Tab,
    pub active_mascot: Mascot,
}
impl Default for App {
    fn default() -> Self {
        App {
            screen: Tab::Home,
            active_mascot: Mascot::Duck,
        }
    }
}