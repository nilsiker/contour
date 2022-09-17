use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::game::WorldState;

pub fn paused(state: Res<WorldState>) -> bool {
    *state == WorldState::Paused
}
