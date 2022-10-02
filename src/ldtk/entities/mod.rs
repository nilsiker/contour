mod bundles;
mod components;

use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;

use self::{bundles::*, components::*};

pub struct EntitiesPlugin;
impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_ldtk_entity::<ContainerBundle>("Container")
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<InfoBundle>("Info")
            .add_system(set_entity_names)
            .add_system(set_sprite_anchor);

        // #[cfg(debug_assertions)]
        // app.register_inspectable::<....>();
    }
}

fn set_entity_names(mut query: Query<(&mut Name, &NamedEntity), Added<NamedEntity>>) {
    for (mut name, named) in &mut query {
        name.set(named.0.clone());
    }
}

fn set_sprite_anchor(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteOffset), Added<TextureAtlasSprite>>,
) {
    for (mut sprite, offset) in &mut query {
        sprite.anchor = Anchor::Custom(offset.0);
    }
}
