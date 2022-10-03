use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use iyes_loopless::{
    prelude::AppLooplessStateExt,
    state::{CurrentState, NextState},
};

use crate::{pawn::player::Player, state::GameState};

use super::{Gate, LastLevelIndex};

struct FadeData {
    speed: f32,
}

pub struct LevelTransitionPlugin;
impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(LevelState::Idle)
            .add_loopless_state(TeleportState::Idle)
            .insert_resource(FadeData { speed: 3.0 })
            .add_event::<StartTransition>()
            .add_event::<LevelChange>()
            .add_startup_system(setup)
            .add_system(center_on_player)
            .add_system(handle_level_transition_events)
            .add_system(start_transition)
            .add_system(end_transition)
            .add_system(perform_level_change)
            .add_system(teleport_player_on_level_change);

        #[cfg(debug_assertions)]
        app.add_system(debug_transition);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LevelState {
    Idle,
    Requested(usize),
    Loaded,
}

#[derive(Component)]
pub struct Fade;
pub struct StartTransition {
    pub from: usize,
    pub to: usize,
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("sprites/fade.png"),
            sprite: Sprite {
                color: Color::NONE,
                ..default()
            },
            ..default()
        })
        .insert(Fade);
}

fn debug_transition(
    mut toggle: Local<bool>,
    input: Res<Input<KeyCode>>,
    mut transitions: EventWriter<StartTransition>,
) {
    if input.just_pressed(KeyCode::End) {
        if *toggle {
            transitions.send(StartTransition { from: 1, to: 0 });
        } else {
            transitions.send(StartTransition { from: 0, to: 1 });
        }
        *toggle = !*toggle
    }
}

fn center_on_player(
    players: Query<&Transform, (With<Player>, Without<Fade>)>,
    mut fades: Query<&mut Transform, (Without<Player>, With<Fade>)>,
) {
    if let Ok(player) = players.get_single() {
        if let Ok(mut fade) = fades.get_single_mut() {
            fade.translation.x = player.translation.x;
            fade.translation.y = player.translation.y;
            fade.translation.z = 50.0;
        }
    }
}

fn handle_level_transition_events(
    mut events: EventReader<StartTransition>,
    mut last_level_index: ResMut<LastLevelIndex>,
    mut commands: Commands,
) {
    for event in events.iter() {
        commands.insert_resource(NextState(LevelState::Requested(event.to)));
        commands.insert_resource(NextState(GameState::Loading));
        *last_level_index = LastLevelIndex(event.from);
    }
}
struct LevelChange(usize);

fn start_transition(
    mut query: Query<&mut Sprite, With<Fade>>,
    mut level_changes: EventWriter<LevelChange>,
    current_state: Res<CurrentState<LevelState>>,
    time: Res<Time>,
    fade: ResMut<FadeData>,
) {
    if let LevelState::Requested(next_level) = current_state.0 {
        for mut sprite in &mut query {
            let a = sprite.color.a();

            let step = &(time.delta_seconds() * fade.speed);
            let next_a = f32::min(1.0, a + step);
            sprite.color.set_a(next_a);

            if sprite.color.a() == 1.0 {
                level_changes.send(LevelChange(next_level));
            }
        }
    }
}

fn perform_level_change(
    mut level_changes: EventReader<LevelChange>,
    mut level: ResMut<LevelSelection>,
    mut commands: Commands,
    last_level: Res<LastLevelIndex>,
) {
    for change in level_changes.iter() {
        *level = LevelSelection::Index(change.0);

        commands.insert_resource(NextState(LevelState::Loaded));
        commands.insert_resource(NextState(TeleportState::Pending(last_level.0)));
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum TeleportState {
    Pending(usize),
    Idle,
}

fn teleport_player_on_level_change(
    mut players: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    teleport_state: Res<CurrentState<TeleportState>>,
    gates: Query<(&Transform, &Gate), (Added<Gate>, Without<Player>)>,
) {
    if let TeleportState::Pending(gate_index) = teleport_state.0 {
        if let Ok(mut player) = players.get_single_mut() {
            if let Some((transform, gate)) = gates.iter().find(|(_, gate)| match gate {
                Gate::Entry(from) => *from == gate_index,
                Gate::Exit(_) => false,
            }) {
                bevy::log::info!(
                    "found a gate entrance from {} with translation {:?}",
                    gate_index,
                    transform.translation
                );
                if let Gate::Entry(_) = *gate {
                    player.translation = transform.translation;
                    commands.insert_resource(NextState(TeleportState::Idle));
                    commands.insert_resource(NextState(GameState::InGame));
                }
            }
        }
    }
}

fn end_transition(
    mut query: Query<&mut Sprite, With<Fade>>,
    mut commands: Commands,
    current_state: Res<CurrentState<LevelState>>,

    time: Res<Time>,
    fade: ResMut<FadeData>,
) {
    if let LevelState::Loaded = current_state.0 {
        for mut sprite in &mut query {
            let a = sprite.color.a();

            let step = &(time.delta_seconds() * -fade.speed);
            let next_a = f32::max(0.0, a + step);
            sprite.color.set_a(next_a);

            if sprite.color.a() == 0.0 {
                commands.insert_resource(NextState(LevelState::Idle));
            }
        }
    }
}
