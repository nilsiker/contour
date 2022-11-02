use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

use crate::physics::Drag;

use self::{
    entities::EntitiesPlugin, int_grid_cells::IntGridCellPlugin, level::LevelPlugin,
    utils::FieldReturn,
};

pub mod entities;
mod int_grid_cells;
pub mod level;
pub mod utils;

pub struct LdtkPlugin;
impl Plugin for LdtkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EntitiesPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(IntGridCellPlugin);
    }
}

// TODO move to bundle.rs
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

        let rigid_body = get_rigidbody_from_entity(&entity_instance);
        let collider = get_collision_shape_from_entity(&entity_instance);

        PhysicsBundle {
            collider,
            rigid_body,
            rotation_constraints,
            ..default()
        }
    }
}

fn get_rigidbody_from_entity(entity_instance: &EntityInstance) -> RigidBody {
    match entity_instance.get_string_value("physics") {
        Some(physics_value) => match physics_value.as_str() {
            "Fixed" => RigidBody::Static,
            "Dynamic" => RigidBody::Dynamic,
            _ => {
                bevy::log::info!("{} on {}", physics_value, entity_instance.identifier);
                RigidBody::Sensor
            }
        },
        _ => {
            bevy::log::warn!(
                "{} defaulting to Rigidbody::Sensor",
                entity_instance.identifier
            );
            RigidBody::Sensor
        }
    }
}

fn get_collision_shape_from_entity(entity_instance: &EntityInstance) -> CollisionShape {
    match entity_instance.get_string_value("collider_shape") {
        Some(shape) => match shape.as_str() {
            "Sphere" => {
                let radius = entity_instance
                    .get_value::<f32>("radius")
                    .unwrap_or_default();
                dbg!(&radius);
                CollisionShape::Sphere { radius }
            }
            _ => {
                let half_extends = match entity_instance.get_value::<Vec3>("collider_size") {
                    Some(vec) => vec,
                    None => {
                        bevy::log::info!(
                            "No collider_size specified on {}.",
                            entity_instance.identifier
                        );
                        Vec3::default()
                    }
                };

                CollisionShape::Cuboid {
                    half_extends,
                    border_radius: None,
                }
            }
        },
        _ => {
            bevy::log::warn!(
                "{} defaulting to Sensor Sphere with radius 1.5",
                entity_instance.identifier
            );
            CollisionShape::Cuboid {
                half_extends: (4.0, 4.0, 0.0).into(),
                border_radius: None,
            }
        }
    }
}
