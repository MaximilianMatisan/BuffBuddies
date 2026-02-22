use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::backend::profile_stat_manager::calculate_activity_data;
use crate::client::backend::widget_state::progress_bar_manager::ProgressBarStateManager;
use crate::client::gui::bb_widget::activity_widget::activity::ActivityWidget;
use crate::client::gui::bb_widget::bmi_calculator::BMIWidgetState;
use crate::client::gui::bb_widget::circle_widget::CircleWidgetState;
use crate::client::gui::bb_widget::graph::GraphWidgetState;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::user_mod::user::UserInformation;

pub struct WidgetManager {
    pub activity_widget: ActivityWidget,
    pub graph_widget_state: GraphWidgetState,
    pub circle_widget_state: CircleWidgetState,
    pub bmi_widget_state: BMIWidgetState,
    pub progress_bar_state_manager: ProgressBarStateManager,
    pub pending_progress_bar_state_manager: Option<ProgressBarStateManager>,
}

impl WidgetManager {
    pub(crate) fn new(user_information: &UserInformation) -> Self {
        let exercise_manager = ExerciseManager::default();
        let default_mascot = Mascot::default();

        WidgetManager {
            activity_widget: ActivityWidget::new(
                default_mascot,
                calculate_activity_data(&exercise_manager.exercises),
            ),
            graph_widget_state: GraphWidgetState::new(),
            circle_widget_state: CircleWidgetState::new(),
            bmi_widget_state: BMIWidgetState::new(),
            progress_bar_state_manager: ProgressBarStateManager::new(user_information),
            pending_progress_bar_state_manager: None,
        }
    }
}
