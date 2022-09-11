use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_egui::{
    egui::{
        self, style::Margin, Color32, FontId, Frame, Layout, Pos2, Rect, RichText, Rounding, Sense,
        Stroke, Style, Vec2, Visuals,
    },
    EguiContext,
};
use strum_macros::Display;

use crate::audio::AudioChannels;

use super::{
    releasable_action_buttons::{Action, ActionButton, LastInteractionTracker},
    UiData,
};

#[derive(Component)]
pub struct OptionsMenu;

#[derive(Component, Display)]
pub enum OptionValue {
    BgmVolume,
    SfxVolume,
}

#[derive(Default)]
pub struct OptionsUiState {
    open: bool,
    resolution: (i32, i32),
    vsync: bool,
    bgm: f64,
    sfx: f64,
}

#[derive(Clone)]
pub struct OptionStyles {
    pub centered: Style,
    pub options_button: Style,
    pub options_button_text: TextStyle,
    pub bit_font: TextStyle,
}

pub struct OptionsMenuPlugin;
impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OptionsUiState {
            open: true,
            sfx: 50.,
            bgm: 50.,
            resolution: (1920, 1080),
            ..default()
        })
        .add_system(render);
    }
}

pub fn render(
    mut egui: ResMut<EguiContext>,
    mut state: ResMut<OptionsUiState>,
    windows: Res<Windows>,
) {
    // let styles = OptionStyles {
    //     centered: Style {
    //         align_content: AlignContent::Center,
    //         align_items: AlignItems::Center,
    //         align_self: AlignSelf::Center,
    //         ..Default::default()
    //     },
    //     options_button: Style {
    //         size: Size {
    //             width: Val::Auto,
    //             height: Val::Px(25.0),
    //         },
    //         align_content: AlignContent::Center,
    //         align_items: AlignItems::Center,
    //         align_self: AlignSelf::Center,
    //         ..Default::default()
    //     },
    //     options_button_text: TextStyle {
    //         font: ui_data.font.clone(),
    //         font_size: 20.0,
    //         color: Color::WHITE.into(),
    //     },
    //     bit_font: TextStyle {
    //         font: ui_data.font.clone(),
    //         font_size: 20.0,
    //         ..default()
    //     },
    // };

    let window = windows.get_primary().unwrap();
    let center_pos: Pos2 = (window.width() / 2.0, window.height() / 2.0).into();
    let size: Vec2 = (window.width() / 3.0, window.height() / 2.0).into();
    if state.open {
        egui::Window::new("OptionsMenu")
            .title_bar(false)
            .frame(Frame {
                inner_margin: Margin::same(40.0),
                fill: Color32::DARK_GRAY,
                rounding: Rounding::same(8.0),
                stroke: Stroke::new(5.0, Color32::GRAY),
                ..default()
            })
            .fixed_rect(Rect::from_center_size(center_pos, size))
            .show(egui.clone().ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("OPTIONS").color(Color32::WHITE).size(40.0));

                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(RichText::new("VIDEO").color(Color32::WHITE).size(20.0));

                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Resolution").color(Color32::WHITE));
                                let x = if state.resolution.0.to_string().len() < 4 {
                                    format!(" {}", state.resolution.0)
                                } else {
                                    state.resolution.0.to_string()
                                };
                                let y = if state.resolution.1.to_string().len() < 4 {
                                    format!("{} ", state.resolution.1)
                                } else {
                                    state.resolution.1.to_string()
                                };
                                egui::ComboBox::from_label("")
                                    .selected_text(format!("{x}x{y}"))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut state.resolution,
                                            (960, 540),
                                            "960x540",
                                        );
                                        ui.selectable_value(
                                            &mut state.resolution,
                                            (1280, 720),
                                            "1280x720",
                                        );
                                        ui.selectable_value(
                                            &mut state.resolution,
                                            (1920, 1080),
                                            "1920x1080",
                                        );
                                    });
                            });

                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Vsync").color(Color32::WHITE));
                                ui.add(egui::Checkbox::new(&mut state.vsync, ""));
                            });
                        });

                        ui.vertical(|ui| {
                            ui.label(RichText::new("AUDIO").color(Color32::WHITE).size(20.0));

                            ui.horizontal(|ui| {
                                ui.label(RichText::new("BGM").color(Color32::WHITE));
                                ui.add(
                                    egui::Slider::new(&mut state.sfx, 0.0..=100.0)
                                        .integer()
                                        .step_by(10.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("SFX").color(Color32::WHITE));
                                ui.add(
                                    egui::Slider::new(&mut state.bgm, 0.0..=100.0)
                                        .integer()
                                        .step_by(10.0),
                                );
                            });
                        });
                    });

                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        // Close button
                        if ui
                            .add_sized(
                                [100.0, 50.0],
                                egui::Button::new(RichText::new("Close").color(Color32::WHITE))
                                    .fill(Color32::DARK_RED),
                            )
                            .clicked()
                        {
                            state.open = false;
                        }

                        // Apply button
                        if ui
                            .add_sized(
                                [100.0, 50.0],
                                egui::Button::new(RichText::new("Apply").color(Color32::WHITE))
                                    .fill(Color32::DARK_BLUE),
                            )
                            .clicked()
                        {
                            state.open = false;
                            bevy::log::warn!("TODO apply values to resources");
                        }
                    })
                })
            });
    }

    // ui.with_children(|ui| {
    //     ui.spawn_bundle(NodeBundle {
    //         color: Color::rgba(0.5, 0.5, 0.5, 0.5).into(),
    //         style: Style {
    //             display: Display::None,
    //             position_type: PositionType::Absolute,
    //             flex_direction: FlexDirection::ColumnReverse,
    //             align_self: AlignSelf::Center,
    //             align_items: AlignItems::Center,
    //             justify_content: JustifyContent::Center,
    //             size: Size::new(Val::Percent(20.0), Val::Auto),
    //             padding: UiRect::all(Val::Px(10.0)),
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(Name::new("OptionsMenu"))
    //     .insert(OptionsMenu)
    //     .with_children(|options_menu| {
    //         options_menu.spawn_bundle(
    //             TextBundle::from_section(
    //                 "VOLUME",
    //                 TextStyle {
    //                     color: Color::WHITE.into(),
    //                     font: ui_data.font.clone(),
    //                     font_size: 30.0,
    //                 },
    //             )
    //             .with_style(Style {
    //                 margin: UiRect::all(Val::Px(20.0)),
    //                 ..default()
    //             }),
    //         );

    //         spawn_float_control(
    //             options_menu,
    //             String::from("BGM"),
    //             OptionValue::BgmVolume,
    //             Action::LowerVolumeBGM,
    //             Action::RaiseVolumeBGM,
    //             styles.clone(),
    //         );

    //         spawn_float_control(
    //             options_menu,
    //             String::from("SFX"),
    //             OptionValue::SfxVolume,
    //             Action::LowerVolumeSFX,
    //             Action::RaiseVolumeSFX,
    //             styles,
    //         );

    //         options_menu
    //             .spawn_bundle(ButtonBundle {
    //                 style: ui_data.button_style.clone(),
    //                 color: Color::BLACK.into(),
    //                 ..default()
    //             })
    //             .insert(Name::new("Close"))
    //             .insert(LastInteractionTracker(Interaction::None))
    //             .insert(ActionButton(Action::OptionsClose))
    //             .with_children(|parent| {
    //                 parent.spawn_bundle(TextBundle::from_section(
    //                     "Close",
    //                     TextStyle {
    //                         color: Color::rgb(0.9, 0.9, 0.9),
    //                         font_size: 20.0,
    //                         font: ui_data.font.clone(),
    //                     },
    //                 ));
    //             });
    //     });
    // });
}

pub fn update_sound_value_text(
    mut query: Query<(&mut Text, &OptionValue)>,
    audio_channels: Res<AudioChannels>,
) {
    if audio_channels.is_changed() {
        for (mut text, option_label) in &mut query {
            let volume = match option_label {
                OptionValue::BgmVolume => audio_channels.bgm.0,
                OptionValue::SfxVolume => audio_channels.sfx.0,
            };
            text.sections[0].value = format!("{}", volume)
        }
    }
}
