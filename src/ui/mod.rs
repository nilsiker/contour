use bevy::prelude::*;
use bevy_egui::{egui::FontTweak, EguiContext};

use crate::assets::bit_font_bytes;

use self::{main_menu::MainMenuPlugin, options_menu::OptionsMenuPlugin};

mod dialogue;
mod main_menu;
pub mod options_menu;
mod styling;
mod text;

pub struct ContourUiPlugins;
impl PluginGroup for ContourUiPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group
            .add(CoreUiPlugin)
            .add(MainMenuPlugin)
            .add(OptionsMenuPlugin);
    }
}

struct CoreUiPlugin;
impl Plugin for CoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_font);
    }
}

#[derive(Component)]
pub struct UI;

pub fn initialize_font(mut egui: ResMut<EguiContext>) {
    let mut fonts = bevy_egui::egui::FontDefinitions::default();
    fonts.font_data.insert(
        "Bit".to_owned(),
        bevy_egui::egui::FontData::from_owned(bit_font_bytes()).tweak(FontTweak {
            y_offset_factor: 0.1,
            ..Default::default()
        }),
    );

    fonts.families.insert(
        bevy_egui::egui::FontFamily::Name("Bit".into()),
        vec!["Bit".to_owned()],
    );

    fonts
        .families
        .get_mut(&bevy_egui::egui::FontFamily::Proportional)
        .unwrap() //it works
        .insert(0, "Bit".to_owned());

    fonts
        .families
        .get_mut(&bevy_egui::egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "Bit".to_owned()); //.push("Helvetica".to_owned());

    egui.ctx_mut().set_fonts(fonts);
}
