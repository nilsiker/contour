use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub mod enemy;
pub mod player;

#[derive(Component, Inspectable)]
pub struct GameOver(pub bool);

#[derive(Component, Inspectable)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Speed(f32);

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&MoveDirection, &mut Transform, &Speed)>,
) {
    for (movement, mut transform, speed) in &mut sprite_position {
        let move_vector =
            Vec3::new(movement.0.x, movement.0.y, 0.) * time.delta_seconds() * speed.0;
        transform.translation += move_vector;
    }
}

fn sprite_flipping(mut query: Query<(&mut TextureAtlasSprite, &MoveDirection)>) {
    for (mut sprite, movement) in &mut query {
        if movement.0.x < 0. {
            sprite.flip_x = true
        } else if movement.0.x > 0. {
            sprite.flip_x = false
        }
    }
}
