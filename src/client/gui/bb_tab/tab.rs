use strum_macros::Display;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Tab {
    //TAB-BUTTONS
    Home,
    Workout,
    Social,
    Mascot,
    Settings,
    Exit,
    //Further frames
    ViewProfile,
}

impl Tab {
    pub fn get_tab_button_categories() -> [Tab; 6] {
        [
            Tab::Home,
            Tab::Workout,
            Tab::Social,
            Tab::Mascot,
            Tab::Settings,
            Tab::Exit,
        ]
    }
}
