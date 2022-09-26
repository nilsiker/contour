use bevy::prelude::*;
use iyes_loopless::{prelude::AppLooplessStateExt, state::CurrentState};

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loopless_state(GameState::Loading);

        #[cfg(debug_assertions)]
        app.add_system(debug_game_state_change.label("debug"));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    Cutscene,
    InGame,
    UI,
}

fn debug_game_state_change(
    game_state: Res<CurrentState<GameState>>,
) {
    if game_state.is_changed() {
        bevy::log::info!("GameState changed: {:?}", game_state.0)
    }
}
