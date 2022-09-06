mod background_music;

use bevy::prelude::*;

use background_music::BackgroundMusicPlugin;

pub struct ContourAudioPlugins;
impl PluginGroup for ContourAudioPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(BackgroundMusicPlugin);
    }
}
