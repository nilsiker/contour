use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    character::{
        player::{Lantern, ScreenTextTimer},
        GameOver,
    },
    game::GameState,
};

#[derive(Component)]
pub struct Score(pub f32);

#[derive(Component)]
pub struct MainText(pub String);
#[derive(Component)]
pub struct SubText(pub String);

pub struct ExpositionText {
    pub shown: bool,
    pub main: &'static str,
    pub sub: &'static str,
}
pub struct ExpositionTexts {
    pub inner: [ExpositionText; 4],
}

const EXPOSITION_TEXTS: [ExpositionText; 4] = [
    ExpositionText {
        shown: false,
        main: "CONTOUR",
        sub: "<space>",
    },
    ExpositionText {
        shown: false,
        main: "They are stronger together, and that is meant literally.",
        sub: "<g>",
    },
    ExpositionText {
        shown: false,
        main: "Your light will slow them down.",
        sub: "<f>",
    },
    ExpositionText {
        shown: false,
        main: "But in time, they will consume you.",
        sub: "<wasd>",
    },
];

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ExpositionTexts {
            inner: EXPOSITION_TEXTS,
        })
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_update(GameState::Prelude)
                .with_system(show_exposition_texts)
                .with_system(step_through_prelude),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(start_screen_text_timer)
                .with_system(clear_screen_text)
                .with_system(update_score_by_time)
                .with_system(update_score_text),
        )
        .add_system(show_center_text);
    }
}

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

fn show_center_text(
    state: Res<State<GameState>>,
    mut egui_context: ResMut<EguiContext>,
    query: Query<(&MainText, &SubText)>,
) {
    for (line1, line2) in &query {
        egui::Area::new("")
            .anchor(egui::Align2::CENTER_CENTER, (0., -100.))
            .show(egui_context.ctx_mut(), |ui| {
                ui.colored_label(egui::Color32::WHITE, line1.0.to_owned());
            });

        egui::Area::new("score")
            .anchor(egui::Align2::CENTER_CENTER, (0., -50.))
            .show(egui_context.ctx_mut(), |ui| {
                if let GameState::GameOver = state.current() {
                    ui.colored_label(egui::Color32::WHITE, line2.0.to_owned());
                }
            });

        egui::Area::new("input")
            .anchor(egui::Align2::CENTER_CENTER, (0., 50.))
            .show(egui_context.ctx_mut(), |ui| match state.current() {
                GameState::GameOver => {
                    ui.colored_label(egui::Color32::WHITE, "<refresh page to restart>".to_owned())
                }
                _ => ui.colored_label(egui::Color32::WHITE, line2.0.to_owned()),
            });
    }
}

fn start_screen_text_timer(mut query: Query<&mut ScreenTextTimer, Changed<MainText>>) {
    for mut timer in &mut query {
        timer.0.unpause();
        timer.0.reset();
    }
}

fn clear_screen_text(
    time: Res<Time>,
    mut query: Query<(&mut MainText, &mut SubText, &mut ScreenTextTimer)>,
) {
    for (mut main, mut sub, mut timer) in &mut query {
        if timer.0.just_finished() {
            main.0 = "".to_owned();
            sub.0 = "".to_owned();
            timer.0.reset();
        } else {
            timer.0.tick(time.delta());
        }
    }
}

fn update_score_by_time(
    mut query: Query<&mut Score>,
    game_over_query: Query<(&Lantern, &GameOver)>,
    time: Res<Time>,
) {
    for mut score in &mut query {
        if let Ok((lantern, game_over)) = game_over_query.get_single() {
            if !game_over.0 {
                score.0 += match lantern.0 {
                    true => time.delta_seconds(),
                    false => time.delta_seconds() * 2.0,
                }
            }
        }
    }
}

fn update_score_text(mut query: Query<(&mut Text, &Score)>) {
    for (mut text, score) in &mut query {
        text.sections[0].value = format!("{}", score.0.round());
    }
}

fn show_exposition_texts(
    texts: ResMut<ExpositionTexts>,
    mut query: Query<(&mut MainText, &mut SubText)>,
) {
    for (mut main, mut sub) in &mut query {
        let next_unshown_text = texts.inner.iter().find(|text| !text.shown);
        match next_unshown_text {
            Some(text) => {
                main.0 = text.main.to_owned();
                sub.0 = text.sub.to_owned();
            }
            None => (),
        }
    }
}

fn step_through_prelude(
    mut state: ResMut<State<GameState>>,
    mut local: Local<usize>,
    input: Res<Input<KeyCode>>,
    mut texts: ResMut<ExpositionTexts>,
) {
    match *local {
        0 => {
            if input.just_pressed(KeyCode::Space) {
                *local = 1;
                texts.inner[0].shown = true;
            }
        }
        1 => {
            if input.just_pressed(KeyCode::G) {
                *local = 2;
                texts.inner[1].shown = true;
            }
        }
        2 => {
            if input.just_pressed(KeyCode::F) {
                *local = 3;
                texts.inner[2].shown = true;
            }
        }
        _ => {
            if input.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
                state
                    .set(GameState::InGame)
                    .expect("State must be added to the game at this point.");
                texts.inner[3].shown = true;
            }
        }
    }
}
