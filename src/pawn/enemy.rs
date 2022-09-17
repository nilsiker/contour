use std::time::Duration;

use crate::{
    ai::Destination,
    consts::path,
    rendering::{self, animation::Anim, lighting::GlobalLight},
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::{Collider, RapierContext, Sensor};
use rand::prelude::*;

use super::{player::Player, AnimationTimer, MoveDirection, Speed};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct EnemySpawnTimer(Timer);

struct EnemyAtlas(Handle<TextureAtlas>);

#[derive(Component)]
struct EnemyAnimations {
    stalk: Anim,
}

#[derive(Component)]
struct DangerousTimer(Timer);

#[derive(Component)]
struct Merge(usize);

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(set_move_to_destination)
            .add_system(sprite_animation);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(path::SPRITE_ENEMY);
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
    player_query: Query<&Transform, With<Player>>,
    mut timer_query: Query<&mut EnemySpawnTimer>,
    time: Res<Time>,
    texture_atlas: Res<EnemyAtlas>,
) {
    for global_light in &query {
        for player in &player_query {
            if !global_light.0 {
                let mut timer = timer_query.single_mut();

                if timer.0.just_finished() {
                    let angle: f32 = rand::thread_rng().gen_range(0f32..std::f32::consts::TAU);
                    let distance = 60f32;
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
                        .insert(Enemy)
                        .insert(Collider::ball(2.5))
                        .insert(MoveDirection(Vec2::default()))
                        .insert(Speed(rand::thread_rng().gen_range(1f32..18f32)))
                        .insert(EnemyAnimations {
                            stalk: Anim::new(0, 7),
                        })
                        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
                        .insert(rendering::OrderedZ)
                        .insert(DangerousTimer(Timer::from_seconds(0.5, false)))
                        .insert(Merge(0));
                    timer.0.reset();
                } else {
                    timer.0.tick(time.delta());
                }
            }
        }
    }
}

fn merge(
    mut commands: Commands,
    rapier: Res<RapierContext>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut Collider,
            &mut Speed,
            &mut Merge,
        ),
        (With<Enemy>, With<Sensor>),
    >,
) {
    rapier.intersection_pairs().for_each(|(e1, e2, _)| {
        let entities_result = query.get_many_mut([e1, e2]);
        if let Ok(mut entities) = entities_result {
            let speed1 = entities[0].3 .0;
            let speed2 = entities[1].3 .0;
            let merge1 = entities[0].4 .0;
            let merge2 = entities[1].4 .0;
            if merge1 >= merge2 && merge1 < 5 {
                let most_merged_entity = entities.get_mut(0).unwrap();
                bevy::log::info!("merge {} into {}", merge1, merge2);

                most_merged_entity.1.scale *= 1.1;
                most_merged_entity.3 .0 = speed1.max(speed2);

                let mut ball = most_merged_entity.2.as_ball_mut().unwrap();
                ball.set_radius(ball.radius() * 1.1);
                most_merged_entity.4 .0 += 1;

                commands
                    .entity(entities.get_mut(1).unwrap().0)
                    .despawn_recursive();
            } else if merge2 >= merge1 && merge2 < 5 {
                let most_merged_entity = entities.get_mut(1).unwrap();
                bevy::log::info!("merge {} into {}", merge2, merge1);
                most_merged_entity.1.scale *= 1.1;
                most_merged_entity.3 .0 = speed1.max(speed2);

                let mut ball = most_merged_entity.2.as_ball_mut().unwrap();
                ball.set_radius(ball.radius() * 1.1);
                most_merged_entity.4 .0 += 1;
                bevy::log::info!("{}", most_merged_entity.4 .0);
                commands
                    .entity(entities.get_mut(0).unwrap().0)
                    .despawn_recursive();
            };
        }
    });
}

fn make_dangerous(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DangerousTimer)>,
) {
    for (entity, mut dangerous) in &mut query {
        if dangerous.0.just_finished() {
            commands.entity(entity).insert(Sensor);
            dangerous.0.reset();
        } else {
            dangerous.0.tick(time.delta());
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
