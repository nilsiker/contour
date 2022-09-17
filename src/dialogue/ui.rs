#![allow(dead_code)]
use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{
    egui::{epaint::Shadow, style::Margin, Color32, Frame, Pos2, Rect, Rounding, Vec2},
    EguiContext,
};

use crate::ui::{styling::MENU_FILL, text::h2};

use super::{DialogueChangeEvent, DialogueNode};

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
        .add_system(show_dialogue)
        .add_system(set_timer_state_on_dialogue_change)
        .add_system(tick_dialogue_timer)
        .add_system(animate_dialogue_text)
        .add_system(step_through_dialogue);
    }
}

struct DialogueTextTimer(Timer);

fn step_through_dialogue(mut dialogue: ResMut<DialogueNode>, input: Res<Input<KeyCode>>) {
    if let DialogueNode::Line(data) = dialogue.as_mut() {
        if input.just_released(KeyCode::Space) {
            if data.progress < data.text.len() {
                data.progress = data.text.len();
            } else {
                *dialogue = *data.next.clone();
            }
        }
    }
}

fn set_timer_state_on_dialogue_change(
    mut timer: ResMut<DialogueTextTimer>,
    mut events: EventReader<DialogueChangeEvent>,
) {
    for event in events.iter() {
        {
            match event.0 {
                DialogueNode::None => timer.0.pause(),
                _ => {
                    timer.0.reset();
                    timer.0.unpause();
                }
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

fn animate_dialogue_text(mut dialogue: ResMut<DialogueNode>, timer: Res<DialogueTextTimer>) {
    if let DialogueNode::Line(data) = dialogue.as_mut() {
        if timer.0.just_finished() && data.progress < data.text.len() {
            data.progress += 1;
        }
    }
}

fn show_dialogue(
    mut egui: ResMut<EguiContext>,
    mut dialogue: ResMut<DialogueNode>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let center: Pos2 = (
        window.width() / 2.0 - INNER_MARGIN_X,
        window.height() - (DIALOGUE_HEIGHT / 2.0),
    )
        .into();
    let size: Vec2 = (window.width() - (2.0 * INNER_MARGIN_X), DIALOGUE_HEIGHT).into();
    match dialogue.as_mut() {
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
                            let sliced_text = &data.text[..data.progress];
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
