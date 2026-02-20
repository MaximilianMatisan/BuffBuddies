use crate::client::gui::bb_widget::progress_bar::ProgressBarState;

pub struct ProgressBarStateManager {
    pub water_progress_bar_state: ProgressBarState,
    pub steps_progress_bar_state: ProgressBarState,
    pub sleep_progress_bar_state: ProgressBarState,
}

impl ProgressBarStateManager {
    pub(crate) fn new() -> Self {
        Self {
            water_progress_bar_state: ProgressBarState::new(1.0, 3.0),
            steps_progress_bar_state: ProgressBarState::new(2000.0, 10000.0),
            sleep_progress_bar_state: ProgressBarState::new(7.0, 8.0),
        }
    }

    pub(crate) fn copy_states(progress_bar_state: &ProgressBarStateManager) -> Self {
        Self {
            water_progress_bar_state: ProgressBarState::new(
                progress_bar_state.water_progress_bar_state.current_value,
                progress_bar_state.water_progress_bar_state.goal_value,
            ),
            steps_progress_bar_state: ProgressBarState::new(
                progress_bar_state.steps_progress_bar_state.current_value,
                progress_bar_state.steps_progress_bar_state.goal_value,
            ),
            sleep_progress_bar_state: ProgressBarState::new(
                progress_bar_state.sleep_progress_bar_state.current_value,
                progress_bar_state.sleep_progress_bar_state.goal_value,
            ),
        }
    }
}
