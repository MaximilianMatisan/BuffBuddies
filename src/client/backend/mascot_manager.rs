use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot::Mascot::Rare;
use crate::common::mascot_mod::rare_mascot::RareMascot;
use iced::widget::combo_box::State;

pub struct MascotManager {
    pub selected_mascot: Mascot,
    pub owned_mascots: Vec<Mascot>,
    pub owned_mascots_state: State<Mascot>,
}

impl Default for MascotManager {
    fn default() -> Self {
        MascotManager::new()
    }
}

impl MascotManager {
    pub fn new() -> Self {
        MascotManager {
            selected_mascot: Mascot::default(),
            owned_mascots: vec![Mascot::default()],
            owned_mascots_state: State::with_selection(vec![Rare(RareMascot::Duck)], None),
        }
    }

    pub fn owns_mascot(&self, mascot: impl Into<Mascot>) -> bool {
        self.owned_mascots.contains(&mascot.into())
    }

    pub fn add_mascot(&mut self, mascot: impl Into<Mascot>) {
        self.owned_mascots.push(mascot.into());
        self.owned_mascots_state = State::with_selection(self.owned_mascots.clone(), None)
    }
}
