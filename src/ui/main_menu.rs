use bevy::{ecs::system::EntityCommands, prelude::*};

use super::{
    releasable_action_buttons::{Action, ActionButton, LastInteractionTracker},
    UiData,
};

#[derive(Component)]
pub struct MainMenu;

pub fn setup_main_menu(ui: &mut EntityCommands, ui_data: &UiData) {
    // HUD
    ui.with_children(|hud| {
        // MAIN MENU
        hud.spawn_bundle(NodeBundle {
            color: Color::rgba(0.5, 0.5, 0.5, 0.5).into(),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect::new(Val::Px(150.0), Val::Auto, Val::Percent(25.0), Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(20.0), Val::Percent(50.0)),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("MainMenu"))
        .insert(MainMenu)
        .with_children(|main_menu| {
            // TITLE
            main_menu.spawn_bundle(
                TextBundle::from_section(
                    "CONTOUR",
                    TextStyle {
                        color: Color::WHITE.into(),
                        font: ui_data.font.clone(),
                        font_size: 30.0,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );

            // Start
            main_menu
                .spawn_bundle(ButtonBundle {
                    style: ui_data.button_style.clone(),
                    color: Color::BLACK.into(),
                    ..default()
                })
                .insert(Name::new("StartButton"))
                .insert(LastInteractionTracker(Interaction::None))
                .insert(ActionButton(Action::GameStart))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            color: Color::rgb(0.9, 0.9, 0.9),
                            font_size: 20.0,
                            font: ui_data.font.clone(),
                        },
                    ));
                });

            // LOAD
            main_menu
                .spawn_bundle(ButtonBundle {
                    style: ui_data.button_style.clone(),
                    color: Color::BLACK.into(),
                    ..default()
                })
                .insert(Name::new("Load"))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Load",
                        TextStyle {
                            color: Color::rgb(0.9, 0.9, 0.9),
                            font_size: 20.0,
                            font: ui_data.font.clone(),
                        },
                    ));
                });

            // OPTIONS
            main_menu
                .spawn_bundle(ButtonBundle {
                    style: ui_data.button_style.clone(),
                    color: Color::BLACK.into(),
                    ..default()
                })
                .insert(Name::new("Options"))
                .insert(LastInteractionTracker(Interaction::None))
                .insert(ActionButton(Action::OptionsOpen))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Options",
                        TextStyle {
                            color: Color::rgb(0.9, 0.9, 0.9),
                            font_size: 20.0,
                            font: ui_data.font.clone(),
                        },
                    ));
                });

            // QUIT
            main_menu
                .spawn_bundle(ButtonBundle {
                    style: ui_data.button_style.clone(),
                    color: Color::BLACK.into(),
                    ..default()
                })
                .insert(Name::new("Quit"))
                .insert(LastInteractionTracker(Interaction::None))
                .insert(ActionButton(Action::GameQuit))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            color: Color::rgb(0.9, 0.9, 0.9),
                            font_size: 20.0,
                            font: ui_data.font.clone(),
                        },
                    ));
                });
        });
    });
}
