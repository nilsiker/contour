use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use heron::Velocity;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(apply_drag);
    }
}

#[derive(Component, Clone, Inspectable)]
pub struct Drag(f32);
impl Default for Drag {
    fn default() -> Self {
        Self(100.0)
    }
}

fn apply_drag(mut query: Query<(&mut Velocity, &Drag)>, time: Res<Time>) {
    for (mut velocity, drag) in &mut query {
        velocity.linear = velocity
            .linear
            .lerp(Vec3::default(), drag.0 * time.delta_seconds());
    }
}
