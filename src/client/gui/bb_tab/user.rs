use crate::client::backend::mascot_mod::mascot_trait::MascotTrait;
use crate::client::backend::user_mod::user::{UserInformation};
use crate::client::gui::app::App;
use crate::client::gui::bb_theme::color::{HIGHLIGHTED_CONTAINER_COLOR, TEXT_COLOR};
use crate::client::gui::bb_theme::container::{ContainerStyle, create_style_container};
use crate::client::gui::bb_theme::text_format::{
    FIRA_SANS_EXTRABOLD, format_description_text, kg_to_string,
};
use crate::client::gui::bb_widget::stats::profile_stat_container;
use crate::client::gui::bb_widget::widget_utils::{INDENT, LARGE_INDENT};
use crate::client::gui::size::LARGE_PROFILE_PICTURE_DIMENSION;
use crate::client::gui::user_interface::Message;
use iced::Element;
use iced::widget::{Column, Container, Row, Space, column, container, image, row, text};
use iced_core::Length;
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::image::Handle;
use crate::client::backend::mascot_mod::mascot::Mascot;

pub fn view_profile<'a>(app: &'a App, user: &UserInformation, owned_mascots: &Vec<Mascot>, favorite_mascot: &Mascot) -> Element<'a, Message> {
    let profile_picture = container(
        image(user.profile_picture_handle.clone())
            .width(Length::Shrink)
            .height(Length::Fill),
    )
    .padding([0, 40]);

    let username: Element<Message> = text(user.username.clone())
        .font(FIRA_SANS_EXTRABOLD)
        .color(TEXT_COLOR)
        .size(40)
        .into();

    let description_container: Container<Message> =
        container(format_description_text(text(user.description.clone())))
            .style(create_style_container(
                ContainerStyle::Background,
                None,
                Some(HIGHLIGHTED_CONTAINER_COLOR),
            ))
            .width(Length::FillPortion(10))
            .height(Length::Fill)
            .padding(5);

    let description_element: Row<Message> = row![
        format_description_text(text("Description: ")),
        Space::with_width(Length::FillPortion(1)),
        description_container
    ];

    let username_and_description: Column<Message> = column![username, description_element]
        .height(Length::Fill)
        .spacing(INDENT);

    let header: Container<Message> =
        container(row![profile_picture, username_and_description].align_y(Vertical::Center))
            .width(Length::Fill)
            .height(LARGE_PROFILE_PICTURE_DIMENSION);

    let activity_widget = app.activity_widget.view(app);

    let streak_stat = profile_stat_container(
        Handle::from_path("assets/images/stats/flame.png"),
        user.weekly_workout_streak.to_string(),
        "week".to_string(),
        "streak".to_string(),
    );
    let total_mascots_stat = profile_stat_container(
        Handle::from_path("assets/images/stats/golden_dog.png"),
        owned_mascots.len().to_string(),
        "mascots".to_string(),
        "owned".to_string(),
    );
    let total_sets_stat = profile_stat_container(
        Handle::from_path("assets/images/stats/golden_stats.png"),
        user.profile_stat_manager.total_sets.to_string(),
        "sets".to_string(),
        "done".to_string(),
    );
    let best_pr_stat = profile_stat_container(
        Handle::from_path("assets/images/stats/golden_stats.png"),
        kg_to_string(user.profile_stat_manager.best_pr.1),
        user.profile_stat_manager.best_pr.0.clone(),
        "best pr".to_string(),
    );
    let total_reps_stat = profile_stat_container(
        Handle::from_path("assets/images/stats/golden_biceps.png"),
        user.profile_stat_manager.total_reps.to_string(),
        "reps".to_string(),
        "done".to_string(),
    );
    let total_lifted_stat = profile_stat_container(
        Handle::from_path("assets/images/stats/golden_dumbbell.png"),
        user.profile_stat_manager.total_lifted_weight.to_string(),
        "kilograms".to_string(),
        "lifted".to_string(),
    );

    let stat_row_one = Row::new()
        .push(streak_stat)
        .push(total_sets_stat)
        .push(total_reps_stat)
        .spacing(INDENT);

    let stat_row_two = Row::new()
        .push(total_mascots_stat)
        .push(best_pr_stat)
        .push(total_lifted_stat)
        .spacing(INDENT);

    let stats = Column::new()
        .push(
            text("Stats")
                .font(FIRA_SANS_EXTRABOLD)
                .color(TEXT_COLOR)
                .size(25),
        )
        .push(stat_row_one)
        .push(stat_row_two)
        .spacing(INDENT);

    let favorite_mascot = Column::new()
        .push(
            text("Favorite Mascot")
                .font(FIRA_SANS_EXTRABOLD)
                .color(TEXT_COLOR)
                .size(25),
        )
        .push(image(favorite_mascot.get_file_path()))
        .spacing(INDENT)
        .align_x(Horizontal::Center);

    let stat_mascot_row = Row::new()
        .push(stats)
        .push(favorite_mascot)
        .spacing(LARGE_INDENT);

    let content = column![header, activity_widget, stat_mascot_row]
        .align_x(Horizontal::Center)
        .spacing(LARGE_INDENT)
        .padding(INDENT);

    content.into()
}
