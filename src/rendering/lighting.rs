use bevy::prelude::*;

#[derive(Component)]
pub struct GlobalLight(pub bool);

/// A darkness struct, where 0 represents no global darkness and 1 represents complete darkness.
pub struct Darkness(f32);
pub struct LightingPlugin;
impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Darkness(0.0))
        // TODO here I must do shaders and stuff, and update lighting stuffs once I know how to model it...
        ;
    }
}
