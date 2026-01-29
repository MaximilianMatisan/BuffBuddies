use crate::common::mascot_mod::epic_mascot::EpicMascot;
use crate::common::mascot_mod::mascot_trait::MascotTrait;
use crate::common::mascot_mod::rare_mascot::RareMascot;
use iced_core::Color;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;

pub enum MascotError {
    AllBought,
}

#[derive(Clone, Debug)]
pub enum MascotRarity {
    Epic,
    Rare,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mascot {
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
impl Mascot {
    pub fn iter() -> impl Iterator<Item = Mascot> {
        RareMascot::iter()
            .map(Mascot::Rare)
            .chain(EpicMascot::iter().map(Mascot::Epic))
    }
}
