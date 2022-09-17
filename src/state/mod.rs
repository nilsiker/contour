use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loopless_state(GameState::InGame);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Cutscene,
    InGame,
    UI,
}
