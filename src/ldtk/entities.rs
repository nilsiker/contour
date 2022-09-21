use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::Animations,
    assets::paths,
    pawn::{
        player::{Lantern, Player},
        MoveDirection, Speed,
    },
    rendering::YSort,
};

use super::ColliderBundle;

pub struct EntitiesPlugin;
impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_ldtk_entity::<Crate>("Crate");
        app.register_ldtk_entity::<Player2>("Player")
            .add_system(set_sprite_anchor);
    }
}

fn set_sprite_anchor(mut query: Query<&mut TextureAtlasSprite, Added<TextureAtlasSprite>>) {
    for mut sprite in &mut query {
        sprite.anchor = Anchor::BottomCenter;
    }
}

#[derive(Clone, Bundle)]
struct Player2 {
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[bundle]
    pub collider_bundle: ColliderBundle,
    name: Name,
    y_sort: YSort,
    player: Player,
    move_direction: MoveDirection,
    speed: Speed,
    animations: Animations,
    lantern: Lantern,
}
impl LdtkEntity for Player2 {
    fn bundle_entity(
        entity: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let sprite = TextureAtlasSprite {
            anchor: Anchor::BottomCenter,
            ..default()
        };
        let texture_handle = asset_server.load(paths::SPRITE_PLAYER);
        let texture_atlas_base =
            TextureAtlas::from_grid(texture_handle, Vec2::new(16f32, 16f32), 18, 1);
        let texture_atlas = texture_atlases.add(texture_atlas_base);

        Self {
            sprite_bundle: SpriteSheetBundle {
                sprite,
                texture_atlas,
                ..default()
            },
            collider_bundle: ColliderBundle::from(entity.clone()),
            name: "Player".into(),
            speed: Speed(20.0),
            animations: Animations::from_file("player.ron"),
            move_direction: MoveDirection(Vec2::default()),
            player: Player,
            y_sort: YSort::default(),
            lantern: Lantern(false),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
struct Crate {
    #[bundle]
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle, // TODO allow for an offset  child with transform, and collider attached...?
    y_sort: YSort,
    name: Name,
}
