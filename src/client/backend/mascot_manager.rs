use iced::Element;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::mascot_mod::mascot::Mascot::Rare;
use crate::common::mascot_mod::mascot_data_transfer::MascotDataServerClientTransfer;
use crate::common::mascot_mod::rare_mascot::RareMascot;
use iced::widget::combo_box::State;
use iced::widget::{image, Image};
use iced_core::Length::Fill;
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot_trait::MascotTrait;

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
    pub fn update_mascot_manager_on_login(&mut self, mascot_data: MascotDataServerClientTransfer) {
        self.selected_mascot = mascot_data.selected_mascot;
        self.owned_mascots = mascot_data.owned_mascots;
        self.owned_mascots_state = State::with_selection(self.owned_mascots.clone(), None)
    }

    pub fn view_active_mascot(&self) -> Image {
        let image = image(self.selected_mascot.get_file_path());
        image
    }
}
