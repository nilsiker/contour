use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use iyes_loopless::state::NextState;

use crate::state::GameState;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(LdtkPlugin)
            .add_startup_system(setup)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..default()
            })
            .insert_resource(LevelSelection::Index(0))
            .add_system(add_name_on_spawn);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
    commands
        .spawn_bundle(LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/typical.ldtk"),

            ..Default::default()
        })
        .insert(Name::new("Tilemap"));
    bevy::log::info!("Map loaded.");

    commands.insert_resource(NextState(GameState::InGame));
}

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

fn add_name_on_spawn(
    mut commands: Commands,
    entities: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    layers: Query<(Entity, &LayerMetadata), Added<LayerMetadata>>,
    tiles: Query<(Entity, &TilePos, &TileMetadata), Added<TilePos>>,
) {
    for (entity, instance) in &entities {
        commands
            .entity(entity)
            .insert(Name::new(instance.identifier.clone()));
    }

    layers.for_each(|(entity, layer)| {
        commands
            .entity(entity)
            .insert(Name::new(layer.identifier.clone()));
    });

    tiles.for_each(|(entity, tile, metadata)| {
        commands
            .entity(entity)
            .insert(Name::new(format!("{},{}", tile.x, tile.y)));

        let metadata_map: HashMap<String, String> =
            ron::de::from_str(metadata.data.as_str()).unwrap();
        match metadata_map.get("name") {
            Some(value) => bevy::log::info!("{value}"),
            None => (),
        }
    })
}
