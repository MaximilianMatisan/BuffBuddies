use iced::{color, Color};

#[derive(Default)]
pub enum Mascot{
    #[default]
    Duck,
    Shark
}

impl Mascot {
    pub fn get_primary_color(&self) -> Color{
        match self {
            Mascot::Duck => color!(240, 147, 67),
            Mascot::Shark => color!(145,140,134)
        }
    }
    pub fn get_secondary_color(&self) -> Color {
        match self {
            Mascot::Duck => color!(247, 207, 86),
            Mascot::Shark => color!(200,196,185)
        }
    }

    pub fn get_disabled_color(&self) -> Color {
        match self {
            Mascot::Duck => color!(152, 95, 44),
            Mascot::Shark => color!(113,103,93)
        }
    }
}