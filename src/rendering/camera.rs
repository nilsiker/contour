use bevy::{prelude::*, render::camera::ScalingMode};

use crate::pawn::player::Player;

fn follow_camera_system(
    player: Query<&Transform, (Without<Camera2d>, With<Player>)>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    for player_transform in &player {
        for mut transform in &mut camera {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
    }
}

pub struct FollowCameraPlugin;
impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::Last, follow_camera_system);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle({
        let mut camera = Camera2dBundle::default();
        camera.projection.scaling_mode = ScalingMode::WindowSize;
        camera.projection.scale = 0.15;
        camera.projection.far = f32::MAX;
        camera
    });
}
