use crate::client::mascots::Mascot;

pub struct App {
    pub active_mascot: Mascot,
}
impl Default for App {
    fn default() -> Self {
        App {
            active_mascot: Mascot::Duck,
        }
    }
}