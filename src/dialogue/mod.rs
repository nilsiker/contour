#![allow(dead_code, unused_variables)]
use bevy::prelude::*;

use self::ui::DialogueUiPlugin;
mod ui;

pub struct DialoguePlugin;
impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {

        let next_line =  DialogueNode::Line(DialogueData {  // TODO add None when done debugging here
            text: "And here comes the next line".into(),
            progress: 0,
            event: None,
            next: Box::new(DialogueNode::None),
        });

        app.add_event::<DialogueChangeEvent>()
        .insert_resource(DialogueNode::Line(DialogueData {  // TODO add None when done debugging here
            text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent tincidunt congue quam, sed suscipit leo bibendum et. Mauris convallis varius tortor, sed vestibulum ligula efficitur a. Proin dignissim lectus nec dui rutrum, non varius nisl tincidunt. Aliquam justo dolor, consectetur eget feugiat ac, porta ut sem. Vivamus convallis ac nisl in ornare.".into(),
            progress: 0,
            event: None,
            next: Box::new(next_line)
        }))
        .add_system(handle_dialogue_change_events)
        .add_plugin(DialogueUiPlugin);
    }
}

pub struct DialogueChangeEvent(DialogueNode);

fn handle_dialogue_change_events(
    mut dialogue: ResMut<DialogueNode>,
    mut events: EventReader<DialogueChangeEvent>,
) {
    events.iter().for_each(|event| {
        *dialogue = event.0.clone(); // TODO verify that this clone is necessary and without side effects
        bevy::log::info!("dialogue changed");
    })
}

#[derive(Clone)]
pub enum DialogueNode {
    None,
    Line(DialogueData),
    Choice(Vec<DialogueData>),
}

#[derive(Clone)]
pub struct DialogueData {
    text: String,
    progress: usize,
    event: Option<DialogueEvent>,
    next: Box<DialogueNode>,
}

#[derive(Clone)]
enum DialogueEvent {
    ItemReceived(Item),
    ItemLost(Item),
}

#[derive(Clone)]
struct Item;
