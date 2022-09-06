use bevy::prelude::*;

use self::{
    enemy::{Enemy, EnemyPlugin},
    player::PlayerPlugin,
};

pub mod enemy;
pub mod player;

pub struct ContourPawnPlugins;
impl PluginGroup for ContourPawnPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(PawnPlugin).add(PlayerPlugin).add(EnemyPlugin);
    }
}

struct PawnPlugin;
impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(character_movement)
            .add_system(sprite_flipping);
    }
}

#[derive(Component)]
pub struct GameOver(pub bool);

#[derive(Component)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct Speed(f32);

fn character_movement(
    time: Res<Time>,
    mut characters: Query<(&MoveDirection, &mut Transform, &Speed), With<Enemy>>,
) {
    for (movement, mut transform, speed) in &mut characters {
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
