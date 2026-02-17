use crate::client::gui::app::App;
use crate::client::gui::bb_theme;
use crate::client::gui::bb_theme::color::BACKGROUND_COLOR;
use crate::client::gui::bb_theme::container::ContainerStyle;
use crate::client::gui::bb_theme::custom_button::{
    BUTTON_RADIUS_LEFT_ZERO, BUTTON_RADIUS_RIGHT_ZERO, ButtonStyle, create_text_button,
};
use crate::client::gui::bb_theme::text_format;
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::bb_widget::graph::{GraphWidget, view_graph_widget_settings};
use crate::client::gui::bb_widget::progress::ProgressWidget;
use crate::client::gui::bb_widget::stats::exercise_stat_column;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::user_interface::Message;
use crate::client::gui::user_interface::Message::ChangeShownChartType;
use iced::Element;
use iced::widget::{Column, Row, Space, combo_box, container};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::{Length, Padding};
use std::cmp::PartialEq;

pub const CHART_WIDGET_WIDTH: f32 = 700.0;
pub const CHART_WIDGET_HEIGHT: f32 = 500.0;

#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub enum ChartTypes {
    #[default]
    Graph,
    Progress,
}

impl ChartTypes {
    pub fn get_graph_design_name(&self) -> &str {
        match self {
            ChartTypes::Progress => "Bar",
            ChartTypes::Graph => "Line",
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
        Message::SelectExercise,
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

    let chart: Element<'a, Message> = match app.widget_manager.graph_widget_state.shown_chart_type {
        ChartTypes::Progress => {
            let progress: Element<Message> =
                ProgressWidget::new(app.mascot_manager.selected_mascot, &app.exercise_manager)
                    .into();
            let column = Column::new()
                .push(Space::with_height(INDENT))
                .push(progress);

            column.into()
        }
        ChartTypes::Graph => {
            let column = Column::new()
                .push(view_graph_widget_settings(app))
                .push(GraphWidget::new(app).view());

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
        .push(Space::with_width(Length::FillPortion(1)))
        .push(title)
        .push(Space::with_width(Length::FillPortion(2)))
        .push(chart_type_buttons(app))
        .push(Space::with_width(Length::FillPortion(2)))
        .push(search_bar)
        .push(Space::with_width(Length::FillPortion(1)))
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

fn chart_type_buttons(app: &App) -> Row<Message> {
    let (line_button_style, bar_button_style) =
        match app.widget_manager.graph_widget_state.shown_chart_type {
            ChartTypes::Graph => (ButtonStyle::Active, ButtonStyle::InactiveTab),
            ChartTypes::Progress => (ButtonStyle::InactiveTab, ButtonStyle::Active),
        };

    let line_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        ChartTypes::Graph.get_graph_design_name().to_string(),
        line_button_style,
        Some(BUTTON_RADIUS_RIGHT_ZERO),
    )
    .on_press(ChangeShownChartType(ChartTypes::Graph));

    let bar_button = create_text_button(
        &app.mascot_manager.selected_mascot,
        ChartTypes::Progress.get_graph_design_name().to_string(),
        bar_button_style,
        Some(BUTTON_RADIUS_LEFT_ZERO),
    )
    .on_press(ChangeShownChartType(ChartTypes::Progress));

    Row::new().push(line_button).push(bar_button)
}
