use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Tab {
    //TAB-BUTTONS
    Home,
    Workout,
    Health,
    Social,
    Mascot,
    Settings,
    Exit,
    //Further frames
    ViewProfile,
    CreateWorkout,
    CreatePreset,
}

impl Tab {
    /// Shown tab categories on the left hand side of the logged-in view
    pub fn get_tab_button_categories() -> [Tab; 7] {
        [
            Tab::Home,
            Tab::Workout,
            Tab::Health,
            Tab::Social,
            Tab::Mascot,
            Tab::Settings,
            Tab::Exit,
        ]
    }
}
