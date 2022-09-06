use bevy::prelude::*;

use self::{camera::FollowCameraPlugin, lighting::LightingPlugin};
pub mod animation;
pub mod camera;
pub mod lighting;

pub struct ContourRenderingPlugins;
impl PluginGroup for ContourRenderingPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(RenderingPlugin)
            .add(FollowCameraPlugin)
            .add(LightingPlugin);
    }
}

struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(order_z_entities);
    }
}

fn order_z_entities(mut query: Query<&mut Transform, With<OrderedZ>>) {
    for mut transform in &mut query {
        transform.translation.z = -transform.translation.y;
    }
}

#[derive(Component)]
pub struct OrderedZ;
