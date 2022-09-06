use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio};

use crate::consts::path::AUDIO_BGM;

pub struct BackgroundMusicPlugin;
impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(play_background_music);
    }
}

pub fn play_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load(AUDIO_BGM))
        .looped()
        .with_volume(0.75);
}
