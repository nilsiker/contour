use bevy::prelude::*;

use self::cursor::CursorPlugin;

mod cursor;

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_plugin(CursorPlugin);
    }
}

enum Interactable {
    Item,
    Info,
}

struct Interactor {
    current_interactable: Interactable,
}




