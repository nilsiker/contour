use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::Inspectable;
use heron::CollisionEvent;

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
            .register_ldtk_entity::<LevelGateBundle>("LevelGate")
            .add_system(set_entity_names)
            .add_system(set_sprite_anchor)
            .add_system(register_level_gate_collisions);
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

#[derive(Component, Default)]
pub struct LevelGate(usize);
impl From<EntityInstance> for LevelGate {
    fn from(entity: EntityInstance) -> Self {
        Self(match entity.get_field_value("Level") {
            Some(field_value) => {
                if let FieldValue::Int(option) = field_value {
                    if let Some(level_index) = option {
                        level_index as usize
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
            None => 0,
        })
    }
}

#[derive(Bundle, LdtkEntity)]
struct LevelGateBundle {
    #[from_entity_instance]
    level_index: LevelGate,
    #[from_entity_instance]
    #[bundle]
    pub physics_bundle: PhysicsBundle,
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

fn register_level_gate_collisions(
    mut players: Query<Entity, With<Player>>,
    mut gates: Query<(Entity, &LevelGate)>,
    mut current_level: ResMut<LevelSelection>,
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(data1, data2) = event {
            if let Ok(_) = players.get_single_mut() {
                let mut gates = gates.iter_mut().filter(|(g, _)| {
                    g == &data1.rigid_body_entity() || g == &data2.rigid_body_entity()
                });

                if let Some((_, gate)) = gates.next() {
                    match *current_level {
                        LevelSelection::Index(index) => {
                            if index != gate.0 {
                                *current_level = LevelSelection::Index(gate.0);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
