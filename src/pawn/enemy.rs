use std::time::Duration;

use crate::{ai::Destination, assets::paths};
use bevy::prelude::*;

use super::MoveDirection;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct EnemySpawnTimer(Timer);

struct EnemyAtlas(Handle<TextureAtlas>);

#[derive(Component)]
struct DangerousTimer(Timer);

#[derive(Component)]
struct Merge(usize);

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(set_move_to_destination);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(paths::SPRITE_ENEMY);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16f32, 16f32), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(EnemyAtlas(texture_atlas_handle));
    commands
        .spawn()
        .insert(Name::new("Enemy Spawner".to_string()))
        .insert(EnemySpawnTimer(Timer::new(Duration::from_secs(1), true)));
}

fn set_move_to_destination(
    mut enemies: Query<(&Transform, &Destination, &mut MoveDirection), With<Enemy>>,
) {
    for (transform, destination, mut move_direction) in &mut enemies {
        let delta_x = destination.0.translation.x - transform.translation.x;
        let delta_y = destination.0.translation.y - transform.translation.y;

        let direction = Vec2::new(delta_x, delta_y).normalize_or_zero();

        move_direction.0.x = direction.x;
        move_direction.0.y = direction.y;
    }
}
