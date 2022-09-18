use bevy::prelude::*;
use bevy_egui::{
    egui::{self, epaint::Shadow, style::Margin, Color32, Frame, Pos2, Rect, Rounding, Vec2},
    EguiContext,
};

use crate::{
    audio::bgm::BgmVolumeChangedEvent,
    config::{AudioSettings, ConfigUpdateEvent, VideoSettings},
};

use super::{
    styling::{MENU_BUTTON_FILL, MENU_FILL},
    text::*,
};

#[derive(Default)]
pub struct OptionsUiState {
    pub show: bool,
}

pub struct OptionsMenuPlugin;
impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OptionsUiState { show: false })
            .add_system(update);
    }
}

// TODO Move these structs to a settings mod

fn update(
    egui: Res<EguiContext>,
    mut state: ResMut<OptionsUiState>,
    mut video_to_apply: Local<VideoSettings>,
    mut audio_if_revert: Local<AudioSettings>,
    mut video: ResMut<VideoSettings>,
    mut audio: ResMut<AudioSettings>,
    mut events: EventWriter<ConfigUpdateEvent>,
    mut bgm_events: EventWriter<BgmVolumeChangedEvent>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let center_pos: Pos2 = (window.width() / 2.0, window.height() / 2.0).into();
    let size: Vec2 = (window.width() / 3.0, window.height() / 2.0).into();
    if state.show {
        egui::Window::new("OptionsMenu")
            .title_bar(false)
            .frame(Frame {
                inner_margin: Margin::same(40.0),
                fill: MENU_FILL,
                rounding: Rounding::same(8.0),
                shadow: Shadow::big_dark(),
                ..default()
            })
            .fixed_rect(Rect::from_center_size(center_pos, size))
            .show(egui.clone().ctx_mut(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(h1("OPTIONS"));

                    ui.add_space(20.0);
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label(h2("VIDEO"));

                            ui.horizontal(|ui| {
                                ui.label(p("Fullscreen"));
                                ui.add(egui::Checkbox::new(&mut video_to_apply.fullscreen, ""));
                            });

                            ui.horizontal(|ui| {
                                ui.label(p("Resolution"));
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

                            ui.horizontal(|ui| {
                                ui.label(p("Vsync"));
                                ui.add(egui::Checkbox::new(&mut video_to_apply.vsync, ""));
                            });
                        });

                        ui.vertical(|ui| {
                            ui.label(h2("AUDIO"));

                            ui.horizontal(|ui| {
                                ui.label(p("BGM"));
                                let bgm_slider = ui.add(
                                    egui::Slider::new(&mut audio.bgm, 0.0..=100.0)
                                        .integer()
                                        .step_by(10.0),
                                );
                                if bgm_slider.changed() {
                                    bgm_events.send(BgmVolumeChangedEvent);
                                }
                            });
                            ui.horizontal(|ui| {
                                ui.label(p("SFX"));
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
                                egui::Button::new(p("Back")).fill(Color32::TRANSPARENT),
                            )
                            .clicked()
                        {
                            video_to_apply.fullscreen = video.fullscreen;
                            video_to_apply.vsync = video.vsync;
                            video_to_apply.resolution = video.resolution;

                            audio.bgm = audio_if_revert.bgm;
                            audio.sfx = audio_if_revert.sfx;

                            state.show = false;
                        }

                        // Apply button
                        if ui
                            .add_sized(
                                [100.0, 50.0],
                                egui::Button::new(em("Apply")).fill(MENU_BUTTON_FILL),
                            )
                            .clicked()
                        {
                            video.fullscreen = video_to_apply.fullscreen;
                            video.vsync = video_to_apply.vsync;
                            video.resolution = video_to_apply.resolution;
                            audio_if_revert.bgm = audio.bgm;
                            audio_if_revert.sfx = audio.sfx;

                            state.show = false;
                            events.send(ConfigUpdateEvent);
                        }
                    })
                })
            });
    }
}
