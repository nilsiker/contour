use bevy::prelude::*;
use heron::{CollisionEvent, CollisionShape, RigidBody};
use iyes_loopless::{prelude::AppLooplessStateExt, state::CurrentState};

use crate::{physics::TupleUtil, state::GameState};

use super::Interactable;

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(init_cursor_sprite)
            .add_enter_system(GameState::UI, set_cursor_visual_mode)
            .add_exit_system(GameState::UI, set_cursor_visual_mode)
            .add_system(set_cursor_visual_mode)
            .add_system(update_sprite_position)
            .add_system(change_cursor_icon)
            .add_system(handle_cursor_overlap);
    }
}

#[derive(Copy, Clone)]
enum Icon {
    Idle,
    Examine,
    Talk,
}

impl From<Interactable> for Icon {
    fn from(interactable: Interactable) -> Self {
        match interactable {
            Interactable::Examine(_) => Icon::Examine,
            Interactable::Talk(_) => Icon::Talk,
        }
    }
}
impl Default for Icon {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Component)]
struct Cursor(Icon);

fn init_cursor_sprite(
    mut windows: ResMut<Windows>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_cursor_visibility(false);
    }

    let image = assets.load("sprites/cursor.png");
    let texture_atlas = TextureAtlas::from_grid(image, (16.0, 16.0).into(), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut transform = Transform::from_xyz(0.0, 0.0, 999.0);
    transform.scale = Vec3::ONE * 0.35;

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..default()
        })
        .insert(Cursor(Icon::Idle))
        .insert(CollisionShape::Sphere { radius: 0.4 })
        .insert(RigidBody::Sensor);
}

fn set_cursor_visual_mode(
    mut windows: ResMut<Windows>,
    game_state: Res<CurrentState<GameState>>,
    mut custom_cursor: Query<&mut Visibility, With<Cursor>>,
) {
    if game_state.is_changed() {
        if let Some(window) = windows.get_primary_mut() {
            window.set_cursor_visibility(matches!(game_state.0, GameState::UI));

            if let Ok(mut cursor) = custom_cursor.get_single_mut() {
                cursor.is_visible = !matches!(game_state.0, GameState::UI);
            }
        }
    }
}

fn update_sprite_position(
    windows: Res<Windows>,
    mut cursor: Query<&mut Transform, With<Cursor>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if let Some(window) = windows.get_primary() {
        if let Ok(mut transform) = cursor.get_single_mut() {
            if let Ok((camera, camera_transform)) = camera.get_single() {
                match window.cursor_position() {
                    Some(new_position) => {
                        // get the size of the window
                        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

                        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
                        let ndc = (new_position / window_size) * 2.0 - Vec2::ONE;

                        // matrix for undoing the projection and camera transform
                        let ndc_to_world = camera_transform.compute_matrix()
                            * camera.projection_matrix().inverse();

                        // use it to convert ndc to world-space coordinates
                        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

                        // reduce it to a 2D valued
                        let world_pos: Vec2 = world_pos.truncate();

                        transform.translation.x = world_pos.x;
                        transform.translation.y = world_pos.y;
                    }
                    None => (),
                }
            }
        }
    }
}

fn change_cursor_icon(mut query: Query<(&mut TextureAtlasSprite, &Cursor), Changed<Cursor>>) {
    if let Ok((mut sprite, cursor)) = query.get_single_mut() {
        sprite.index = cursor.0 as usize;
    }
}

fn handle_cursor_overlap(
    mut collisions: EventReader<CollisionEvent>,
    cursor_query: Query<Entity, With<Cursor>>,
    info_query: Query<&Interactable>,
) {
    if let Ok(cursor) = cursor_query.get_single() {
        for collision in collisions
            .iter()
            .filter(|ev| ev.collision_shape_entities().any(cursor))
        {
            if let CollisionEvent::Started(_, _) = *collision {
                if let Some(entity) = collision.collision_shape_entities().not(cursor) {
                    if let Ok(interactable) = info_query.get(entity) {
                        match interactable {
                            Interactable::Examine(text) => {
                                bevy::log::info!("Examine found!: {text}")
                            }
                            Interactable::Talk(node) => match node {
                                crate::dialogue::DialogueNode::Line(_) => todo!(),
                                crate::dialogue::DialogueNode::Choice(_) => todo!(),
                                _ => bevy::log::warn!("Found empty dialogue node."),
                            },
                        }
                    }
                }
            }
        }
    }
}
