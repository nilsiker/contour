pub mod bgm;
pub mod sfx;

use bevy::prelude::*;
use bgm::BackgroundMusicPlugin;
use sfx::SFXPlugin;

pub struct AudioPlugin;
impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_kira_audio::AudioPlugin)
            .add_plugin(BackgroundMusicPlugin)
            .add_plugin(SFXPlugin);
    }
}
