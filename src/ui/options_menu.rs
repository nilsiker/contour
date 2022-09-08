use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_inspector_egui::options;

use crate::audio::Volume;

use super::{
    releasable_action_buttons::{Action, ActionButton, LastInteractionTracker},
    UiData,
};

#[derive(Component)]
pub struct OptionsMenu;

#[derive(Component)]
pub struct VolumeLabel;

pub fn setup_options_menu(ui: &mut EntityCommands, ui_data: &UiData) {
    let option_button_style = Style {
        size: Size::new(Val::Auto, Val::Px(25.0)),
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        align_self: AlignSelf::Center,
        ..default()
    };

    let option_button_text_style = TextStyle {
        font: ui_data.font.clone(),
        font_size: 20.0,
        color: Color::WHITE.into(),
    };

    let centered_style = Style {
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        align_self: AlignSelf::Center,
        ..Default::default()
    };

    ui.with_children(|ui| {
        ui.spawn_bundle(NodeBundle {
            color: Color::rgba(0.5, 0.5, 0.5, 0.5).into(),
            style: Style {
                display: Display::None,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::ColumnReverse,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(20.0), Val::Auto),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("OptionsMenu"))
        .insert(OptionsMenu)
        .with_children(|options_menu| {
            options_menu.spawn_bundle(
                TextBundle::from_section(
                    "OPTIONS",
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

            options_menu
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: centered_style.clone(),
                    ..default()
                })
                .with_children(|sound_node| {
                    sound_node
                        .spawn_bundle(
                            TextBundle::from_section(
                                "Sound:",
                                TextStyle {
                                    font: ui_data.font.clone(),
                                    font_size: 20.0,
                                    ..default()
                                },
                            )
                            .with_style(centered_style.clone()),
                        )
                        .insert(Name::new("SoundLabel"));

                    sound_node
                        .spawn_bundle(ButtonBundle {
                            style: option_button_style.clone(),
                            color: Color::DARK_GRAY.into(),

                            ..default()
                        })
                        .insert(Name::new("<"))
                        .insert(LastInteractionTracker(Interaction::None))
                        .insert(ActionButton(Action::LowerVolume))
                        .with_children(|decrement_button| {
                            decrement_button
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "< ",
                                        option_button_text_style.clone(),
                                    )
                                    .with_style(centered_style.clone()),
                                )
                                .insert(Name::new("Label"));
                        });

                    sound_node
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(25.0)),
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .insert(Name::new("Value"))
                        .with_children(|value_node| {
                            value_node
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        format!(" {:02} ", 0),
                                        TextStyle {
                                            font: ui_data.font.clone(),
                                            font_size: 20.0,
                                            ..default()
                                        },
                                    )
                                    .with_style(centered_style.clone()),
                                )
                                .insert(Name::new("Text"))
                                .insert(VolumeLabel);
                        });

                    sound_node
                        .spawn_bundle(ButtonBundle {
                            style: option_button_style.clone(),
                            ..default()
                        })
                        .insert(Name::new(">"))
                        .insert(LastInteractionTracker(Interaction::None))
                        .insert(ActionButton(Action::RaiseVolume))
                        .with_children(|increment_button| {
                            increment_button
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        " >",
                                        option_button_text_style.clone(),
                                    )
                                    .with_style(centered_style.clone()),
                                )
                                .insert(Name::new("Label"));
                        });
                });

            options_menu
                .spawn_bundle(ButtonBundle {
                    style: ui_data.button_style.clone(),
                    color: Color::BLACK.into(),
                    ..default()
                })
                .insert(Name::new("Close"))
                .insert(LastInteractionTracker(Interaction::None))
                .insert(ActionButton(Action::OptionsClose))
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Close",
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

pub fn update_sound_value_text(
    mut query: Query<&mut Text, With<VolumeLabel>>,
    volume: Res<Volume>,
) {
    if volume.is_changed() {
        for mut text in &mut query {
            bevy::log::info!("audio should change to {}", volume.0);
            text.sections[0].value = format!(" {:02} ", volume.0)
        }
    }
}
