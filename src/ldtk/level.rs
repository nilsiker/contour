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
                    load_level_neighbors: false,
                },
                int_grid_rendering: IntGridRendering::Invisible,
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .insert_resource(LevelSelection::Index(0))
            .add_system(add_name_on_ldtk_layers)
            .add_system(add_name_on_ldtk_tiles)
            .add_system(add_name_on_ldtk_levels);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    asset_server.watch_for_changes().unwrap();
    commands
        .spawn_bundle(LdtkWorldBundle {
            ldtk_handle: asset_server.load("levels/contour.ldtk"),
            ..Default::default()
        })
        .insert(Name::new("World"));
    bevy::log::info!("Map loaded.");

    commands.insert_resource(NextState(GameState::InGame));
}

fn add_name_on_ldtk_levels(
    mut commands: Commands,
    assets: Res<Assets<LdtkLevel>>,
    tiles: Query<(Entity, &Handle<LdtkLevel>), Added<Handle<LdtkLevel>>>,
) {
    tiles.for_each(|(entity, handle)| {
        let asset = assets.get(handle).unwrap();
        commands
            .entity(entity)
            .insert(Name::new(format!("{}", asset.level.identifier)));
    })
}

fn add_name_on_ldtk_layers(
    mut commands: Commands,
    layers: Query<(Entity, &LayerMetadata), Added<LayerMetadata>>,
) {
    layers.for_each(|(entity, metadata)| {
        commands
            .entity(entity)
            .insert(Name::new(metadata.identifier.clone()));
    });
}

fn add_name_on_ldtk_tiles(
    mut commands: Commands,
    tiles: Query<(Entity, &TilePos, &TileMetadata), Added<TilePos>>,
) {
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
