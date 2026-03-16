use crate::client::backend::widget_state::widget_state_manager::WidgetMessage;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::color::BACKGROUND_COLOR;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::{
    BUTTON_RADIUS_BOTH_ZERO, BUTTON_RADIUS_LEFT_ZERO, BUTTON_RADIUS_RIGHT_ZERO, ButtonStyle,
    create_text_button,
};
use crate::client::gui::bb_theme::text_format;
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::bb_widget::chart_widget::bar_chart::BarChart;
use crate::client::gui::bb_widget::chart_widget::graph::{GraphMessage, GraphWidget};
use crate::client::gui::bb_widget::stats::exercise_stat_column;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::user_interface::Message;
use crate::common::user_mod::user_goals::GoalType;
use ChartMessage::Graph;
use GraphMessage::DecrementCounter;
use Message::Widget;
use WidgetMessage::Chart;
use iced::widget::{Column, Row, Space, combo_box, container, row, text};
use iced::{Element, Task};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Length, Padding};
use std::cmp::PartialEq;

pub const CHART_WIDGET_WIDTH: f32 = 700.0;
pub const CHART_WIDGET_HEIGHT: f32 = 500.0;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum DataPointsType {
    Exercise(ChartTypes),
    Health(ChartTypes, GoalType),
}

impl Default for DataPointsType {
    fn default() -> Self {
        DataPointsType::Exercise(Default::default())
    }
}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ChartTypes {
    #[default]
    Graph,
    Bar,
}

impl ChartTypes {
    pub fn get_graph_design_name(&self) -> &str {
        match self {
            ChartTypes::Bar => "Bar",
            ChartTypes::Graph => "Line",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChartMessage {
    SelectExercise(String),
    Graph(DataPointsType, GraphMessage),
    ChangeShownChartType(DataPointsType),
    ChangeShownGoalType(ChartTypes, GoalType),
}

impl ChartMessage {
    pub fn update(self, app: &mut App) -> Task<Message> {
        match self {
            ChartMessage::SelectExercise(exercise) => {
                app.exercise_manager.update_selected_exercise(exercise);
                Task::none()
            }

            ChartMessage::Graph(data_points_type, graph_message) => {
                GraphMessage::update_graph(data_points_type, graph_message, app)
            }

            ChartMessage::ChangeShownChartType(data_points_type) => {
                match data_points_type {
                    DataPointsType::Exercise(chart_type) => {
                        app.widget_manager
                            .exercise_graph_widget_state
                            .data_points_type = DataPointsType::Exercise(chart_type);
                    }
                    DataPointsType::Health(chart_type, goal_type) => {
                        app.widget_manager
                            .health_graph_widget_state
                            .data_points_type = DataPointsType::Health(chart_type, goal_type)
                    }
                }

                Task::none()
            }

            ChartMessage::ChangeShownGoalType(chart_type, goal_type) => {
                app.widget_manager
                    .health_graph_widget_state
                    .data_points_type = DataPointsType::Health(chart_type, goal_type);
                Task::none()
            }
        }
    }
}

pub fn chart_environment_widget<'a>(app: &'a App) -> Element<'a, Message> {
    let title: Element<'a, Message> =
        format_button_text(iced::widget::text("Progress").size(40)).into();
    let search_bar: Element<Message> = combo_box(
        &app.exercise_manager.tracked_exercise_state,
        "Search Exercise...",
        Some(&app.exercise_manager.selected_exercise_name),
        |name| Message::Widget(WidgetMessage::Chart(ChartMessage::SelectExercise(name))),
    )
    .menu_style(bb_theme::combo_box::create_menu_style(
        &app.mascot_manager.selected_mascot,
    ))
    .input_style(bb_theme::combo_box::create_text_input_style(
        &app.mascot_manager.selected_mascot,
        BACKGROUND_COLOR,
    ))
    .font(text_format::FIRA_SANS_EXTRABOLD)
    .width(Length::Fixed(250.0))
    .padding([8, 16])
    .into();

    let DataPointsType::Exercise(chart_type) = &app
        .widget_manager
        .exercise_graph_widget_state
        .data_points_type
    else {
        panic!("Wrong chart type!")
    };
    let chart: Element<'a, Message> = match chart_type {
        ChartTypes::Bar => {
            let bar_chart: Element<Message> = BarChart::new(
                app.mascot_manager.selected_mascot,
                &app.exercise_manager.data_points,
            )
            .into();
            let column = Column::new()
                .push(Space::new().height(INDENT))
                .push(bar_chart);

            column.into()
        }
        ChartTypes::Graph => {
            let column = Column::new()
                .push(view_graph_widget_settings(
                    DataPointsType::Exercise(ChartTypes::Graph),
                    app,
                ))
                .push(
                    GraphWidget::new(
                        &app.widget_manager.exercise_graph_widget_state,
                        &app.exercise_manager.data_points,
                        app.mascot_manager.selected_mascot,
                    )
                    .view(),
                );

            column.into()
        }
    };

    let exercise_stats = exercise_stat_column(app)
        .width(Length::Fixed(CHART_WIDGET_WIDTH))
        .padding(Padding {
            top: 0.0,
            right: LARGE_INDENT,
            bottom: LARGE_INDENT,
            left: LARGE_INDENT,
        });

