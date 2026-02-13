use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::gui::bb_widget::activity_widget::activity::{
    ActivityWidget, calculate_activity_data,
};
use crate::client::gui::bb_widget::bmi_calculator::BMIWidgetState;
use crate::client::gui::bb_widget::circle_widget::CircleWidgetState;
use crate::client::gui::bb_widget::graph::GraphWidgetState;
use crate::common::mascot_mod::mascot::Mascot;

pub struct WidgetManager {
    pub activity_widget: ActivityWidget,
    pub graph_widget_state: GraphWidgetState,
    pub circle_widget_state: CircleWidgetState,
    pub bmi_widget_state: BMIWidgetState,
}

impl WidgetManager {
    pub(crate) fn new() -> Self {
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
        }
    }
}
