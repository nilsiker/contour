#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod ai;
pub mod animation;
pub mod assets;
mod audio;
mod config;
mod dialogue;
mod game;
mod ldtk;
mod pawn;
pub mod physics;
pub mod rendering;
mod save;
pub mod state;
pub mod ui;
mod interaction;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use bevy_egui::EguiPlugin;
use heron::prelude::*;

#[cfg(feature = "debug")] // Debug plugins
use {bevy::diagnostic::FrameTimeDiagnosticsPlugin, bevy_inspector_egui::WorldInspectorPlugin};

use game::ContourPlugins;

fn main() {
    let mut app = App::new();
    // Base plugins
    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Contour".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            width: 1280.0,
            height: 720.0,
            transparent: true,
            decorations: false,
            position: WindowPosition::At(Vec2 { x: 1720.0, y: 0.0 }),
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(
            0x16 as f32 / 255.0,
            0x16 as f32 / 255.0,
            0x1D as f32 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(EguiPlugin);

    // Contour game plugins
    app.add_plugins(ContourPlugins);

    #[cfg(feature = "debug")] // Debug plugins
    app.add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(WorldInspectorPlugin::default());

    app.run();
}
