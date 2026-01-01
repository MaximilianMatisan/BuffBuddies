use crate::client::backend::mascot_mod::mascot::MascotError;
use crate::client::backend::mascot_mod::mascot_manager::MascotManager;
use crate::client::backend::mascot_mod::mascot_trait::MascotTrait;
use iced::{Color, color};
use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter)]
pub enum RareMascot {
    #[default]
    Duck,
    Dog,
    Chameleon,
    Whale,
}

impl MascotTrait for RareMascot {
    fn get_name(&self) -> &str {
        match self {
            RareMascot::Duck => "Duck",
            RareMascot::Dog => "Dog",
            RareMascot::Chameleon => "Chameleon",
            RareMascot::Whale => "Whale",
        }
    }

    fn get_file_path(&self) -> &str {
        match self {
            RareMascot::Duck => "assets/images/mascots/duck.png",
            RareMascot::Dog => "assets/images/mascots/dog.png",
            RareMascot::Chameleon => "assets/images/mascots/chameleon.png",
            RareMascot::Whale => "assets/images/mascots/whale.png",
        }
    }

    fn get_primary_color(&self) -> Color {
        match self {
            RareMascot::Duck => color!(240, 147, 67),
            RareMascot::Dog => color!(166, 95, 35),
            RareMascot::Chameleon => color!(138, 206, 24),
            RareMascot::Whale => color!(67, 164, 227),
        }
    }
    fn get_secondary_color(&self) -> Color {
        match self {
            RareMascot::Duck => color!(247, 207, 86),
            RareMascot::Dog => color!(221, 144, 80),
            RareMascot::Chameleon => color!(204, 233, 118),
            RareMascot::Whale => color!(178, 225, 255),
        }
    }
    fn get_dark_color(&self) -> Color {
        match self {
            RareMascot::Duck => color!(152, 95, 44),
            RareMascot::Dog => color!(114, 62, 19),
            RareMascot::Chameleon => color!(62, 119, 21),
            RareMascot::Whale => color!(30, 86, 141),
        }
    }
}

impl RareMascot {
    pub fn random_new_rare(mascot_manager: &MascotManager) -> Result<RareMascot, MascotError> {
        let mut number = rand::rng().random_range(0..=RareMascot::COUNT - 1);
        let mut mascot = RareMascot::iter().get(number).unwrap();
        let mut counter = 0;
        while counter < RareMascot::COUNT && mascot_manager.owns_mascot(mascot) {
            number = (number + 1) % RareMascot::COUNT;
            mascot = RareMascot::iter().get(number).unwrap();
            counter += 1;
        }
        if counter == RareMascot::COUNT {
            Err(MascotError::AllBought)
        } else {
            Ok(mascot)
        }
    }
}
