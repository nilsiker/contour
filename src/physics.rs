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

pub trait TupleUtil<T> {
    fn any(&self, element: T) -> bool;
    fn both(&self, element1: T, element2: T) -> bool;
    fn not(&self, element: T) -> Option<T>;
}

impl TupleUtil<Entity> for (Entity, Entity) {
    fn any(&self, element: Entity) -> bool {
        self.0 == element || self.1 == element
    }

    fn both(&self, element1: Entity, element2: Entity) -> bool {
        self.any(element1) && self.any(element2)
    }

    fn not(&self, element: Entity) -> Option<Entity> {
        if self.0 == element {
            Some(self.1)
        } else if self.1 == element {
            Some(self.0)
        } else {
            None
        }
    }
}
