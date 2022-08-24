use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, egui::Vec2};

use crate::{character::GameOver, player::PlayerThoughtTimer};

#[derive(Component, Inspectable)]
struct Score(f32);

#[derive(Component, Inspectable)]
pub struct MainText(pub &'static str);

pub struct TextPlugin;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut egui_context: ResMut<EguiContext>,
) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([TextSection::from_style(TextStyle {
                font: asset_server.load("PressStart2P.ttf"),
                font_size: 40.0,
                color: Color::GOLD,
                ..default()
            })])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                align_self: AlignSelf::FlexEnd,
                ..default()
            })
            .with_text_alignment(TextAlignment::TOP_LEFT),
        )
        .insert(Score(0.0));

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

fn ui_example(mut egui_context: ResMut<EguiContext>, query: Query<&MainText>) {
    for thought in &query {
        egui::Area::new("")
            .anchor(egui::Align2::CENTER_CENTER, (0., -100.))
            .show(egui_context.ctx_mut(), |ui| {
                ui.colored_label(egui::Color32::WHITE, thought.0);
            });
    }
}

fn update_score_by_time(
    mut query: Query<&mut Score>,
    game_over_query: Query<&GameOver>,
    time: Res<Time>,
) {
    for mut score in &mut query {
        if let Ok(game_over) = game_over_query.get_single() {
            if !game_over.0 {
                score.0 += time.delta_seconds();
            }
        }
    }
}

fn update_score_text(mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in &mut query {
        text.sections[0].value = format!("{}", score.0.round());
    }
}

fn start_player_thought_timer(mut query: Query<&mut PlayerThoughtTimer, Changed<MainText>>) {
    for mut timer in &mut query {
        timer.0.unpause();
        timer.0.reset();
    }
}

fn clear_player_thought(
    time: Res<Time>,
    mut query: Query<(&mut MainText, &mut PlayerThoughtTimer)>,
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

fn debug_input_player_thoughts(input: Res<Input<KeyCode>>, mut query: Query<&mut MainText>) {
    if input.just_pressed(KeyCode::T) {
        query.single_mut().0 = "This is a test thought, hmmm....";
    }
}

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(start_player_thought_timer)
            .add_system(clear_player_thought)
            .add_system(debug_input_player_thoughts)
            .add_system(ui_example)
            .add_system(update_score_by_time)
            .add_system(update_score_text)
            .register_inspectable::<Score>();
    }
}
