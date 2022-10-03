#![allow(dead_code, unused_variables)]
use bevy::prelude::*;

use self::ui::DialogueUiPlugin;
mod talker;
mod ui;

pub struct DialoguePlugin;
impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        let next_line = DialogueNode::Line(DialogueData {
            // TODO add None when done debugging here
            text: "And here comes the next line".into(),
            event: None,
            next: Box::new(DialogueNode::None),
        });

        app
        .insert_resource(DialogueNode::Line(DialogueData {  // TODO add None when done debugging here
            text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Praesent tincidunt congue quam, sed suscipit leo bibendum et. Mauris convallis varius tortor, sed vestibulum ligula efficitur a. Proin dignissim lectus nec dui rutrum, non varius nisl tincidunt. Aliquam justo dolor, consectetur eget feugiat ac, porta ut sem. Vivamus convallis ac nisl in ornare.".into(),
            event: None,
            next: Box::new(next_line)
        }))
        .add_plugin(DialogueUiPlugin);
    }
}

#[derive(Clone, Debug)]
pub enum DialogueNode {
    None,
    Line(DialogueData),
    Choice(Vec<DialogueData>),
}

#[derive(Clone, Debug)]
pub struct DialogueData {
    text: String,
    event: Option<DialogueEvent>,
    next: Box<DialogueNode>,
}

#[derive(Clone,Debug)]
enum DialogueEvent {
    ItemReceived(Item),
    ItemLost(Item),
}

#[derive(Clone, Debug)]
struct Item;
