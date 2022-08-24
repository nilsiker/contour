use bevy::prelude::*;

#[derive(Component)]
pub struct OrderedZ;

pub fn order_z_entities(mut query: Query<&mut Transform, With<OrderedZ>>) {
    for mut transform in &mut query {
        transform.translation.z = -transform.translation.y;
    }
}