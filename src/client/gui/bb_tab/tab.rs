use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumIter)]
pub enum Tab {
    Home,
    Workout,
    Social,
    Mascot,
    Settings,
    Exit
}