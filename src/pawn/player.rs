use bevy::prelude::*;

use iyes_loopless::prelude::*;

use crate::state::GameState;

use super::{enemy::Enemy, MoveDirection};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(movement_input)
                .with_system(lantern_input)
                .with_system(lantern_direction)
                .into(),
        );
    }
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
