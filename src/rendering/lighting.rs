use bevy::prelude::*;

#[derive(Component)]
pub struct GlobalLight(pub bool);
pub enum Time {
    Day,
    Night
}

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::Day)
        // TODO here I must do shaders and stuff, and update lighting stuffs once I know how to model it...
        ;
    }
}
