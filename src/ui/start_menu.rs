use bevy::prelude::*;

use crate::consts::path;

use super::releasable_action_buttons::{Action, ActionButton, LastInteractionTracker};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiData::default());
        app.add_startup_system(setup);
    }
}

struct UiData {
    font: Handle<Font>,
    button_style: Style,
}
impl Default for UiData {
    fn default() -> Self {
        Self {
            font: Default::default(),
            button_style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(90.0), Val::Px(80.0)),
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        }
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut ui_data: ResMut<UiData>) {
    ui_data.font = assets.load(path::FONT_BIT);

    // HUD
    commands
        .spawn_bundle(NodeBundle {
            color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("UI"))
        .with_children(|hud| {
            // MAIN MENU
            hud.spawn_bundle(NodeBundle {
                color: Color::rgba(0.5, 0.5, 0.5, 0.5).into(),
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    align_self: AlignSelf::Center,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(20.0), Val::Percent(50.0)),
                    margin: UiRect::new(
                        Val::Percent(10.0),
                        Val::Auto,
                        Val::Px(20.0),
                        Val::Percent(10.0),
                    ),
                    ..default()
                },
                ..default()
            })
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
                    .insert(ActionButton(Action::QuitApp))
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
