pub mod bgm;
pub mod sfx;

use bevy::prelude::*;
use bgm::BackgroundMusicPlugin;
use sfx::SFXPlugin;

struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, _: &mut App) {
        // Probably add some common denominators
    }
}

pub struct ContourAudioPlugins;
impl PluginGroup for ContourAudioPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(AudioPlugin)
            .add(BackgroundMusicPlugin)
            .add(SFXPlugin);
    }
}
