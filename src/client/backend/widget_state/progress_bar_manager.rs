use crate::client::gui::bb_widget::progress_bar::ProgressBarState;
use crate::common::user_mod::user::UserInformation;

pub struct ProgressBarStateManager {
    pub water_progress_bar_state: ProgressBarState,
    pub steps_progress_bar_state: ProgressBarState,
    pub sleep_progress_bar_state: ProgressBarState,
}

impl ProgressBarStateManager {
    pub(crate) fn new(user_information: &UserInformation) -> Self {
        Self {
            water_progress_bar_state: ProgressBarState::new(0.0, user_information.user_goals.water),
            steps_progress_bar_state: ProgressBarState::new(0.0, user_information.user_goals.steps),
            sleep_progress_bar_state: ProgressBarState::new(0.0, user_information.user_goals.sleep),
        }
    }

    pub(crate) fn duplicate_states(&self) -> Self {
        Self {
            water_progress_bar_state: self.water_progress_bar_state.duplicate(),
            steps_progress_bar_state: self.steps_progress_bar_state.duplicate(),
            sleep_progress_bar_state: self.sleep_progress_bar_state.duplicate(),
        }
    }

    pub(crate) fn update_goals(&mut self, user_information: &UserInformation) {
        self.water_progress_bar_state.goal_value = user_information.user_goals.water;
        self.steps_progress_bar_state.goal_value = user_information.user_goals.steps;
        self.sleep_progress_bar_state.goal_value = user_information.user_goals.sleep;
    }
}
