use crate::client::gui::app::App;
use crate::client::gui::bb_tab::preset_overview::preset_overview_screen;
use crate::client::gui::bb_tab::user::view_profile;
use crate::client::gui::bb_theme::container::{ContainerStyle, create_container_style};
use crate::client::gui::bb_theme::custom_button::{
    ButtonStyle, TAB_BUTTON_HEIGHT, TAB_BUTTON_WIDTH, create_element_button, create_text_button,
};
use crate::client::gui::bb_theme::text_format::format_button_text;
use crate::client::gui::bb_widget::social_elements::profile_tab_button;
use crate::client::gui::bb_widget::widget_utils::INDENT;
use crate::client::gui::user_interface::Message;
use crate::common::mascot_mod::mascot::Mascot;
use crate::common::user_mod::user::{UserInformation, UserType};
use iced::Element;
use iced::widget::{Column, Row, Space, container, row};
use iced_core::alignment::{Horizontal, Vertical};
use iced_core::image::Handle;
use iced_core::{Length, Padding, Theme};
use strum_macros::Display;

pub const FRAME_PADDING: f32 = 15.0;

#[derive(Default, Debug, Clone, PartialEq, Display)]
pub enum Tab {
    //TAB-BUTTONS
    #[default]
    Home,
    Workout,
    Health,
    Social,
    Mascot,
    Settings,
    Exit,
    //Further frames
    ViewProfile,
    Loading,
    CreateWorkout,
    CreatePreset,
    PresetOverview,
}

impl Tab {
    /// Shown tab categories on the left hand side of the logged-in view
    pub fn get_tab_button_categories() -> [Tab; 7] {
        [
            Tab::Home,
            Tab::Workout,
            Tab::Health,
            Tab::Social,
            Tab::Mascot,
            Tab::Settings,
            Tab::Exit,
        ]
    }
}

pub fn view_tab_button_bar<'a>(
    mascot: &'a Mascot,
    screen: &Tab,
    user_info: &'a UserInformation,
) -> impl Into<Element<'a, Message>> {
    let lower_tab_container_button = Row::new()
        .push(Space::new().width(Length::Fill))
        .push(view_money_tab_button(mascot, user_info.coin_balance))
        .width(310);

    let all_tab_buttons = Column::new()
        .push(view_tab_buttons(mascot, screen, user_info))
        .push(Space::new().height(Length::Fill))
        .push(lower_tab_container_button)
        .padding(INDENT);

    let mut tab_container = container(all_tab_buttons)
        .style(create_container_style(ContainerStyle::Default, None, None))
        .height(Length::Fill)
        .width(310);

    tab_container = container(tab_container).padding(Padding {
        right: 0.0,
        ..FRAME_PADDING.into()
    });

    tab_container
}
fn view_tab_buttons<'a>(
    mascot: &Mascot,
    screen: &Tab,
    user_info: &'a UserInformation,
) -> impl Into<Element<'a, Message>> {
    let mut tab_buttons: Column<Message> =
        Column::new().spacing(INDENT).align_x(Horizontal::Center);
    tab_buttons = tab_buttons.push(profile_tab_button(user_info, mascot));
    for tab in Tab::get_tab_button_categories() {
        tab_buttons = tab_buttons.push(
            create_text_button(
                mascot,
                tab.to_string(),
                if *screen == tab {
                    ButtonStyle::ActiveTab
                } else {
                    ButtonStyle::InactiveTab
                },
                None,
            )
            .width(TAB_BUTTON_WIDTH)
            .height(TAB_BUTTON_HEIGHT)
            .on_press(Message::Select(tab)),
        );
    }
    tab_buttons
}

fn view_money_tab_button(mascot: &Mascot, coin_balance: u32) -> impl Into<Element<'_, Message>> {
    let money_button: iced_anim::widget::Button<'_, Message, Theme, iced::Renderer> =
        create_element_button(
            mascot,
            row![
                iced::widget::image(Handle::from_path("assets/images/coin.png"))
                    .width(25)
                    .height(25),
                Space::new().width(Length::Fill),
                format_button_text(iced::widget::text(coin_balance))
            ]
            .align_y(Vertical::Center)
            .into(),
            ButtonStyle::InactiveTab,
            None,
        )
        .on_press(Message::Select(Tab::Mascot))
        .width(Length::Fill)
        .height(Length::Shrink);
    money_button
}

pub fn view_tab_content(app: &App) -> Option<Element<'_, Message>> {
    let tab_window: Option<Element<Message>> = match app.screen {
        Tab::Loading => Some(app.view_loading_screen()), // Loading should be handled earlier, as it covers the whole screen
        Tab::Home => Some(app.homescreen()),
        Tab::Workout => Some(app.workout_screen()),
        Tab::Health => Some(app.health_screen()),
        Tab::Social => Some(app.social_screen()),
        Tab::Mascot => Some(app.mascot_screen()),
        Tab::Settings => Some(app.settings_screen()),
        Tab::Exit => None,
        Tab::CreateWorkout => Some(app.workout_creation_screen()),
        Tab::CreatePreset => Some(app.preset_creation_screen()),
        Tab::PresetOverview => Some(preset_overview_screen(
            &app.mascot_manager.selected_mascot,
            &app.workout_preset_manager.presets,
        )),
        Tab::ViewProfile => {
            let user_type = &app.user_manager.most_recently_viewed_user;

            match user_type {
                UserType::Own => Some(view_profile(
                    app,
                    &app.user_manager.user_info,
                    &app.mascot_manager.owned_mascots,
                    false,
                )),
                UserType::Other(username) => {
                    let viewed_profile = app.user_manager.get_user_by_username(username);

                    viewed_profile.map(|profile| {
                        view_profile(
                            app,
                            &profile.user_information,
                            &profile.owned_mascots,
                            profile.friends_with_active_user,
                        )
                    })
                }
            }
        }
    };
    tab_window
}
