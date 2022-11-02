use bevy::prelude::*;

use crate::{pawn::player::Player, dialogue::DialogueNode};

use self::cursor::CursorPlugin;

mod cursor;

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(CursorPlugin)
            .add_system(init_player_as_interactor)
            .add_system(interact_input);
    }
}

fn init_player_as_interactor(mut commands: Commands, mut query: Query<Entity, Added<Player>>) {
    if let Ok(player) = query.get_single_mut() {
        commands.entity(player).insert(Interactor::default());
    }
}

fn interact_input(input: Res<Input<MouseButton>>, query: Query<&Interactor>) {
    if input.just_released(MouseButton::Left) {
        match query.get_single() {
            Ok(interactor) => match &interactor.current_interactable {
                Some(interactable) => bevy::log::info!("pressed interact on: {:?}", interactable),
                None => bevy::log::info!("no interactable"),
            },
            Err(e) => bevy::log::error!("{e}"),
        }
    }
}

#[derive(Component, Debug)]
pub enum Interactable {
    Examine(String),
    Talk(DialogueNode),
}

#[derive(Component, Default, Debug)]
pub struct Interactor {
    current_interactable: Option<Interactable>,
}
