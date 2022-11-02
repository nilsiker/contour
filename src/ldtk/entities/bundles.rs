use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::Animations,
    assets::paths,
    ldtk::PhysicsBundle,
    pawn::{
        player::{Lantern, Player},
        MoveDirection, Speed,
    },
    rendering::YSort,
};

use super::components::*;

#[derive(Clone, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[bundle]
    collider_bundle: PhysicsBundle,
    name: Name,
    y_sort: YSort,
    player: Player,
    move_direction: MoveDirection,
    speed: Speed,
    animations: Animations,
    lantern: Lantern,
    worldly: Worldly,
}
impl LdtkEntity for PlayerBundle {
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
            collider_bundle: PhysicsBundle::from(entity.clone()),
            name: "Player".into(),
            speed: Speed(20.0),
            animations: Animations::from_file("player.ron"),
            move_direction: MoveDirection(Vec2::default()),
            player: Player,
            y_sort: YSort::default(),
            lantern: Lantern(false),
            worldly: Worldly {
                entity_iid: entity.iid.clone(),
            },
        }
    }
}

#[derive(Bundle, LdtkEntity)]
pub struct ContainerBundle {
    #[from_entity_instance]
    sprite_offset: SpriteOffset,
    #[from_entity_instance]
    named: NamedEntity,
    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    physics_bundle: PhysicsBundle, // TODO allow for an offset  child with transform, and collider attached...?
    y_sort: YSort,
    name: Name,
}

#[derive(Bundle, LdtkEntity)]
pub struct InfoBundle {
    #[from_entity_instance]
    sprite_offset: SpriteOffset,
    #[from_entity_instance]
    named: NamedEntity,
    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    physics_bundle: PhysicsBundle,
    name: Name,
    #[from_entity_instance]
    info: Info,
    ysort: YSort,
}

#[derive(Bundle, LdtkEntity)]
pub struct NpcBundle {
    #[from_entity_instance]
    sprite_offset: SpriteOffset,
    #[from_entity_instance]
    named: NamedEntity,
    #[bundle]
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    physics_bundle: PhysicsBundle,
    y_sort: YSort,
    name: Name,
}
