use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio, AudioSource};

use super::{AudioChannels};

pub struct SFXPlayEvent(Handle<AudioSource>);

pub struct SFXPlugin;
impl Plugin for SFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SFXPlayEvent>()
            .add_system(handle_sfx_music_changed);
    }
}

pub fn handle_sfx_music_changed(
    mut sfx_play_events: EventReader<SFXPlayEvent>,
    audio: Res<Audio>,
    volumes: Res<AudioChannels>,
) {
    for event in sfx_play_events.iter() {
        audio
            .play(event.0.clone())
            .looped()
            .with_volume(volumes.sfx.0);
    }
}
