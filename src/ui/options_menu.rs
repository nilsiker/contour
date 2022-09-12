use bevy::prelude::*;
use bevy_egui::{
    egui::{self, style::Margin, Color32, Frame, Pos2, Rect, RichText, Rounding, Vec2},
    EguiContext,
};

use super::styling::{MENU_BUTTON_FILL, MENU_FILL, MENU_STROKE};

#[derive(Default)]
pub struct OptionsUiState {
    pub open: bool,
}

pub struct OptionsMenuPlugin;
impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OptionsUiState {
            open: true,
            ..default()
        })
        .insert_resource(VideoSettings::default())
        .insert_resource(AudioConfig::default())
        .add_system(update);
    }
}

// TODO Move these structs to a settings mod
pub struct VideoSettings {
    pub vsync: bool,
    pub fullscreen: bool,
    pub resolution: (f32, f32),
}
impl VideoSettings {
    fn default() -> Self {
        VideoSettings {
            vsync: false,
            fullscreen: false,
            resolution: (1920.0, 1080.),
        }
    }
}
impl FromWorld for VideoSettings {
    fn from_world(_: &mut World) -> Self {
        VideoSettings {
            vsync: false,
            fullscreen: false,
            resolution: (1920.0, 1080.),
        }
    }
}
pub struct AudioConfig {
    pub sfx: f64,
    pub bgm: f64,
}

impl AudioConfig {
    fn default() -> Self {
        AudioConfig {
            sfx: 50.0,
            bgm: 50.0,
        }
    }
}
impl FromWorld for AudioConfig {
    fn from_world(_: &mut World) -> Self {
        AudioConfig {
            sfx: 50.0,
            bgm: 50.0,
        }
    }
}

fn update(
    egui: Res<EguiContext>,
    mut state: ResMut<OptionsUiState>,
    mut video_to_apply: Local<VideoSettings>,
    mut video: ResMut<VideoSettings>,
    mut audio: ResMut<AudioConfig>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let center_pos: Pos2 = (window.width() / 2.0, window.height() / 2.0).into();
    let size: Vec2 = (window.width() / 3.0, window.height() / 2.0).into();
    if state.open {
        egui::Window::new("OptionsMenu")
            .title_bar(false)
            .frame(Frame {
                inner_margin: Margin::same(40.0),
                fill: MENU_FILL,
                rounding: Rounding::same(8.0),
                stroke: MENU_STROKE,
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
                                ui.label(RichText::new("Fullscreen").color(Color32::WHITE));
                                ui.add(egui::Checkbox::new(&mut video_to_apply.fullscreen, ""));
                            });
                            if !video.fullscreen {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("Resolution").color(Color32::WHITE));
                                    let x = if video_to_apply.resolution.0.to_string().len() < 4 {
                                        format!(" {}", video_to_apply.resolution.0)
                                    } else {
                                        video_to_apply.resolution.0.to_string()
                                    };
                                    let y = if video_to_apply.resolution.1.to_string().len() < 4 {
                                        format!("{} ", video_to_apply.resolution.1)
                                    } else {
                                        video_to_apply.resolution.1.to_string()
                                    };
                                    egui::ComboBox::from_label("")
                                        .selected_text(format!("{x}x{y}"))
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut video_to_apply.resolution,
                                                (960.0, 540.0),
                                                "960x540",
                                            );
                                            ui.selectable_value(
                                                &mut video_to_apply.resolution,
                                                (1280.0, 720.0),
                                                "1280x720",
                                            );
                                            ui.selectable_value(
                                                &mut video_to_apply.resolution,
                                                (1920.0, 1080.0),
                                                "1920x1080",
                                            );
                                        });
                                });
                            }
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("Vsync").color(Color32::WHITE));
                                ui.add(egui::Checkbox::new(&mut video_to_apply.vsync, ""));
                            });
                        });

                        ui.vertical(|ui| {
                            ui.label(RichText::new("AUDIO").color(Color32::WHITE).size(20.0));

                            ui.horizontal(|ui| {
                                ui.label(RichText::new("BGM").color(Color32::WHITE));
                                ui.add(
                                    egui::Slider::new(&mut audio.bgm, 0.0..=100.0)
                                        .integer()
                                        .step_by(10.0),
                                );
                            });
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("SFX").color(Color32::WHITE));
                                ui.add(
                                    egui::Slider::new(&mut audio.sfx, 0.0..=100.0)
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
                                egui::Button::new(RichText::new("Back").color(Color32::WHITE))
                                    .fill(Color32::TRANSPARENT),
                            )
                            .clicked()
                        {
                            video_to_apply.fullscreen = video.fullscreen;
                            video_to_apply.vsync = video.vsync;
                            video_to_apply.resolution = video.resolution;

                            state.open = false;
                        }

                        // Apply button
                        if ui
                            .add_sized(
                                [100.0, 50.0],
                                egui::Button::new(RichText::new("Apply").color(Color32::WHITE))
                                    .fill(MENU_BUTTON_FILL),
                            )
                            .clicked()
                        {
                            video.fullscreen = video_to_apply.fullscreen;
                            video.vsync = video_to_apply.vsync;
                            video.resolution = video_to_apply.resolution;
                        }
                    })
                })
            });
    }
}
