use bevy::prelude::*;
use bevy_egui::{egui::FontTweak, EguiContext};

use crate::{assets::bit_font_bytes, consts::path};

use self::{main_menu::MainMenuPlugin, options_menu::OptionsMenuPlugin};

mod styling;
mod main_menu;
pub mod options_menu;

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
        app.insert_resource(UiData::default())
            .add_startup_system(setup);
    }
}

#[derive(Component)]
pub struct UI;

pub struct UiData {
    font: Handle<Font>,
}
impl Default for UiData {
    fn default() -> Self {
        Self {
            font: Default::default(),
        }
    }
}

pub fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut ui_data: ResMut<UiData>,
    mut egui: ResMut<EguiContext>,
) {
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

    ui_data.font = assets.load(path::FONT_BIT);
    let mut ui = commands.spawn_bundle(NodeBundle {
        color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
        style: Style {
            flex_direction: FlexDirection::ColumnReverse,
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            margin: UiRect::all(Val::Auto),
            ..default()
        },
        ..default()
    });
    ui.insert(Name::new("UI"));
    ui.insert(UI);
}
