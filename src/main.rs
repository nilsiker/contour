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

use audio::AudioPlugin;
#[cfg(debug_assertions)]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::RegisterInspectable;
#[cfg(debug_assertions)]
use bevy_inspector_egui::WorldInspectorPlugin;
use heron::prelude::*;

use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use game::ContourPlugins;
use ldtk::entities::SpriteOffset;
use physics::Drag;
use rendering::YSort;

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
        .insert_resource(ClearColor(Color::rgb(
            0x16 as f32 / 255.0,
            0x16 as f32 / 255.0,
            0x1D as f32 / 255.0,
        )))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(EguiPlugin)
        // Add Contour game plugins
        .add_plugins(ContourPlugins);

    #[cfg(debug_assertions)]
    app.add_plugin(FrameTimeDiagnosticsPlugin)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .register_inspectable::<YSort>()
        .register_inspectable::<Drag>()
        .register_inspectable::<SpriteOffset>();

    app.run();
}
