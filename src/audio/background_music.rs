use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio, AudioSource};

use super::AudioChannels;

pub struct BackgroundMusicChangedEvent(Handle<AudioSource>);

pub struct BackgroundMusicPlugin;
impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackgroundMusicChangedEvent>() // TODO read this initial value from config file
            .add_system(handle_background_music_changed);
    }
}

pub fn handle_background_music_changed(
    mut background_music_changed_events: EventReader<BackgroundMusicChangedEvent>,
    audio: Res<Audio>,
    audio_channels: Res<AudioChannels>,
) {
    for event in background_music_changed_events.iter() {
        if audio.is_playing_sound() {
            audio.stop();
        }

        audio
            .play(event.0.clone())
            .looped()
            .with_volume(audio_channels.bgm.0);
    }
}
