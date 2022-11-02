use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use serde::de::DeserializeOwned;



pub trait FieldReturn {
    fn get_field_value(&self, field_name: &str) -> Option<FieldValue>;
    fn get_string_value(&self, field_name: &str) -> Option<String>;
    fn get_value<T: DeserializeOwned>(&self, field_name: &str) -> Option<T>;
}
impl FieldReturn for EntityInstance {
    fn get_field_value(&self, field_name: &str) -> Option<FieldValue> {
        let field = self
            .field_instances
            .iter()
            .find(|field| field.identifier == field_name);

        match field {
            Some(field) => Some(field.value.clone()),
            None => {
                bevy::log::warn!("No field {field_name} on {}", self.identifier);
                None
            }
        }
    }

    fn get_string_value(&self, field_name: &str) -> Option<String> {
        match self.get_field_value(field_name) {
            Some(field_value) => match &field_value {
                FieldValue::String(value) | FieldValue::Enum(value) => value.clone(),
                _ => {
                    bevy::log::warn!(
                        "Field {field_name} on {} is not of type String.",
                        self.identifier
                    );
                    None
                }
            },
            None => None,
        }
    }

    fn get_value<T: DeserializeOwned>(&self, field_name: &str) -> Option<T> {
        match self.get_string_value(field_name) {
            Some(string) => match ron::from_str::<T>(&string) {
                Ok(value) => Some(value),
                Err(_) => {
                    bevy::log::warn!(
                        target: "FieldReturn",
                        "Failed to deserialize field value {field_name} on {}.", self.identifier
                    );
                    None
                }
            },
            None => None,
        }
    }
}
