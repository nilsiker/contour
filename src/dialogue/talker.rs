use bevy::prelude::*;

use super::DialogueNode;

pub struct TalkerPlugin;
impl Plugin for TalkerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

pub struct Talker(DialogueNode);

