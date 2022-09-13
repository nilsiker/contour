use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio};

use crate::{config::AudioSettings, consts::path::AUDIO_BGM};

pub struct BgmVolumeChangedEvent;

pub struct BackgroundMusicPlugin;
impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app // TODO read this initial value from config file
            .add_event::<BgmVolumeChangedEvent>()
            .add_startup_system(setup)
            .add_system(handle_background_music_changed);
    }
}

fn setup(assets: Res<AssetServer>, audio: Res<Audio>, config: Res<AudioSettings>) {
    audio
        .play(assets.load(AUDIO_BGM))
        .looped()
        .with_volume(config.bgm / 100.0);
}

pub fn handle_background_music_changed(
    audio: Res<Audio>,
    config: Res<AudioSettings>,
    mut events: EventReader<BgmVolumeChangedEvent>,
) {
    for _ in events.iter() {
        audio.set_volume(config.bgm / 100.0);
    }
}
