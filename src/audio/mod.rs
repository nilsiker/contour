mod background_music;
mod sfx;

use std::collections::HashMap;

use bevy::prelude::*;

use background_music::BackgroundMusicPlugin;

use self::sfx::SFXPlugin;

pub enum AudioChannel {
    BGM,
    SFX,
}

pub struct AudioChannels {
    pub bgm: Volume,
    pub sfx: Volume,
}

pub struct Volume(pub f64);

struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioChannels {
            bgm: Volume(50.0),
            sfx: Volume(50.0),
        });
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
