use crate::client::gui::user_interface::Message;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Clone)]
pub enum PopUpType {
    /// A popup of this type will replace the whole screen with this popup
    Major,
    /// A popup of this type will be stacked over the current tab
    Minor,
    /// Used for a "yes" or "no" question yes calls the true Message, no call the false Message
    // Rc instead of Box to be able to derive Clone
    Question(Rc<dyn Fn(bool) -> Message>),
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
    pub question_pop_up: Option<Rc<dyn Fn(bool) -> Message>>,
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_pop_up_minor() {
        let mut pop_up_manager = PopUpManager::default();
        pop_up_manager.new_pop_up(PopUpType::Minor, "title".to_string(), "text".to_string());
        assert!(!pop_up_manager.major_pop_up);
        assert!(pop_up_manager.minor_pop_up);
        assert!(pop_up_manager.question_pop_up.is_none());
        assert_eq!(pop_up_manager.title, "title".to_string());
        assert_eq!(pop_up_manager.text, "text".to_string());
    }
    #[test]
    fn new_pop_up_major() {
        let mut pop_up_manager = PopUpManager::default();
        pop_up_manager.new_pop_up(PopUpType::Major, "title".to_string(), "text".to_string());
        assert!(pop_up_manager.major_pop_up);
        assert!(!pop_up_manager.minor_pop_up);
        assert!(pop_up_manager.question_pop_up.is_none());
        assert_eq!(pop_up_manager.title, "title".to_string());
        assert_eq!(pop_up_manager.text, "text".to_string());
    }
    #[test]
    fn new_pop_up_question() {
        let mut pop_up_manager = PopUpManager::default();
        pop_up_manager.new_pop_up(
            PopUpType::Question(Rc::new(|_bool| -> Message { Message::ResetPopUp })),
            "title".to_string(),
            "text".to_string(),
        );
        assert!(!pop_up_manager.major_pop_up);
        assert!(pop_up_manager.minor_pop_up);
        assert!(pop_up_manager.question_pop_up.is_some());
        assert_eq!(pop_up_manager.title, "title".to_string());
        assert_eq!(pop_up_manager.text, "text".to_string());
    }

    #[test]
    fn reset_pop_up() {
        let mut pop_up_manager = PopUpManager::default();
        pop_up_manager.new_pop_up(PopUpType::Minor, "title".to_string(), "text".to_string());
        pop_up_manager.reset();
        assert!(!pop_up_manager.major_pop_up);
        assert!(!pop_up_manager.minor_pop_up);
        assert!(pop_up_manager.question_pop_up.is_none());
        assert!(pop_up_manager.title.is_empty());
        assert!(pop_up_manager.text.is_empty());
    }
}
