use bevy::prelude::*;

use iyes_loopless::prelude::*;

use crate::{animation::Animations, state::GameState};

use super::{enemy::Enemy, MoveDirection};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_input)
            .add_loopless_state(PlayerState::Unpaused)
            .add_system(set_scale)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(PlayerState::Unpaused)
                    .with_system(lantern_input)
                    .with_system(lantern_direction)
                    .into(),
            )
            .add_system(component_status_on_state_change)
            .add_enter_system(GameState::Loading, |mut commands: Commands| {
                commands.insert_resource(NextState(PlayerState::Paused));
            })
            .add_exit_system(GameState::Loading, |mut commands: Commands| {
                commands.insert_resource(NextState(PlayerState::Unpaused));
            });
    }
}

fn set_scale(mut query: Query<&mut Transform, Added<Player>>) {
    for mut transform in &mut query {
        transform.scale *= 0.85;
    }
}

fn component_status_on_state_change(
    mut query: Query<&mut Animations, With<Player>>,
    state: Res<CurrentState<PlayerState>>,
) {
    if state.is_changed() {
        for mut animation in &mut query {
            animation.active = match state.0 {
                PlayerState::Paused => false,
                PlayerState::Unpaused => true,
            };
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerState {
    Paused,
    Unpaused,
}

#[derive(Component, Clone)]
pub struct Player;

#[derive(Component)]
pub struct LightDirection(pub Vec2);

#[derive(Component, Clone)]
pub struct Lantern(pub bool);

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
