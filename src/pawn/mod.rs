use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};

use crate::ldtk::utils::FieldReturner;

use self::{enemy::EnemyPlugin, player::PlayerPlugin};

pub mod enemy;
pub mod player;

pub struct PawnPlugin;
impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_system(character_movement);
    }
}

#[derive(Component, Clone, Default)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Clone, Default)]
pub struct Speed(pub f32);
impl From<EntityInstance> for Speed {
    fn from(entity: EntityInstance) -> Self {
        match entity.get_field_value("speed") {
            Some(speed) => {
                if let FieldValue::Float(speed) = speed {
                    if let Some(speed) = speed {
                        Self(speed)
                    } else {
                        Self::default()
                    }
                } else {
                    Self::default()
                }
            }
            None => Self::default(),
        }
    }
}

fn character_movement(
    time: Res<Time>,
    mut characters: Query<(&MoveDirection, &mut Transform, &Speed)>,
) {
    for (movement, mut transform, speed) in &mut characters {
        let move_vector =
            Vec3::new(movement.0.x, movement.0.y, 0.) * time.delta_seconds() * speed.0;
        transform.translation += move_vector;
    }
}
