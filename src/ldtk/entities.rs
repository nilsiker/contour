use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

use crate::{
    animation::Animations,
    assets::paths,
    pawn::{
        player::{Lantern, Player},
        MoveDirection, Speed,
    },
    rendering::{
        layers::{Layer, PAWN_LAYER},
        YSort,
    },
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
    layer: Layer,
    player: Player,
    move_direction: MoveDirection,
    speed: Speed,
    animations: Animations,
    lantern: Lantern,
}
impl LdtkEntity for Player2 {
    fn bundle_entity(
        _: &EntityInstance,
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
            collider_bundle: ColliderBundle {
                collider: CollisionShape::Sphere { radius: 4.0 },
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: RotationConstraints::lock(),
                physic_material: PhysicMaterial {   // TODO  this hardly works, include some drag-system...
                    restitution: 0.0,
                    density: 50.0,
                    friction: 1.0,
                },
                ..default()
            },
            name: "Player".into(),
            y_sort: YSort,
            layer: Layer(PAWN_LAYER),
            speed: Speed(20.0),
            player: Player,
            move_direction: MoveDirection(Vec2::default()),
            animations: Animations::from_file("player.ron"),
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
    layer: Layer,
    name: Name,
}
