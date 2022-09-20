use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

use self::{entities::EntitiesPlugin, int_grid_cells::IntGridCellPlugin, level::LevelPlugin};

mod entities;
mod int_grid_cells;
mod level;

pub struct LdtkPlugin;
impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EntitiesPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(IntGridCellPlugin);
    }
}

#[derive(Clone, Debug, Default, Bundle)]
struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
    pub physic_material: PhysicMaterial,
}

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = RotationConstraints::lock();

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: CollisionShape::Sphere { radius: 2.5 },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                physic_material: PhysicMaterial {
                    restitution: 0.0,
                    density: 50.0,
                    friction: 1.0,
                },
                ..default()
            },
            "Crate" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: (8.0, 2.0, 0.0).into(),
                    border_radius: None,
                },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                physic_material: PhysicMaterial {
                    restitution: 0.0,
                    density: 25.0,
                    friction: 1.0,
                },
                ..default()
            },
            _ => ColliderBundle::default(),
        }
    }
}
