use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio, AudioSource};

use super::Volume;

pub struct BackgroundMusicChangedEvent(Handle<AudioSource>);

pub struct BackgroundMusicPlugin;
impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackgroundMusicChangedEvent>()
            .insert_resource(Volume(50.0))
            .add_system(handle_background_music_changed);
    }
}

pub fn handle_background_music_changed(
    mut background_music_changed_events: EventReader<BackgroundMusicChangedEvent>,
    audio: Res<Audio>,
) {
    for event in background_music_changed_events.iter() {
        if audio.is_playing_sound() {
            audio.stop();
        }

        audio.play(event.0.clone()).looped().with_volume(0.75);
    }
}
