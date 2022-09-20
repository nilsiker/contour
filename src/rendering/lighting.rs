use bevy::prelude::*;

#[derive(Component)]
pub struct GlobalLight(pub bool);

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, _: &mut App) {
        // TODO here I must do shaders and stuff, and update lighting stuffs once I know how to model it...
    }
}
