use std::fmt::{Display, Formatter};
use iced_core::Color;
use crate::client::backend::mascot::epic_mascot::EpicMascot;
use crate::client::backend::mascot::rare_mascot::RareMascot;
use crate::client::backend::mascot::mascot_trait::MascotTrait;

pub enum MascotError {
    AllBought,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mascot{
    Rare(RareMascot),
    Epic(EpicMascot),
}

impl Display for Mascot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Rare(rare) => rare.get_name(),
            Self::Epic(epic) => epic.get_name(),
        })
    }
}

impl MascotTrait for Mascot {
    fn get_name(&self) -> &str {
        match self {
            Mascot::Rare(rare) => rare.get_name(),
            Mascot::Epic(epic) => epic.get_name(),
        }
    }
    fn get_file_path(&self) -> &str {
        match self {
            Mascot::Rare(rare) => rare.get_file_path(),
            Mascot::Epic(epic) => epic.get_file_path(),
        }
    }

    fn get_primary_color(&self) -> Color {
        match self {
            Mascot::Rare(rare_mascot) => rare_mascot.get_primary_color(),
            Mascot::Epic(epic_mascot) => epic_mascot.get_primary_color(),
        }
    }
    fn get_secondary_color(&self) -> Color {
        match self {
            Mascot::Rare(rare_mascot) => rare_mascot.get_secondary_color(),
            Mascot::Epic(epic_mascot) => epic_mascot.get_secondary_color(),
        }
    }
    fn get_dark_color(&self) -> Color {
        match self {
            Mascot::Rare(rare_mascot) => rare_mascot.get_dark_color(),
            Mascot::Epic(epic_mascot) => epic_mascot.get_dark_color(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MascotRarity {
    Rare,
    Epic
}

impl From<RareMascot> for Mascot {
    fn from(rare: RareMascot) -> Self {
        Mascot::Rare(rare)
    }
}

impl From<EpicMascot> for Mascot {
    fn from(epic: EpicMascot) -> Self {
        Mascot::Epic(epic)
    }
}

impl Default for Mascot {
    fn default() -> Self {
        Mascot::Rare(RareMascot::Duck)
    }
}