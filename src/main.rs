#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod ai;
pub mod animation;
pub mod assets;
mod audio;
mod config;
mod dialogue;
mod game;
mod paths;
mod pawn;
mod rendering;
pub mod state;
pub mod ui;
mod level;

use audio::AudioPlugin;
#[cfg(debug_assertions)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;
#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;
#[cfg(debug_assertions)]
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use game::ContourPlugins;

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
        // Add Contour game plugins
        .add_plugins(ContourPlugins);

    #[cfg(debug_assertions)]
    app.add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default());

    app.run();
}
