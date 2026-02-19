use crate::client::gui::user_interface::Message;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub enum PopUpType {
    Major,
    Minor,
    Question(Arc<dyn Fn(bool) -> Message>),
}

impl Debug for PopUpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PopUpType::Minor => f.write_str("PopUpType::Minor"),
            PopUpType::Major => f.write_str("PopUpType::Major"),
            PopUpType::Question(_) => f.write_str("PopupType::Question"),
        }
    }
}
pub struct PopUpManager {
    pub title: String,
    pub text: String,
    pub major_pop_up: bool,
    pub minor_pop_up: bool,
    pub question_pop_up: Option<Arc<dyn Fn(bool) -> Message>>,
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
            question_pop_up: None,
        }
    }

    pub fn new_pop_up(&mut self, pop_up_type: PopUpType, title: String, text: String) {
        self.title = title;
        self.text = text;
        match pop_up_type {
            PopUpType::Major => self.major_pop_up = true,
            PopUpType::Minor => self.minor_pop_up = true,
            PopUpType::Question(message_fn) => {
                self.question_pop_up = Some(message_fn);
                self.minor_pop_up = true;
            }
        }
    }

    pub fn reset(&mut self) {
        self.title = "".to_string();
        self.text = "".to_string();
        self.major_pop_up = false;
        self.minor_pop_up = false;
        self.question_pop_up = None;
    }
}
