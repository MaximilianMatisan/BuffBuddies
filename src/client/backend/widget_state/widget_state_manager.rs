use crate::client::backend::exercise_manager::ExerciseManager;
use crate::client::backend::profile_stat_manager::calculate_activity_data;
use crate::client::backend::widget_state::progress_bar_manager::ProgressBarStateManager;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::animated_background::{
    BackgroundAnimationMessage, BackgroundAnimationState,
};
use crate::client::gui::bb_widget::activity_widget::activity::{ActivityMessage, ActivityWidget};
use crate::client::gui::bb_widget::bmi_calculator::{BMIMessage, BMIWidgetState};
use crate::client::gui::bb_widget::chart_widget::chart::{ChartMessage, DataPointsType};
use crate::client::gui::bb_widget::chart_widget::graph::GraphWidgetState;
use crate::client::gui::bb_widget::circle_widget::{CircleMessage, CircleWidgetState};
use crate::client::gui::bb_widget::progress_bar::ProgressBarMessage;
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::user_mod::user::UserInformation;
use iced::Task;

pub struct WidgetManager {
    pub activity_widget: ActivityWidget,
    pub exercise_graph_widget_state: GraphWidgetState,
    pub health_graph_widget_state: GraphWidgetState,
    pub circle_widget_state: CircleWidgetState,
    pub bmi_widget_state: BMIWidgetState,
    pub progress_bar_state_manager: ProgressBarStateManager,
    pub pending_progress_bar_state_manager: Option<ProgressBarStateManager>,
    pub background_animation_state: BackgroundAnimationState,
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
            exercise_graph_widget_state: GraphWidgetState::new(DataPointsType::Exercise(
                Default::default(),
            )),
            health_graph_widget_state: GraphWidgetState::new(DataPointsType::Health(
                Default::default(),
                Default::default(),
            )),
            circle_widget_state: CircleWidgetState::new(),
            bmi_widget_state: BMIWidgetState::new(),
            progress_bar_state_manager: ProgressBarStateManager::new(user_information),
            pending_progress_bar_state_manager: None,
            background_animation_state: BackgroundAnimationState::default(),
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

#[derive(Debug, Clone)]
pub enum WidgetMessage {
    Chart(ChartMessage),
    Activity(ActivityMessage),
    Circle(CircleMessage),
    Bmi(BMIMessage),
    ProgressBar(ProgressBarMessage),
    BackgroundAnimation(BackgroundAnimationMessage),
    ToggleGeneralExerciseInfo(u32),
}

impl WidgetMessage {
    pub fn update(widget_message: WidgetMessage, app: &mut App) -> Task<Message> {
        match widget_message {
            WidgetMessage::Chart(chart_message) => ChartMessage::update(chart_message, app),

            WidgetMessage::Activity(activity_message) => {
                app.widget_manager.activity_widget.update(activity_message)
            }

            WidgetMessage::Circle(circle_message) => match circle_message {
                CircleMessage::UpdateCircleAnimation(event) => {
                    app.widget_manager
                        .circle_widget_state
                        .animation_progress
                        .update(event);
                    app.widget_manager.circle_widget_state.update_circle();
                    Task::none()
                }
            },

            WidgetMessage::Bmi(bmi_message) => BMIMessage::update_bmi_message(bmi_message, app),
            WidgetMessage::ProgressBar(progress_bar_message) => {
                ProgressBarMessage::update_progress_bar_message(progress_bar_message, app)
            }
            WidgetMessage::BackgroundAnimation(background_message) => match background_message {
                BackgroundAnimationMessage::UpdateAnimation(event) => {
                    app.widget_manager
                        .background_animation_state
                        .animation_progress
                        .update(event);
                    app.widget_manager.background_animation_state.cache.clear();
                    Task::none()
                }
            },
            WidgetMessage::ToggleGeneralExerciseInfo(id) => {
                let extended_set = &mut app.exercise_manager.extended_general_exercise_infos;
                if extended_set.contains(&id) {
                    extended_set.remove(&id);
                } else {
                    extended_set.insert(id);
                }
                Task::none()
            }
        }
    }
}
