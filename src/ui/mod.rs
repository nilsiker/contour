use bevy::prelude::PluginGroup;

use self::{
    releasable_action_buttons::ReleaseActionButtonPlugin, start_menu::MainMenuPlugin,
    styled_buttons::InteractionStyledButtonsPlugin,
};

mod releasable_action_buttons;
mod start_menu;
mod styled_buttons;

pub struct ContourUiPlugins;
impl PluginGroup for ContourUiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(MainMenuPlugin)
            .add(ReleaseActionButtonPlugin)
            .add(InteractionStyledButtonsPlugin);
    }
}

