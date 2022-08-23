use std::iter::Map;

use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::{Collider, GravityScale, LockedAxes, RigidBody};

use crate::{
    animation::Anim,
    constants,
    lighting::{GlobalLight, Lighting},
};

pub struct PlayerCharacterPlugin;

#[derive(Component, Inspectable)]
enum Locomotion {
    Idle,
    Walking,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct Anims(Map<String, Anim>);

#[derive(Component)]
pub struct PlayerPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct PlayerAnimations {
    idle: Anim,
    walk: Anim,
    idle_light: Anim,
    walk_light: Anim,
}

#[derive(Component, Inspectable)]
pub struct PlayerThought(pub &'static str);

#[derive(Component, Reflect)]
pub struct PlayerThoughtTimer(pub Timer);

#[derive(Component, Inspectable)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Inspectable)]
pub struct LightDirection(pub Vec2);

#[derive(Component, Inspectable)]
pub struct Lantern(pub bool);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16f32, 16f32), 18, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let transform = Transform::from_xyz(0., 0., constants::layers::MAIN);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: transform,
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(GravityScale(0.))
        .with_children(|children| {
            children
                .spawn()
                .insert(Name::new("Collider"))
                .insert(Collider::ball(2.5))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0., -6.0, 0.)));
        })
        .insert(MoveDirection(Vec2::new(0., 0.)))
        .insert(LightDirection(Vec2::new(0., 0.)))
        .insert(Locomotion::Idle)
        .insert(AnimationTimer(Timer::from_seconds(0.12, true)))
        .insert(PlayerPosition { x: 0., y: 0. })
        .insert(PlayerAnimations {
            walk_light: Anim::new(0, 7),
            walk: Anim::new(8, 15),
            idle_light: Anim::new(16, 16),
            idle: Anim::new(17, 17),
        })
        .insert(Lantern(false))
        .insert(PlayerThought("Empty..."))
        .insert(PlayerThoughtTimer(Timer::from_seconds(5.0, false)));
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
        } else {
            if flashlight.0 {
                sprite.index = anims.idle_light.step();
            } else {
                sprite.index = anims.idle.step();
            }
        }
    }
}

fn light_thoughts(
    mut query: Query<&mut PlayerThought>,
    lighting_query: Query<&Lighting, Changed<Lighting>>,
) {
    for mut thought in &mut query {
        for lighting in &lighting_query {
            thought.0 = match lighting.0 {
                crate::lighting::LightingMode::Dark => "I'd better light my lantern... [RMB]",
                crate::lighting::LightingMode::Lantern => "I need to be mindful of the battery...",
                crate::lighting::LightingMode::Light => "Phew... safe for now!",
            }
        }
    }
}

fn collider_offset(
    directional_entities: Query<(&LightDirection, &Children)>,
    mut colliders: Query<&mut Transform, With<Collider>>,
) {
    for (direction, children) in &directional_entities {
        for &child in children {
            let mut collider = colliders.get_mut(child).unwrap();
            if direction.0.x > 0. {
                collider.translation.x = 20.;
            } else if direction.0.x < 0. {
                collider.translation.x = -20.;
            }
        }
    }
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&MoveDirection, &mut Transform)>) {
    for (movement, mut transform) in &mut sprite_position {
        let move_vector = Vec3::new(movement.0.x, movement.0.y, 0.) * time.delta_seconds() * 15.;
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

fn lantern_toggle(input: Res<Input<MouseButton>>, mut query: Query<&mut Lantern>) {
    if input.just_pressed(MouseButton::Right) {
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

fn movement_input(input: Res<Input<KeyCode>>, mut movement_input: Query<&mut MoveDirection>) {
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
        }
    }
}

fn update_player_position(mut query: Query<(&mut PlayerPosition, &Transform)>) {
    for (mut player_data, transform) in &mut query {
        player_data.x = transform.translation.x;
        player_data.y = transform.translation.y;
    }
}

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(movement_input)
            .add_system(sprite_movement)
            .add_system(sprite_flipping)
            .add_system(collider_offset)
            .add_system(sprite_animation)
            .add_system(lantern_toggle)
            .add_system(lantern_direction)
            .add_system(update_player_position)
            .add_system(light_thoughts)
            .register_inspectable::<LightDirection>()
            .register_inspectable::<Lantern>();
    }
}
