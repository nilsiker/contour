use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::{Collider, GravityScale, LockedAxes, RigidBody};
use iyes_loopless::prelude::*;

use crate::{
    animation::Anim,
    assets::paths,
    rendering::{self, lighting::GlobalLight},
    state::GameState,
};

use super::{enemy::Enemy, AnimationTimer, GameOver, MoveDirection, Speed};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerAnimations {
    idle: Anim,
    walk: Anim,
    idle_light: Anim,
    walk_light: Anim,
}

#[derive(Component, Reflect)]
pub struct ScreenTextTimer(pub Timer);

#[derive(Component)]
pub struct LightDirection(pub Vec2);

#[derive(Component)]
pub struct Lantern(pub bool);

#[derive(Component)]
pub struct LanternTimer(pub Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(movement_input)
                .with_system(lantern_input)
                .with_system(sprite_animation)
                .with_system(lantern_direction)
                .into(),
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(paths::SPRITE_PLAYER);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16f32, 16f32), 18, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let transform = Transform::from_xyz(0., 0., 0.);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomCenter,
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            transform,
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(0.))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(2.5))
        .insert(MoveDirection(Vec2::new(0., 0.)))
        .insert(Speed(15.0))
        .insert(LightDirection(Vec2::new(0., 0.)))
        .insert(AnimationTimer(Timer::from_seconds(0.12, true)))
        .insert(PlayerAnimations {
            walk_light: Anim::new(0, 7),
            walk: Anim::new(8, 15),
            idle_light: Anim::new(16, 16),
            idle: Anim::new(17, 17),
        })
        .insert(Lantern(false))
        .insert(ScreenTextTimer(Timer::from_seconds(5.0, false)))
        .insert(GameOver(false))
        .insert(rendering::OrderedZ);
}

fn sprite_animation(
    time: Res<Time>,
    mut query: Query<(
        &mut PlayerAnimations,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &MoveDirection,
        &Lantern,
    )>,
) {
    for (mut anims, mut timer, mut sprite, direction, flashlight) in &mut query {
        timer.tick(time.delta());
        if direction.0.length_squared() > 0. {
            if timer.just_finished() {
                if flashlight.0 {
                    sprite.index = anims.walk_light.step();
                } else {
                    sprite.index = anims.walk.step();
                }
            }
        } else if flashlight.0 {
            sprite.index = anims.idle_light.step();
        } else {
            sprite.index = anims.idle.step();
        }
    }
}

fn lantern_input(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Lantern>,
    light: Query<&GlobalLight>,
) {
    let light = light.single();

    if input.just_pressed(KeyCode::F) && !light.0 {
        for mut lantern in &mut query {
            lantern.0 = !lantern.0;
        }
    }
}

fn lantern_direction(input: Res<Input<KeyCode>>, mut query: Query<&mut LightDirection>) {
    for mut vector in &mut query {
        if input.any_pressed([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D]) {
            let mut x = 0f32;
            let mut y = 0f32;
            if input.any_pressed([KeyCode::A, KeyCode::D]) {
                x = if input.pressed(KeyCode::A) {
                    -1f32
                } else {
                    1f32
                };
            }
            if input.any_pressed([KeyCode::W, KeyCode::S]) {
                y = if input.pressed(KeyCode::W) {
                    1f32
                } else {
                    -1f32
                };
            }
            vector.0.x = x;
            vector.0.y = y;
        }
    }
}

fn lantern_extinguisher(time: Res<Time>, mut query: Query<(&mut Lantern, &mut LanternTimer)>) {
    if let Ok((mut lantern, mut timer)) = query.get_single_mut() {
        if lantern.0 {
            if timer.0.just_finished() {
                lantern.0 = false;
                timer.0.reset();
            } else {
                timer.0.tick(time.delta());
            }
        }
    }
}

fn movement_input(
    input: Res<Input<KeyCode>>,
    mut movement_input: Query<&mut MoveDirection, Without<Enemy>>,
) {
    for mut vector in &mut movement_input {
        if input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
            || input.any_just_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
        {
            let x = {
                let mut temp = 0.;
                if input.pressed(KeyCode::A) {
                    temp -= 1.;
                }
                if input.pressed(KeyCode::D) {
                    temp += 1.;
                }
                temp
            };
            let y = {
                let mut temp = 0.;
                if input.pressed(KeyCode::S) {
                    temp -= 1.;
                }

                if input.pressed(KeyCode::W) {
                    temp += 1.
                }
                temp
            };

            vector.0.x = x;
            vector.0.y = y;
            vector.0 = vector.0.normalize_or_zero();
        }
    }
}
