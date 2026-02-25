use std::time::Duration;
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::{BACKGROUND_COLOR, CONTAINER_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::text_format::{FIRA_SANS_EXTRABOLD, format_description_text};
use crate::client::gui::bb_widget::general_exercise_info_elements::display_general_exercise_infos;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::bb_widget::{circle_widget, new_widget, workout};
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{Column, Row, Scrollable, column, container, row, text, image, Space, Stack};
use iced_anim::Motion;
use iced_core::{Alignment, Border, Image, Length, Padding, Theme};
use iced_core::alignment::Vertical;
use iced_core::border::Radius;
use iced_core::image::Handle;
use crate::client::gui::bb_tab::tab::Tab;
use crate::client::gui::bb_theme::color;
use crate::client::gui::bb_theme::custom_button::create_button_style;
use crate::client::gui::bb_theme::scrollable::{main_style, mascot_style};
use crate::client::gui::bb_widget::workout::WorkoutWidget;
use crate::client::gui::size::FRAME_WIDTH;
use crate::common::mascot_mod::mascot_trait::MascotTrait;

impl App {
    pub fn workout_screen(&self) -> Element<Message> {

        let selected_mascot_image: iced::widget::Image<Handle> = self.mascot_manager.view_active_mascot().height(200).width(200);
        let recent_workouts_text = container(text("Recent workouts").font(FIRA_SANS_EXTRABOLD).color(color::TEXT_COLOR).size(32))
            .height(100)
            .align_y(Alignment::End);

        let background_mascot_with_text =
            container(
                Row::new()
                    .push(selected_mascot_image)
                    .push(Space::with_width(Length::FillPortion(1)))
                    .push(recent_workouts_text)
                    .push(Space::with_width(Length::FillPortion(10))))
                .height(285).width(FRAME_WIDTH);


        let recent_workouts_row = Row::new()
            .push(new_widget::new_workout_widget_button())
            .push(WorkoutWidget::default_recent_workout_widget())
            .push(WorkoutWidget:: default_recent_workout_widget())
            .push(WorkoutWidget:: default_recent_workout_widget())
            .spacing(10);

        let aligned_workout_preset_row =
            container(
                recent_workouts_row)
        .height(285)
            .align_y(Alignment::End);

        let recent_workouts_with_mascot =
        Stack::new()
            .push(background_mascot_with_text)
            .push(aligned_workout_preset_row);


        let mut workout_preset_row = Row::new()
            .spacing(10);

        for preset in &self.workout_preset_manager.presets {
            workout_preset_row =
                workout_preset_row.push(workout::WorkoutWidget::new_workout_preset_widget(preset));
        }

        let workout_preset_scrollable =
                 Scrollable::new(workout_preset_row).direction(Direction::Horizontal(Scrollbar::new().scroller_width(6))).style(|theme: &_, status: iced::widget::scrollable::Status| {
                     mascot_style(status,self.mascot_manager.selected_mascot)
                 });

        let workout_preset_scrollable_with_button =
            Row::new()
                .push(new_widget::new_preset_widget_button())
                .push(workout_preset_scrollable)
                .spacing(10);


        let test_border =  Border {
            color: iced::color!(146,142,142),
            width: 0.0,
            radius:Radius {
                top_left: 10.0,
                top_right: 10.0,
                bottom_right: 10.0,
                bottom_left: 10.0,
            }
        };

        let workout_presets_button =  iced_anim::widget::button(text("Workout presets >").font(FIRA_SANS_EXTRABOLD).color(color::TEXT_COLOR).size(30))
            .on_press(Message::Select(Tab::Settings)) //TODO: Screen with presets
            .animation(Motion{
                damping: Motion::SMOOTH.damping ,
                response: Duration::from_millis(350),
            })
            .style( move |_: &Theme, status|{
              create_button_style(status, test_border, BACKGROUND_COLOR, CONTAINER_COLOR, CONTAINER_COLOR)
            });


        let general_exercise_info_elements = display_general_exercise_infos(
            &self.mascot_manager.selected_mascot,
            &self.exercise_manager,
        );
        let browse_exercises_title = text("Browse exercises")
            .size(30)
            .font(FIRA_SANS_EXTRABOLD)
            .color(TEXT_COLOR);

        let title_bar = Row::new()
            .push(browse_exercises_title)
            .push(format_description_text(text(format!(
                " - {} results",
                self.exercise_manager.exercises.len()
            ))))
            .align_y(Vertical::Center);

        let exercise_browser = Column::new()
            .push(title_bar)
            .push(general_exercise_info_elements)
            .spacing(20.0);

        let exercise_info_container = container(exercise_browser)
            .style(create_container_style(ContainerStyle::Default, None, None))
            .padding(LARGE_INDENT);


        let workout_interface =
        Column::new()
            .push(recent_workouts_with_mascot)
            .push(workout_presets_button)
            .push(workout_preset_scrollable_with_button)
            .push(exercise_info_container)
            .spacing(30)
            .padding(Padding {
                top: 20.0,
                left: 20.0,
                ..0.0.into()
            });;

        let scrollable_workout_interface =
        Scrollable::new(workout_interface)
            .style(|theme: &Theme, status: iced::widget::scrollable::Status| main_style(status, self.mascot_manager.selected_mascot))
            .direction(Direction::Vertical(Scrollbar::new().scroller_width(7.0).margin(4)));

        scrollable_workout_interface.into()

    }
}
