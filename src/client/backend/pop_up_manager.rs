#[derive(Debug, Clone)]
pub enum PopUpType {
    Major,
    Minor,
}

pub struct PopUpManager {
    pub title: String,
    pub text: String,
    pub major_pop_up: bool,
    pub minor_pop_up: bool,
}

impl Default for PopUpManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PopUpManager {
    pub fn new() -> Self {
        PopUpManager {
            title: "".to_string(),
            text: "".to_string(),
            major_pop_up: false,
            minor_pop_up: false,
        }
    }

    pub fn new_pop_up(&mut self, pop_up_type: PopUpType, title: String, text: String) {
        self.title = title;
        self.text = text;
        match pop_up_type {
            PopUpType::Major => self.major_pop_up = true,
            PopUpType::Minor => self.minor_pop_up = true,
        }
    }

    pub fn reset(&mut self) {
        self.title = "".to_string();
        self.text = "".to_string();
        self.major_pop_up = false;
        self.minor_pop_up = false;
    }
}
