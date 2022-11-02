#![allow(dead_code)]
use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{
    egui::{epaint::Shadow, style::Margin, Color32, Frame, Pos2, Rect, Rounding, Vec2},
    EguiContext,
};
use iyes_loopless::state::NextState;

use crate::{
    state::GameState,
    ui::{styling::MENU_FILL, text::h2},
};

use super::DialogueNode;

const DIALOGUE_HEIGHT: f32 = 200.0;
const OUTER_MARGIN_X: f32 = 40.0;
const OUTER_MARGIN_Y: f32 = 40.0;
const INNER_MARGIN_X: f32 = 40.0;
const INNER_MARGIN_Y: f32 = 30.0;

pub struct DialogueUiPlugin;
impl Plugin for DialogueUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DialogueTextTimer(Timer::new(
            Duration::from_millis(100),
            true,
        )))
        .insert_resource(DialogueProgress(0))
        .add_system(show_dialogue)
        .add_system(set_game_state_on_dialogue_change)
        .add_system(set_timer_state_on_dialogue_change)
        .add_system(tick_dialogue_timer)
        .add_system(animate_dialogue_text)
        .add_system(step_through_dialogue);
    }
}

struct DialogueTextTimer(Timer);

#[derive(Clone, Debug)]
struct DialogueProgress(usize);

fn step_through_dialogue(
    mut progress: ResMut<DialogueProgress>,
    mut dialogue: ResMut<DialogueNode>,
    dialogues: Res<HashMap<i32, DialogueNode>>,
    input: Res<Input<KeyCode>>,
) {
    if let DialogueNode::Line(data) = dialogue.as_ref() {
        if input.just_released(KeyCode::Space) {
            if progress.0 < data.text.len() {
                progress.0 = data.text.len();
            } else {
                match data.next {
                    super::NextDialogue::None => *dialogue = DialogueNode::None,
                    super::NextDialogue::Id(id) => {
                        *dialogue = match dialogues.get(&id) {
                            Some(dialogue) => dialogue.clone(),
                            None => DialogueNode::None,
                        }
                    }
                }

                progress.0 = 0;
            }
        }
    }
}

fn set_game_state_on_dialogue_change(mut commands: Commands, dialogue: Res<DialogueNode>) {
    if dialogue.is_changed() {
        commands.insert_resource(NextState(match *dialogue {
            DialogueNode::None => GameState::InGame,
            _ => GameState::UI,
        }));
    }
}

fn set_timer_state_on_dialogue_change(
    dialogue: Res<DialogueNode>,
    mut timer: ResMut<DialogueTextTimer>,
) {
    if dialogue.is_changed() {
        bevy::log::info!("{:?}", dialogue);

        match *dialogue {
            DialogueNode::None => timer.0.pause(),
            _ => {
                timer.0.reset();
                timer.0.unpause();
            }
        }
    }
}

fn tick_dialogue_timer(mut timer: ResMut<DialogueTextTimer>, time: Res<Time>) {
    if timer.0.just_finished() {
        timer.0.reset();
    } else {
        timer.0.tick(time.delta());
    }
}

fn animate_dialogue_text(
    dialogue: Res<DialogueNode>,
    mut progress: ResMut<DialogueProgress>,
    timer: Res<DialogueTextTimer>,
) {
    if let DialogueNode::Line(data) = dialogue.clone() {
        if timer.0.just_finished() && progress.0 < data.text.len() {
            progress.0 += 1;
        }
    }
}

fn show_dialogue(
    mut egui: ResMut<EguiContext>,
    dialogue: Res<DialogueNode>,
    progress: ResMut<DialogueProgress>,
    windows: Res<Windows>,
) {
    if let Some(window) = windows.get_primary() {
        let center: Pos2 = (
            window.width() / 2.0 - INNER_MARGIN_X,
            window.height() - (DIALOGUE_HEIGHT / 2.0),
        )
            .into();
        let size: Vec2 = (window.width() - (2.0 * INNER_MARGIN_X), DIALOGUE_HEIGHT).into();
        match dialogue.clone() {
            DialogueNode::Line(data) => {
                bevy_egui::egui::Window::new("DialogueWindow")
                    .title_bar(false)
                    .fixed_rect(Rect::from_center_size(center, size))
                    .frame(Frame {
                        inner_margin: Margin::symmetric(INNER_MARGIN_X, INNER_MARGIN_Y),
                        rounding: Rounding::same(0.0),
                        fill: MENU_FILL,
                        shadow: Shadow {
                            extrusion: 30.0,
                            color: Color32::BLACK,
                        },
                        ..default()
                    })
                    .show(egui.ctx_mut(), |ui| {
                        ui.horizontal_centered(|ui| {
                            ui.vertical(|ui| {
                                let sliced_text = &data.text[..progress.0];
                                ui.label(h2(sliced_text));
                            });
                        });
                        ui.allocate_space(ui.available_size());
                    });
            }
            DialogueNode::Choice(_) => {
                bevy::log::warn!(
                    "And here you would be presented with a few choices, IF I HAD IMPLEMENTED IT!"
                )
            }
            _ => (),
        }
    }
}
