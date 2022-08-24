mod animation;
mod camera;
mod character;
mod lighting;
mod rendering;
mod text;

use character::{enemy::EnemyPlugin, *};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
    sprite::Anchor,
    window::PresentMode,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::{
    prelude::{Collider, NoUserData, RapierPhysicsPlugin, RigidBody},
    render::RapierDebugRenderPlugin,
};
use camera::follow_camera::FollowCameraPlugin;
use player::PlayerCharacterPlugin;
use rendering::OrderedZ;
use text::TextPlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(ImageSettings::default_nearest()) // prevents blurry sprites
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Bevy Gamejam 2022".to_string(),
            position: WindowPosition::At(Vec2::new(1720., 0.)),
            height: 1440.,
            width: 1720.,
            present_mode: PresentMode::AutoNoVsync,
            transparent: true,
            resizable: true,
            cursor_visible: true,

            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Debug
        .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TextPlugin)
        // Physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        // Custom
        .add_startup_system(setup_debug_level)
        .add_plugin(lighting::LightingPlugin)
        .add_plugin(PlayerCharacterPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(FollowCameraPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_system(rendering::order_z_entities);

    app.run();
}

fn setup_debug_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                flip_x: false,
                flip_y: false,
                custom_size: Some(Vec2::new(8., 16.)),
                anchor: Anchor::Center,
            },
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(20., 20., 0.)))
        .insert(Name::new("Block"))
        .insert(RigidBody::Fixed)
        .insert(OrderedZ)
        .with_children(|children| {
            children
                .spawn()
                .insert(Collider::cuboid(4.0, 4.0))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -4.0, 0.0)));
        });
}