    let header_row = Row::new()
        .width(Length::Fixed(CHART_WIDGET_WIDTH))
        .push(Space::new().width(Length::FillPortion(1)))
        .push(title)
        .push(Space::new().width(Length::FillPortion(2)))
        .push(chart_type_buttons(
            app,
            DataPointsType::Exercise(*chart_type),
        ))
        .push(Space::new().width(Length::FillPortion(2)))
        .push(search_bar)
        .push(Space::new().width(Length::FillPortion(1)))
        .align_y(Vertical::Center);

    let contents = Column::new()
        .width(Length::Shrink)
        .push(header_row)
        .push(chart)
        .push(exercise_stats)
        .padding(Padding {
            top: LARGE_INDENT / 2.0,
            ..Default::default()
        })
        .align_x(Horizontal::Center);

    container(contents)
        .width(Length::Shrink)
        .style(bb_theme::container::create_container_style(
            ContainerStyle::Default,
            None,
            None,
        ))
        .into()
}

fn chart_type_buttons(app: &App, data_points_type: DataPointsType) -> Row<'_, Message> {
    let current_chart = match &data_points_type {
        DataPointsType::Exercise(chart) => chart,
        DataPointsType::Health(chart, _) => chart,
    };

    let (line_button_style, bar_button_style) = match current_chart {
        ChartTypes::Graph => (ButtonStyle::Active, ButtonStyle::InactiveTab),
        ChartTypes::Bar => (ButtonStyle::InactiveTab, ButtonStyle::Active),
    };

    let make_message = |chart_type| {
        Message::Widget(WidgetMessage::Chart(ChartMessage::ChangeShownChartType(
            match &data_points_type {
                DataPointsType::Exercise(_) => DataPointsType::Exercise(chart_type),
                DataPointsType::Health(_, goal_type) => {
                    DataPointsType::Health(chart_type, *goal_type)
                }
            },
        )))
    };

    let line_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        ChartTypes::Graph.get_graph_design_name().to_string(),
        line_button_style,
        Some(BUTTON_RADIUS_RIGHT_ZERO),
    )
    .on_press(make_message(ChartTypes::Graph));

    let bar_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        ChartTypes::Bar.get_graph_design_name().to_string(),
        bar_button_style,
        Some(BUTTON_RADIUS_LEFT_ZERO),
    )
    .on_press(make_message(ChartTypes::Bar));

    Row::new().push(line_button).push(bar_button)
}

fn goal_type_buttons(app: &App) -> Row<'_, Message> {
    let DataPointsType::Health(chart_type, goal_type) = &app
        .widget_manager
        .health_graph_widget_state
        .data_points_type
    else {
        panic!("Wrong chart type!")
    };
    let (sleep_button_style, water_button_style, weight_button_style) = match goal_type {
        GoalType::Sleep => (
            ButtonStyle::Active,
            ButtonStyle::InactiveTab,
            ButtonStyle::InactiveTab,
        ),
        GoalType::Water => (
            ButtonStyle::InactiveTab,
            ButtonStyle::Active,
            ButtonStyle::InactiveTab,
        ),
        GoalType::Weight => (
            ButtonStyle::InactiveTab,
            ButtonStyle::InactiveTab,
            ButtonStyle::Active,
        ),
        _ => panic!("There is no graph option for the other goal types"),
    };

    let sleep_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        GoalType::Sleep.to_string(),
        sleep_button_style,
        Some(BUTTON_RADIUS_RIGHT_ZERO),
    )
    .on_press(Widget(Chart(ChartMessage::ChangeShownGoalType(
        *chart_type,
        GoalType::Sleep,
    ))));

    let water_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        GoalType::Water.to_string(),
        water_button_style,
        Some(BUTTON_RADIUS_BOTH_ZERO),
    )
    .on_press(Widget(Chart(ChartMessage::ChangeShownGoalType(
        *chart_type,
        GoalType::Water,
    ))));

    let weight_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        GoalType::Weight.to_string(),
        weight_button_style,
        Some(BUTTON_RADIUS_LEFT_ZERO),
    )
    .on_press(Widget(Chart(ChartMessage::ChangeShownGoalType(
        *chart_type,
        GoalType::Weight,
    ))));

    Row::new()
        .push(sleep_button)
        .push(water_button)
        .push(weight_button)
}

