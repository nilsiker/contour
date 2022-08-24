pub mod follow_camera {
    use bevy::{prelude::*, render::camera::ScalingMode};
    
    use crate::character::player::PlayerPosition;
    
    fn follow_camera_system(
        player: Query<&PlayerPosition>,
        mut camera: Query<&mut Transform, With<Camera2d>>,
    ) {

        for player_position in &player {
            for mut transform in &mut camera {
                transform.translation.x = player_position.x;
                transform.translation.y = player_position.y;
            }
        }
    }

    pub struct FollowCameraPlugin;
    impl Plugin for FollowCameraPlugin {
        fn build(&self, app: &mut App) {
            app.add_startup_system(setup)
                .add_system_to_stage(CoreStage::PostUpdate, follow_camera_system);
        }
    }

    fn setup(mut commands: Commands) {
        commands.spawn_bundle({
            let mut camera = Camera2dBundle::default();
            camera.projection.scaling_mode = ScalingMode::WindowSize;
            camera.projection.scale = 0.1;
            camera.projection.far = f32::MAX;
            camera
        });
    }
}
