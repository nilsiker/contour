use bevy::{prelude::*, sprite::Anchor, transform::TransformSystem};
use bevy_ecs_ldtk::EntityInstance;
use bevy_rapier2d::prelude::{Collider, GravityScale, LockedAxes, RigidBody};
use iyes_loopless::prelude::*;

use crate::{animation::Animations, assets::paths, rendering, save, state::GameState};

use super::{enemy::Enemy, MoveDirection, Speed};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            spawn_player
                .run_if_not(save::save_exists)
                .after(TransformSystem::TransformPropagate),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(movement_input)
                .with_system(lantern_input)
                .with_system(lantern_direction)
                .into(),
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LightDirection(pub Vec2);

#[derive(Component)]
pub struct Lantern(pub bool);

fn spawn_player(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    query: Query<(&GlobalTransform, &EntityInstance)>,
    player: Query<&Player>, // TODO figure out timing so this does not have to be an on-update system.
) {
    if player.is_empty() {
        bevy::log::info!("Trying to spawn player");
        if let Some((transform, _)) = query
            .iter()
            .filter(|(_, i)| i.identifier == "Player")
            .nth(0)
        {
            let sprite = TextureAtlasSprite {
                anchor: Anchor::BottomCenter,
                ..default()
            };
            let texture_handle = asset_server.load(paths::SPRITE_PLAYER);
            let texture_atlas_base =
                TextureAtlas::from_grid(texture_handle, Vec2::new(16f32, 16f32), 18, 1);
            let texture_atlas = texture_atlases.add(texture_atlas_base);
            let transform = Transform {
                translation: transform.translation(),
                ..default()
            };
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite,
                    texture_atlas,
                    transform,
                    ..default()
                })
                .insert(Name::new("Player"))
                .insert(Player)
                // Physics & Collision
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(GravityScale(0.))
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(2.5))
                // Movement & Animation
                .insert(MoveDirection(Vec2::new(0., 0.)))
                .insert(Speed(50.0))
                .insert(Animations::from_file("player.ron"))
                .insert(Lantern(false))
                .insert(rendering::OrderedZ);
            bevy::log::info!("Player spawned at {}", transform.translation);
        }
    }
}

fn lantern_input(input: Res<Input<KeyCode>>, mut query: Query<&mut Lantern>) {
    if input.just_pressed(KeyCode::F) {
        for mut lantern in &mut query {
            lantern.0 = !lantern.0;
        }
    }
}

fn lantern_direction(input: Res<Input<KeyCode>>, mut query: Query<&mut LightDirection>) {
    for mut vector in &mut query {
        if input.any_pressed([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D]) {
            let mut new_direction = Vec2::default();
            if input.any_pressed([KeyCode::A, KeyCode::D]) {
                new_direction.x = if input.pressed(KeyCode::A) { -1.0 } else { 1.0 };
            }
            if input.any_pressed([KeyCode::W, KeyCode::S]) {
                new_direction.y = if input.pressed(KeyCode::W) { 1.0 } else { -1.0 };
            }
            vector.0 = new_direction;
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
