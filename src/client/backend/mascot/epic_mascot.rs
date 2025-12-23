use rand::Rng;
use strum_macros::{EnumCount, EnumIter};
use strum::{EnumCount, IntoEnumIterator};
use crate::client::backend::mascot::mascot_trait::MascotTrait;
use iced::{color, Color};
use crate::client::backend::mascot::mascot::MascotError;
use crate::client::backend::mascot::mascot_manager::MascotManager;

#[derive(Debug, Clone, Copy, PartialEq, EnumCount, EnumIter, Default)]
pub enum EpicMascot {
    #[default]
    Capybara,
    Shark,
}

impl MascotTrait for EpicMascot {
    fn get_name(&self) -> &str {
        match self {
            EpicMascot::Capybara => "Capybara",
            EpicMascot::Shark => "Shark",
        }
    }

    fn get_file_path(&self) -> &str {
        match self {
            EpicMascot::Capybara => "assets/images/mascots/capybara.png",
            EpicMascot::Shark => "assets/images/mascots/shark.png",
        }
    }

    fn get_primary_color(&self) -> Color {
        match self {
            EpicMascot::Capybara => color!(174, 87, 171),
            EpicMascot::Shark => color!(145,140,134),
        }
    }
    fn get_secondary_color(&self) -> Color {
        match self {
            EpicMascot::Capybara => color!(212,156,210),
            EpicMascot::Shark => color!(200,196,185),
        }
    }
    fn get_dark_color(&self) -> Color {
        match self {
            EpicMascot::Capybara => color!(102,52,91),
            EpicMascot::Shark => color!(113,103,93)
        }
    }
}

impl EpicMascot {

    pub fn random_new_epic(mascot_manager: &MascotManager) -> Result<EpicMascot, MascotError> {
        let mut number = rand::rng().random_range(0..=EpicMascot::COUNT-1);
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
