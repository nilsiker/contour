use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

use crate::physics::Drag;

use self::{
    entities::EntitiesPlugin, int_grid_cells::IntGridCellPlugin, level::LevelPlugin,
    utils::FieldReturner,
};

pub mod entities;
mod int_grid_cells;
mod level;
pub mod utils;

pub struct LdtkPlugin;
impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EntitiesPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(IntGridCellPlugin);
    }
}

#[derive(Clone, Default, Bundle)]
struct PhysicsBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
    pub physic_material: PhysicMaterial,
    pub drag: Drag,
}

impl From<EntityInstance> for PhysicsBundle {
    fn from(entity_instance: EntityInstance) -> PhysicsBundle {
        let rotation_constraints = RotationConstraints::lock();

        let rigidbody_type = match entity_instance.get_field_value("static") {
            Some(_) => RigidBody::Static,
            None => RigidBody::Dynamic,
        };

        match entity_instance.identifier.as_ref() {
            "Player" => PhysicsBundle {
                collider: CollisionShape::Sphere { radius: 1.5 },
                rigid_body: rigidbody_type,
                rotation_constraints,
                ..default()
            },
            "Container" => PhysicsBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: (6.0, 3.0, 0.0).into(),
                    border_radius: None,
                },
                rigid_body: rigidbody_type,
                rotation_constraints,
                ..default()
            },
            "LevelGate" => PhysicsBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: (2.0, 2.0, 0.0).into(),
                    border_radius: None,
                },
                rigid_body: RigidBody::Sensor,
                rotation_constraints,
                ..default()
            },
            _ => PhysicsBundle::default(),
        }
    }
}