pub fn view_graph_widget_settings<'a>(
    data_points_type: DataPointsType,
    app: &App,
) -> Element<'a, Message> {
    let graph_state = match data_points_type {
        DataPointsType::Exercise(_) => &app.widget_manager.exercise_graph_widget_state,
        DataPointsType::Health(_, _) => &app.widget_manager.health_graph_widget_state,
    };

    let counter = format_button_text(text!("{}", graph_state.points_to_draw)).size(19);

    let increment_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        "+".to_string(),
        ButtonStyle::Active,
        Some(BUTTON_RADIUS_LEFT_ZERO),
    )
    .on_press(Widget(Chart(Graph(
        data_points_type,
        GraphMessage::IncrementCounter,
    ))));

    let decrement_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        "-".to_string(),
        ButtonStyle::Active,
        Some(BUTTON_RADIUS_RIGHT_ZERO),
    )
    .on_press(Widget(Chart(Graph(data_points_type, DecrementCounter))));

    let row_counter_with_buttons = row![
        decrement_button,
        Space::new().width(Length::FillPortion(1)),
        counter,
        Space::new().width(Length::FillPortion(1)),
        increment_button,
    ]
    .align_y(Vertical::Center);

    let counter_with_buttons = container(row_counter_with_buttons)
        .style(create_container_style(
            ContainerStyle::Light,
            Some(10.into()),
            None,
        ))
        .width(Length::Fixed(100.0));

    let button_style_dots_button = match graph_state.visible_points {
        true => ButtonStyle::Active,
        _ => ButtonStyle::InactiveTab,
    };

    let toggle_dots_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        "Dots".to_string(),
        button_style_dots_button,
        Some(10.0.into()),
    )
    .on_press(Widget(Chart(Graph(
        data_points_type,
        GraphMessage::ToggleDots,
    ))));

    let button_style_cursor_button = match graph_state.visible_cursor_information {
        true => ButtonStyle::Active,
        _ => ButtonStyle::InactiveTab,
    };

    let toggle_cursor_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        "Cursor".to_string(),
        button_style_cursor_button,
        Some(10.0.into()),
    )
    .on_press(Widget(Chart(Graph(
        data_points_type,
        GraphMessage::ToggleCursor,
    ))));

    let button_style_vertical_lines_button = match graph_state.visible_vertical_lines {
        true => ButtonStyle::Active,
        _ => ButtonStyle::InactiveTab,
    };

    let toggle_vertical_lines = create_text_button(
        &app.mascot_manager.selected_mascot,
        "Vertical lines".to_string(),
        button_style_vertical_lines_button,
        Some(10.0.into()),
    )
    .on_press(Widget(Chart(Graph(
        data_points_type,
        GraphMessage::ToggleVerticalLines,
    ))));

    let settings_row = Row::new()
        .width(Length::Fixed(CHART_WIDGET_WIDTH))
        .push(Space::new().width(Length::Fixed(LARGE_INDENT)))
        .push(counter_with_buttons)
        .push(Space::new().width(Length::FillPortion(1)))
        .push(toggle_dots_button)
        .push(Space::new().width(Length::FillPortion(1)))
        .push(toggle_cursor_button)
        .push(Space::new().width(Length::FillPortion(1)))
        .push(toggle_vertical_lines)
        .push(Space::new().width(Length::FillPortion(15)))
        .align_y(Vertical::Bottom);

    let settings_row_with_padding = Column::new()
        .push(Space::new().height(21.5))
        .push(settings_row);

    settings_row_with_padding.into()
}

pub fn health_chart_environment_widget<'a>(app: &'a App) -> Element<'a, Message> {
    let DataPointsType::Health(chart_type, goal_type) = &app
        .widget_manager
        .health_graph_widget_state
        .data_points_type
    else {
        panic!("Wrong chart type!")
    };
    let title_content: String = goal_type.to_string();

    let chart: Element<'a, Message> = match chart_type {
        ChartTypes::Bar => {
            let bar_chart: Element<Message> = BarChart::new(
                app.mascot_manager.selected_mascot,
                &app.user_manager.user_info.user_logs.weight_log,
            )
            .into();
            let column = Column::new()
                .push(Space::new().height(INDENT))
                .push(bar_chart);

            column.into()
        }
        ChartTypes::Graph => {
            let column = Column::new()
                .push(view_graph_widget_settings(
                    DataPointsType::Health(ChartTypes::Graph, GoalType::Weight),
                    app,
                ))
                .push(
                    GraphWidget::new(
                        &app.widget_manager.health_graph_widget_state,
                        &app.user_manager.user_info.user_logs.weight_log,
                        app.mascot_manager.selected_mascot,
                    )
                    .view(),
                );

            column.into()
        }
    };

    let title: Element<'a, Message> =
        format_button_text(iced::widget::text(title_content).size(40)).into();

    let header_row = Row::new()
        .width(Length::Fixed(CHART_WIDGET_WIDTH))
        .push(Space::new().width(Length::Fixed(LARGE_INDENT)))
        .push(title)
        .push(Space::new().width(Length::FillPortion(1)))
        .push(chart_type_buttons(
            app,
            DataPointsType::Health(*chart_type, *goal_type),
        ))
        .push(Space::new().width(Length::FillPortion(5)))
        .push(goal_type_buttons(app))
        .push(Space::new().width(Length::FillPortion(2)))
        .align_y(Vertical::Center);

    let contents = Column::new()
        .width(Length::Shrink)
        .push(header_row)
        .push(chart)
        .padding(Padding {
            top: LARGE_INDENT / 2.0,
            ..Default::default()
        })
        .align_x(Horizontal::Center);

    container(contents)
        .width(Length::Shrink)
        .style(bb_theme::container::create_container_style(
            ContainerStyle::Default,
            None,
            None,
        ))
        .into()
}
