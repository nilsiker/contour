use std::{collections::HashMap, iter::Map, time::Duration};

use crate::{
    animation::Anim,
    game::GameState,
    lighting::{GlobalLight, Lighting},
    rendering,
    text::{MainText, Score, SubText},
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::{Collider, RapierContext, Sensor};
use rand::prelude::*;

use super::{
    player::PlayerPosition, sprite_flipping, AnimationTimer, GameOver, MoveDirection, Speed,
};

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

#[derive(Component, Inspectable)]
struct Merge(usize);

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(spawn_enemies)
                    .with_system(enemy_intersecting_player)
                    .with_system(hide_in_light)
                    .with_system(make_dangerous),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(GameState::InGame).with_system(merge),
            )
            .add_system(set_move_to_player)
            .add_system(sprite_animation)
            .register_inspectable::<Merge>();
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
    player_query: Query<&Transform, With<PlayerPosition>>,
    score_query: Query<&Score>,
    mut timer_query: Query<&mut EnemySpawnTimer>,
    time: Res<Time>,
    texture_atlas: Res<EnemyAtlas>,
) {
    for global_light in &query {
        for player in &player_query {
            if !global_light.0 {
                let mut timer = timer_query.single_mut();

                if timer.0.just_finished() {
                    timer.0.set_duration(Duration::from_secs_f32(
                        1.0 - (score_query.single().0) / 100.0,
                    ));
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
            if merge1 > merge2 && merge1 < 5 {
                let most_merged_entity = entities.get_mut(0).unwrap();

                most_merged_entity.1.scale *= 1.1;
                most_merged_entity.3 .0 = speed1.max(speed2);

                let mut ball = most_merged_entity.2.as_ball_mut().unwrap();
                ball.set_radius(ball.radius() * 1.1);
                most_merged_entity.4 .0 += 1;
                commands
                    .entity(entities.get_mut(1).unwrap().0)
                    .despawn_recursive();
            } else if merge2 < 5 {
                let most_merged_entity = entities.get_mut(1).unwrap();

                most_merged_entity.1.scale *= 1.1;
                most_merged_entity.3 .0 = speed1.max(speed2);

                let mut ball = most_merged_entity.2.as_ball_mut().unwrap();
                ball.set_radius(ball.radius() * 1.1);
                most_merged_entity.4 .0 += 1;
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
    mut state: ResMut<State<GameState>>,
    rapier: Res<RapierContext>,
    mut player: Query<(
        &mut MainText,
        &mut SubText,
        Entity,
        &mut GameOver,
        &mut TextureAtlasSprite,
    )>,
    mut camera: Query<&mut OrthographicProjection>,
    score: Query<&Score>,
) {
    for (mut text, mut sub, entity, mut game_over, mut sprite) in &mut player {
        let intersections: Vec<(Entity, Entity, bool)> =
            rapier.intersections_with(entity).collect();

        if intersections.len() > 0 {
            bevy::log::info!("{}", intersections.len());
            game_over.0 = true;
            for mut projection in &mut camera {
                projection.scale = 0.15;
                text.0 = "GAME OVER".to_owned();
                sub.0 = format!("Score: {}", score.single().0.round().to_owned());
            }
            state.set(GameState::GameOver).unwrap();
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
