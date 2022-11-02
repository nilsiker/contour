#![allow(dead_code, unused_variables)]
use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use serde::Deserialize;

use self::ui::DialogueUiPlugin;
mod talker;
mod ui;

pub struct DialoguePlugin;
impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        let map: HashMap<i32, DialogueNode> = HashMap::new();
        let dialogue = DialogueNode::None;
        app.insert_resource(dialogue)
            .insert_resource(map)
            .add_startup_system(setup.chain(add_dummy_dialogue))
            .add_plugin(DialogueUiPlugin);
    }
}

fn setup(assets: Res<AssetServer>, mut dialogue_nodes: ResMut<HashMap<i32, DialogueNode>>) {
    let work_dir = std::env::current_dir().unwrap();
    let path = work_dir.join("assets").join("dialogues.ron");
    dbg!(&path);
    let file = std::fs::read(path).expect("path to dialogues ron file");

    let dialogues: HashMap<i32, DialogueNode> = match ron::de::from_bytes(&file[..]) {
        Ok(map) => map,
        Err(e) => panic!("{e}"),
    };
    *dialogue_nodes = dialogues;
}

fn add_dummy_dialogue(
    dialogue_nodes: Res<HashMap<i32, DialogueNode>>,
    mut dialogue: ResMut<DialogueNode>,
) {
    *dialogue = match dialogue_nodes.get(&0) {
        Some(dialogue) => dialogue.clone(),
        None => DialogueNode::None,
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum DialogueNode {
    None,
    Line(DialogueData),
    Choice(Vec<DialogueData>),
}

#[derive(Deserialize, Clone, Debug)]
enum NextDialogue {
    None,
    Id(i32),
}

#[derive(Deserialize, Clone, Debug)]
pub struct DialogueData {
    text: String,
    event: Option<DialogueEvent>,
    next: NextDialogue,
}

#[derive(Deserialize, Clone, Debug)]
enum DialogueEvent {
    ItemReceived(Item),
    ItemLost(Item),
}

#[derive(Deserialize, Clone, Debug)]
struct Item;
