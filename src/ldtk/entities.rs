use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{
    animation::Animations,
    assets::paths,
    pawn::{
        player::{Lantern, Player},
        MoveDirection, Speed,
    },
    rendering::YSort,
};

use super::{utils::FieldReturner, PhysicsBundle};

pub struct EntitiesPlugin;
impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_ldtk_entity::<ContainerBundle>("Container")
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<InfoBundle>("Info")
            .add_system(set_entity_names)
            .add_system(set_sprite_anchor);
    }
}

#[derive(Component)]
pub struct Info(pub String);
impl From<EntityInstance> for Info {
    fn from(entity: EntityInstance) -> Self {
        Self(match entity.get_field_value("text") {
            Some(field_value) => {
                if let FieldValue::String(option) = field_value {
                    if let Some(text) = option {
                        text
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
            None => String::new(),
        })
    }
}

#[derive(Clone, Bundle)]
struct PlayerBundle {
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[bundle]
    pub collider_bundle: PhysicsBundle,
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
struct ContainerBundle {
    #[from_entity_instance]
    pub sprite_offset: SpriteOffset,
    #[from_entity_instance]
    named: NamedEntity,
    #[bundle]
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub physics_bundle: PhysicsBundle, // TODO allow for an offset  child with transform, and collider attached...?
    y_sort: YSort,
    name: Name,
}

#[derive(Bundle, LdtkEntity)]
struct InfoBundle {
    #[from_entity_instance]
    pub sprite_offset: SpriteOffset,
    #[from_entity_instance]
    named: NamedEntity,
    #[bundle]
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub physics_bundle: PhysicsBundle,
    name: Name,
    #[from_entity_instance]
    info: Info,
    ysort: YSort,

}

#[derive(Bundle, LdtkEntity)]
struct Wardrobe {
    #[from_entity_instance]
    pub sprite_offset: SpriteOffset,
    #[from_entity_instance]
    named: NamedEntity,
    #[bundle]
    #[sprite_sheet_bundle]
    pub sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub physics_bundle: PhysicsBundle, // TODO allow for an offset  child with transform, and collider attached...?
    y_sort: YSort,
    name: Name,
}

#[derive(Component, Clone, Inspectable)]
pub struct SpriteOffset(Vec2);
impl From<EntityInstance> for SpriteOffset {
    fn from(entity: EntityInstance) -> Self {
        let offset = match entity
            .field_instances
            .iter()
            .find(|field| field.identifier == "sprite_offset")
        {
            Some(field) => {
                if let FieldValue::String(Some(offset_string)) = &field.value {
                    match ron::from_str::<Vec2>(offset_string) {
                        Ok(offset) => offset,
                        Err(_) => Vec2::default(),
                    }
                } else {
                    Vec2::default()
                }
            }
            None => Vec2::default(),
        };
        Self(offset)
    }
}

#[derive(Component, Clone)]
struct NamedEntity(String);
impl From<EntityInstance> for NamedEntity {
    fn from(entity: EntityInstance) -> Self {
        Self(entity.identifier)
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

