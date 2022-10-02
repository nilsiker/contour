use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};

use crate::ldtk::utils::FieldReturn;

#[derive(Component)]
pub struct Info(pub String);
impl From<EntityInstance> for Info {
    fn from(entity: EntityInstance) -> Self {
        Self(match entity.get_field_value("text") {
            Some(field_value) => {
                if let FieldValue::String(option) = field_value {
                    if let Some(text) = option {
                        text
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
            None => String::new(),
        })
    }
}

#[derive(Component, Clone, Default)]
pub struct SpriteOffset(pub Vec2);
impl From<EntityInstance> for SpriteOffset {
    fn from(entity: EntityInstance) -> Self {
        match entity.get_value::<Vec2>("sprite_offset") {
            Some(value) => Self(value),
            None => Self::default(),
        }
    }
}

#[derive(Component, Clone)]
pub struct NamedEntity(pub String);
impl From<EntityInstance> for NamedEntity {
    fn from(entity: EntityInstance) -> Self {
        Self(entity.identifier)
    }
}
