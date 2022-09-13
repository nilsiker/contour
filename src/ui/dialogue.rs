#![allow(dead_code)]
use bevy::prelude::*;

#[derive(Component)]
enum Dialogue {
    None,
    Text(DialogueData),
    Choices(Vec<DialogueData>),
}

struct DialogueData {
    text: String,
    event: Option<DialogueEvent>,
    next: Option<Box<Dialogue>>,
}

enum DialogueEvent {
    ItemReceived(Item),
    ItemLost(Item),
}

struct Item;

pub struct DialoguePlugin;
impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_dialogue);
    }
}

fn show_dialogue(dialogue: Res<Dialogue>) {
    match *dialogue {
        Dialogue::Text(_) => bevy::log::info!("Text for a dialogue goes here"),
        Dialogue::Choices(_) => bevy::log::info!("And here you would be presented with a few choices!"),
        _ => ()
    }
}


