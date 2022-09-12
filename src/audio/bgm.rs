use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio};

use crate::{consts::path::AUDIO_BGM, ui::options_menu::AudioConfig};

pub struct BackgroundMusicPlugin;
impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app // TODO read this initial value from config file
            .add_startup_system(setup)
            .add_system(handle_background_music_changed);
    }
}

fn setup(assets: Res<AssetServer>, audio: Res<Audio>, config: Res<AudioConfig>) {
    audio
        .play(assets.load(AUDIO_BGM))
        .looped()
        .with_volume(config.bgm / 100.0);
}

pub fn handle_background_music_changed(audio: Res<Audio>, config: Res<AudioConfig>) {
    if config.is_changed() {
        audio.set_volume(config.bgm / 100.0);
    }
}
