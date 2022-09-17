#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod ai;
pub mod assets;
mod audio;
mod config;
mod consts;
mod dialogue;
mod game;
mod pawn;
mod rendering;
pub mod ui;
pub mod state;

use audio::ContourAudioPlugins;
#[cfg(debug_assertions)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;
#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;
#[cfg(debug_assertions)]
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use iyes_loopless::prelude::*;

use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use config::ConfigPlugin;
use dialogue::DialoguePlugin;
use game::{GameState, WorldState};
use pawn::PawnPlugin;
use rendering::ContourRenderingPlugins;
use ui::ContourUiPlugins;

fn main() {
    let mut app = App::new();
    // Setup stuff

    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Contour".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            width: 1920.0,
            height: 1080.0,
            transparent: true,
            position: WindowPosition::At(Vec2 { x: 1400.0, y: 0.0 }),
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(EguiPlugin)
        // Game specific
        .add_state(WorldState::Unpaused)
        .add_state(GameState::Prelude)
        .add_plugin(ConfigPlugin)
        .add_plugins(ContourRenderingPlugins)
        .add_plugin(PawnPlugin)
        .add_plugins(ContourAudioPlugins)
        .add_plugins(ContourUiPlugins)
        .add_plugin(DialoguePlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default());

    app.run();
}