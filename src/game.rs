use bevy::prelude::PluginGroup;

use crate::{
    animation::AnimPlugin, audio::AudioPlugin, config::ConfigPlugin, dialogue::DialoguePlugin,
    pawn::PawnPlugin, rendering::RenderingPlugin, state::StatePlugin, ui::UiPlugin, level::LevelPlugin,
};

pub struct ContourPlugins;
impl PluginGroup for ContourPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(StatePlugin)
            .add(ConfigPlugin)
            .add(RenderingPlugin)
            .add(PawnPlugin)
            .add(AudioPlugin)
            .add(UiPlugin)
            .add(DialoguePlugin)
            .add(AnimPlugin)
            .add(LevelPlugin);
    }
}
