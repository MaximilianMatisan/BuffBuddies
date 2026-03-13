use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::backend::profile_stat_manager::calculate_activity_data;
use crate::client::backend::widget_state::progress_bar_manager::ProgressBarStateManager;
use crate::client::gui::app::App;
use crate::client::gui::bb_widget::activity_widget::activity::ActivityWidget;
use crate::client::gui::bb_widget::bmi_calculator::BMIWidgetState;
use crate::client::gui::bb_widget::chart_widget::chart::DataPointsType;
use crate::client::gui::bb_widget::chart_widget::graph::GraphWidgetState;
use crate::client::gui::bb_widget::circle_widget::CircleWidgetState;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::user_mod::user::UserInformation;

pub struct WidgetManager {
    pub activity_widget: ActivityWidget,
    pub exercise_graph_widget_state: GraphWidgetState,
    pub health_graph_widget_state: GraphWidgetState,
    pub circle_widget_state: CircleWidgetState,
    pub bmi_widget_state: BMIWidgetState,
    pub progress_bar_state_manager: ProgressBarStateManager,
    pub pending_progress_bar_state_manager: Option<ProgressBarStateManager>,
}

impl WidgetManager {
    pub(crate) fn new(user_information: &UserInformation) -> Self {
        let exercise_manager = ExerciseManager::default();
        let default_mascot = Mascot::default();
        let activity_widget = ActivityWidget::new(
            default_mascot,
            calculate_activity_data(&exercise_manager.exercises),
        );

        WidgetManager {
            activity_widget,
            exercise_graph_widget_state: GraphWidgetState::new(DataPointsType::Exercise(Default::default())),
            health_graph_widget_state: GraphWidgetState::new(DataPointsType::Health(Default::default(), Default::default())),
            circle_widget_state: CircleWidgetState::new(),
            bmi_widget_state: BMIWidgetState::new(),
            progress_bar_state_manager: ProgressBarStateManager::new(user_information),
            pending_progress_bar_state_manager: None,
        }
    }
}
pub fn update_progress_bar_goals_after_updated_user_info(app: &mut App) {
    // Update the normal progress bar states
    app.widget_manager
        .progress_bar_state_manager
        .update_goals(&app.user_manager.user_info);

    // Update pending state if existent
    if let Some(pending_state) = &mut app.widget_manager.pending_progress_bar_state_manager {
        pending_state.update_goals(&app.user_manager.user_info)
    }
}
