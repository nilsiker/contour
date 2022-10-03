mod transition;

use bevy::{prelude::*, utils::HashMap};
use bevy_ecs_ldtk::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_inspector_egui::Inspectable;
use heron::CollisionEvent;
use iyes_loopless::state::NextState;

use crate::{pawn::player::Player, physics::TupleUtil, state::GameState};

use self::transition::{LevelTransitionPlugin, StartTransition};

use super::{utils::FieldReturn, PhysicsBundle};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(LdtkPlugin)
            .add_plugin(LevelTransitionPlugin)
            .add_startup_system(setup)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
                int_grid_rendering: IntGridRendering::Invisible,
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .insert_resource(LevelSelection::Index(0))
            .insert_resource(LastLevelIndex(0))
            .register_ldtk_entity::<ExitBundle>("Gate")
            .add_system(add_name_on_ldtk_layers)
            .add_system(add_name_on_ldtk_tiles)
            .add_system(add_name_on_ldtk_levels)
            .add_system(register_level_gate_collisions);
    }
}

pub struct LastLevelIndex(pub usize);

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
            .insert(Name::new(asset.level.identifier.clone()));
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

#[derive(Component, Inspectable, Debug)]
pub enum Gate {
    Exit(usize),
    Entry(usize),
}
impl Default for Gate {
    fn default() -> Self {
        Self::Exit(0)
    }
}
impl From<EntityInstance> for Gate {
    fn from(entity: EntityInstance) -> Self {
        let index = if let Some(FieldValue::Int(Some(level_index))) = entity.get_field_value("to") {
            level_index as usize
        } else {
            0
        };

        match entity.get_string_value("Gate") {
            Some(value) => {
                if value == "Entry" {
                    Gate::Entry(index)
                } else {
                    Gate::Exit(index)
                }
            }
            None => Gate::Exit(index),
        }
    }
}

#[derive(Bundle, LdtkEntity)]
struct ExitBundle {
    #[from_entity_instance]
    level_index: Gate,
    #[from_entity_instance]
    #[bundle]
    pub physics_bundle: PhysicsBundle,
}

#[derive(Bundle, LdtkEntity)]
struct EntryBundle {
    #[from_entity_instance]
    level_index: Gate,
    #[from_entity_instance]
    #[bundle]
    pub physics_bundle: PhysicsBundle,
}

fn register_level_gate_collisions(
    players: Query<Entity, With<Player>>,
    mut gates: Query<(Entity, &Gate)>,
    level_selection: ResMut<LevelSelection>,
    mut level_transitions: EventWriter<StartTransition>,

    mut events: EventReader<CollisionEvent>,
) {
    let player = match players.get_single() {
        Ok(player) => player,
        Err(_) => return,
    };

    for event in events
        .iter()
        .filter(|ev| ev.collision_shape_entities().any(player))
    {
        if let CollisionEvent::Started(data1, data2) = event {
            let mut gates = gates.iter_mut().filter(|(g, _)| {
                g == &data1.rigid_body_entity() || g == &data2.rigid_body_entity()
            });

            if let Some((_, gate)) = gates.next() {
                if let LevelSelection::Index(current_index) = *level_selection {
                    if let Gate::Exit(next_index) = gate {
                        if current_index != *next_index {
                            level_transitions.send(StartTransition {
                                from: current_index,
                                to: *next_index,
                            });
                        }
                    }
                }
            }
        }
    }
}
