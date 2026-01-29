use crate::common::mascot_mod::mascot::MascotError;
use crate::client::backend::mascot_manager::MascotManager;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use iced::{Color, color};
use rand::Rng;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter, Default)]
pub enum EpicMascot {
    #[default]
    Capybara,
    Shark,
    Reindeer,
}

impl MascotTrait for EpicMascot {
    fn get_name(&self) -> &str {
        match self {
            EpicMascot::Capybara => "Capybara",
            EpicMascot::Shark => "Shark",
            EpicMascot::Reindeer => "Reindeer",
        }
    }

    fn get_file_path(&self) -> &str {
        match self {
            EpicMascot::Capybara => "assets/images/mascots/capybara.png",
            EpicMascot::Shark => "assets/images/mascots/shark.png",
            EpicMascot::Reindeer => "assets/images/mascots/reindeer.png",
        }
    }

    fn get_primary_color(&self) -> Color {
        match self {
            EpicMascot::Capybara => color!(174, 87, 171),
            EpicMascot::Shark => color!(145, 140, 134),
            EpicMascot::Reindeer => color!(234, 35, 0),
        }
    }
    fn get_secondary_color(&self) -> Color {
        match self {
            EpicMascot::Capybara => color!(212, 156, 210),
            EpicMascot::Shark => color!(200, 196, 185),
            EpicMascot::Reindeer => color!(244, 99, 86),
        }
    }
    fn get_dark_color(&self) -> Color {
        match self {
            EpicMascot::Capybara => color!(102, 52, 91),
            EpicMascot::Shark => color!(113, 103, 93),
            EpicMascot::Reindeer => color!(134, 34, 16),
        }
    }
}

impl EpicMascot {
    pub fn random_new_epic(mascot_manager: &MascotManager) -> Result<EpicMascot, MascotError> {
        let mut number = rand::rng().random_range(0..=EpicMascot::COUNT - 1);
        let mut mascot = EpicMascot::iter().get(number).unwrap();
        let mut counter = 0;
        while counter < EpicMascot::COUNT && mascot_manager.owns_mascot(mascot) {
            number = (number + 1) % EpicMascot::COUNT;
            mascot = EpicMascot::iter().get(number).unwrap();
            counter += 1;
        }
        if counter == EpicMascot::COUNT {
            Err(MascotError::AllBought)
        } else {
            Ok(mascot)
        }
    }
}
