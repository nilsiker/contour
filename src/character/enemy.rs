use std::time::Duration;

use crate::{
    animation::Anim,
    lighting::{GlobalLight, Lighting},
    player::PlayerPosition,
    rendering,
    text::MainText,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::{Collider, RapierContext, Sensor};
use rand::prelude::*;

use super::{sprite_flipping, AnimationTimer, GameOver, MoveDirection, Speed};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct EnemySpawnTimer(Timer);

struct EnemyAtlas(Handle<TextureAtlas>);

#[derive(Component)]
struct EnemyAnimations {
    stalk: Anim,
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(spawn_enemies)
            .add_system(set_move_to_player)
            .add_system(enemy_intersecting_player)
            .add_system(sprite_flipping)
            .add_system(sprite_animation)
            .add_system(hide_in_light);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("enemy.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16f32, 16f32), 8, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(EnemyAtlas(texture_atlas_handle));
    commands
        .spawn()
        .insert(Name::new("Enemy Spawner".to_string()))
        .insert(EnemySpawnTimer(Timer::new(Duration::from_secs(1), true)));
}

fn spawn_enemies(
    mut commands: Commands,
    query: Query<&GlobalLight>,
    player_position_query: Query<&Transform, With<PlayerPosition>>,
    mut timer_query: Query<&mut EnemySpawnTimer>,
    time: Res<Time>,
    texture_atlas: Res<EnemyAtlas>,
) {
    for global_light in &query {
        for player in &player_position_query {
            if !global_light.0 {
                let mut timer = timer_query.single_mut();
                if timer.0.just_finished() {
                    let angle: f32 = rand::thread_rng().gen_range(0f32..std::f32::consts::TAU);
                    let distance = rand::thread_rng().gen_range(20f32..100f32);
                    let x = player.translation.x + (angle.cos() * distance);
                    let y = player.translation.y + (angle.sin() * distance);
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                anchor: Anchor::BottomCenter,
                                ..default()
                            },
                            texture_atlas: texture_atlas.0.clone(),
                            transform: Transform::from_xyz(x, y, 0.),
                            ..default()
                        })
                        .insert(Collider::ball(2.5))
                        .insert(Sensor)
                        .insert(Enemy)
                        .insert(MoveDirection(Vec2::default()))
                        .insert(Speed(rand::thread_rng().gen_range(1f32..10f32)))
                        .insert(EnemyAnimations {
                            stalk: Anim::new(0, 7),
                        })
                        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
                        .insert(rendering::OrderedZ);
                    timer.0.reset();
                } else {
                    timer.0.tick(time.delta());
                }
            }
        }
    }
}

fn sprite_animation(
    time: Res<Time>,
    mut query: Query<(
        &mut EnemyAnimations,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    lighting_query: Query<&GlobalLight>,
) {
    for (mut anims, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if let Ok(global_light) = lighting_query.get_single() {
            if !global_light.0 && timer.just_finished() {
                sprite.index = anims.stalk.step()
            }
        }
    }
}

fn set_move_to_player(
    mut enemy_query: Query<(&Transform, &mut MoveDirection), With<Enemy>>,
    player_query: Query<&Transform, With<PlayerPosition>>,
    lighting_query: Query<&Lighting>,
) {
    let player = player_query.single();
    if let Ok(lighting) = lighting_query.get_single() {
        for (enemy_transform, mut move_direction) in &mut enemy_query {
            let delta_x = player.translation.x - enemy_transform.translation.x;
            let delta_y = player.translation.y - enemy_transform.translation.y;

            let mut direction = Vec2::new(delta_x, delta_y).normalize_or_zero();

            match lighting.0 {
                crate::lighting::LightingMode::Lantern => {
                    direction.x /= 2.0;
                    direction.y /= 2.0;
                }
                crate::lighting::LightingMode::Light => {
                    direction.x = 0.0;
                    direction.y = 0.0;
                }
                _ => (),
            }
            move_direction.0.x = direction.x;
            move_direction.0.y = direction.y;
        }
    }
}

fn enemy_intersecting_player(
    rapier: Res<RapierContext>,
    mut player: Query<(&mut MainText, Entity, &mut GameOver)>,
    mut camera: Query<&mut OrthographicProjection>,
) {
    for (mut text, entity, mut game_over) in &mut player {
        let intersections: Vec<(Entity, Entity, bool)> =
            rapier.intersections_with(entity).collect();

        if intersections.len() > 0 {
            bevy::log::info!("{}", intersections.len());
            game_over.0 = true;
            for mut projection in &mut camera {
                projection.scale = 0.05;
                text.0 = "GAME OVER";
            }
        }
    }
}

fn hide_in_light(
    mut query: Query<&mut Visibility, With<Enemy>>,
    global_light: Query<&GlobalLight, Changed<GlobalLight>>,
) {
    for light in &global_light {
        for mut visibility in &mut query {
            visibility.is_visible = !light.0;
        }
    }
}
