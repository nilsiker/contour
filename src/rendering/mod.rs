mod camera;
mod lighting;

use crate::config::{ConfigUpdateEvent, VideoSettings};
use bevy::{
    prelude::*,
    transform::TransformSystem,
    window::{PresentMode, WindowMode},
};
use bevy_inspector_egui::Inspectable;

use self::{camera::FollowCameraPlugin, lighting::LightingPlugin};

pub const PAWN_LAYER: f32 = 3.0;

pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        let window = app
            .world
            .get_resource::<Windows>()
            .unwrap()
            .get_primary()
            .unwrap();
        app.insert_resource(ResolutionSetting((window.width(), window.height())))
            .insert_resource(VSyncSetting(false))
            .add_startup_system(initiate_window_settings)
            .add_system(update_window_settings)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                order_z_entities.after(TransformSystem::TransformPropagate),
            )
            .add_plugin(FollowCameraPlugin)
            .add_plugin(LightingPlugin);
    }
}

#[derive(Component, Inspectable, Clone)]
pub struct YSort(pub f32);
impl Default for YSort {
    fn default() -> Self {
        Self(PAWN_LAYER)
    }
}

pub struct ResolutionSetting((f32, f32));

pub struct VSyncSetting(bool);

fn initiate_window_settings(mut windows: ResMut<Windows>, video: Res<VideoSettings>) {
    let window = windows
        .get_primary_mut()
        .expect("primary window not loaded");

    update_window_helper(window, &video);
}

fn order_z_entities(mut query: Query<(&mut Transform, &GlobalTransform, &YSort)>) {
    for (mut transform, global, ysort) in &mut query {
        transform.translation.z =
            ysort.0 - (1.0f32 / (1.0f32 + (2.0f32.powf(-0.00001 * global.translation().y))));
    }
}

fn update_window_settings(
    mut windows: ResMut<Windows>,
    mut events: EventReader<ConfigUpdateEvent>, // TODO make own event..?
    video: Res<VideoSettings>,
) {
    for _ in events.iter() {
        let window = windows.get_primary_mut().unwrap();
        update_window_helper(window, &video);
    }
}

fn update_window_helper(window: &mut Window, video: &Res<VideoSettings>) {
    window.set_present_mode(if video.vsync {
        PresentMode::AutoVsync
    } else {
        PresentMode::AutoNoVsync
    });

    window.set_mode(if video.fullscreen {
        WindowMode::SizedFullscreen
    } else {
        WindowMode::Windowed
    });

    window.set_resolution(video.resolution.0, video.resolution.1);
}
