use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio};

use crate::{
    camera::follow_camera::FollowCameraPlugin,
    character::{enemy::EnemyPlugin, player::PlayerPlugin, CharacterPlugin},
    lighting::LightingPlugin,
    text::TextPlugin,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Prelude,
    InGame,
    GameOver,
}

pub struct ContourPlugins;

impl PluginGroup for ContourPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(CharacterPlugin)
            .add(PlayerPlugin)
            .add(FollowCameraPlugin)
            .add(EnemyPlugin)
            .add(LightingPlugin)
            .add(TextPlugin);
    }
}

pub fn play_audio_system(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("bgm.wav"))
        .looped()
        .with_volume(0.75);
}
