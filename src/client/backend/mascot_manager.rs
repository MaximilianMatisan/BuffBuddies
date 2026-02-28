use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot_data_transfer::MascotDataServerClientTransfer;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::widget::combo_box::State;
use iced::widget::{Image, image};
use rand::random_range;

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
            owned_mascots_state: State::with_selection(vec![Mascot::default()], None),
        }
    }

    pub fn owns_mascot(&self, mascot: impl Into<Mascot>) -> bool {
        self.owned_mascots.contains(&mascot.into())
    }

    pub fn add_mascot(&mut self, mascot: impl Into<Mascot>) {
        self.owned_mascots.push(mascot.into());
        self.owned_mascots_state = State::with_selection(self.owned_mascots.clone(), None)
    }
    pub fn update_mascot_manager_on_login(&mut self, mascot_data: MascotDataServerClientTransfer) {
        self.selected_mascot = mascot_data.selected_mascot;
        self.owned_mascots = mascot_data.owned_mascots;
        self.owned_mascots_state = State::with_selection(self.owned_mascots.clone(), None)
    }

    pub fn view_active_mascot(&self) -> Image {
        let image = image(self.selected_mascot.get_file_path());
        image
    }

    pub fn get_random_owned_mascot(&self) -> Mascot {
        if self.owned_mascots.is_empty() {
            // Avoid calling random_range with 0..0, which would panic if no mascots are owned
            return Mascot::default();
        }

        let enumerated_mascots: Vec<(usize, &Mascot)> =
            self.owned_mascots.iter().enumerate().collect();
        let random_number = random_range(0..enumerated_mascots.len());

        *enumerated_mascots.get(random_number).unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::mascot_mod::epic_mascot::EpicMascot;
    use crate::common::mascot_mod::rare_mascot::RareMascot;

    #[test]
    fn owns_mascot_false() {
        let mascot_manager = MascotManager::default();
        assert!(!mascot_manager.owns_mascot(EpicMascot::Reindeer));
    }
    #[test]
    fn owns_mascot_true() {
        let mut mascot_manager = MascotManager::default();
        mascot_manager.add_mascot(EpicMascot::Reindeer);
        assert!(mascot_manager.owns_mascot(EpicMascot::Reindeer));
    }
    #[test]
    fn update_on_login() {
        let mut mascot_manager = MascotManager::default();
        mascot_manager.update_mascot_manager_on_login(MascotDataServerClientTransfer {
            selected_mascot: Default::default(),
            owned_mascots: vec![
                Mascot::Rare(RareMascot::Duck),
                Mascot::Epic(EpicMascot::Reindeer),
            ],
        });
        assert!(mascot_manager.owns_mascot(EpicMascot::Reindeer));
        assert_eq!(mascot_manager.selected_mascot, Mascot::default())
    }
}
