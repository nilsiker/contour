use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio, AudioSource};

use crate::ui::options_menu::AudioConfig;

pub struct SfxPlayEvent(Handle<AudioSource>);
pub struct SFXPlugin;
impl Plugin for SFXPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SfxPlayEvent>().add_system(handle_play_sfx);
    }
}

fn handle_play_sfx(
    mut sfx_play_events: EventReader<SfxPlayEvent>,
    audio: Res<Audio>,
    config: Res<AudioConfig>,
) {
    for event in sfx_play_events.iter() {
        audio.play(event.0.clone()).looped().with_volume(config.sfx);
    }
}
