use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use serde::Deserialize;

pub trait FieldReturner {
    fn get_field_value(&self, field_name: &str) -> Option<FieldValue>;
    fn get_value<'a, T: Deserialize<'a> + Default>(&self, field_name: &str) -> T;
}

impl FieldReturner for EntityInstance {
    fn get_field_value(&self, field_name: &str) -> Option<FieldValue> {
        let field = self
            .field_instances
            .iter()
            .find(|field| field.identifier == field_name);

        match field {
            Some(field) => Some(field.value.clone()),
            None => None,
        }
    }

    fn get_value<'a, T: Deserialize<'a> + Default>(&self, _field_name: &str) -> T {
        todo!()
    }
}
