use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::player::{PlayerThought, PlayerThoughtTimer};

#[derive(Component, Inspectable)]
struct FpsText;

pub struct TextPlugin;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut egui_context: ResMut<EguiContext>,
) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        color: Color::WHITE,
                        font: asset_server.load("PressStart2P.ttf"),
                        font_size: 20.0,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("PressStart2P.ttf"),
                    font_size: 20.0,
                    color: Color::GOLD,
                    ..default()
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexEnd,
                ..default()
            })
            .with_text_alignment(TextAlignment::TOP_LEFT),
        )
        .insert(FpsText);

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "press_start".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/PressStart2P.ttf")),
    );

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "press_start".to_owned());
    egui_context.ctx_mut().set_fonts(fonts);
}

fn ui_example(mut egui_context: ResMut<EguiContext>, query: Query<&PlayerThought>) {
    for thought in &query {
        egui::Area::new("")
            .anchor(egui::Align2::CENTER_CENTER, (0., -100.))
            .show(egui_context.ctx_mut(), |ui| {
                ui.colored_label(egui::Color32::WHITE, thought.0);
            });
    }
}

fn fps_text(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{average:.2}");
            }
        }
    }
}

fn start_player_thought_timer(mut query: Query<&mut PlayerThoughtTimer, Changed<PlayerThought>>) {
    for mut timer in &mut query {
        timer.0.unpause();
        timer.0.reset();
    }
}

fn clear_player_thought(
    time: Res<Time>,
    mut query: Query<(&mut PlayerThought, &mut PlayerThoughtTimer)>,
) {
    for (mut thought, mut timer) in &mut query {
        if timer.0.just_finished() {
            thought.0 = "";
            timer.0.reset();
        } else {
            timer.0.tick(time.delta());
        }
    }
}

fn debug_input_player_thoughts(input: Res<Input<KeyCode>>, mut query: Query<&mut PlayerThought>) {
    if input.just_pressed(KeyCode::T) {
        query.single_mut().0 = "This is a test thought, hmmm....";
    }
}

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(fps_text)
            .add_system(start_player_thought_timer)
            .add_system(clear_player_thought)
            .add_system(debug_input_player_thoughts)
            .add_system(ui_example)
            .register_inspectable::<FpsText>();
    }
}