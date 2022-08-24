use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::character::player::{Lantern, LightDirection, PlayerPosition};

#[derive(Inspectable)]
pub enum LightingMode {
    Dark,
    Lantern,
    Light,
}

#[derive(Component)]
pub struct Follow;
#[derive(Component, Inspectable)]
pub struct Lighting(pub LightingMode);
#[derive(Component, Inspectable)]
pub struct GlobalLight(pub bool);

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let transform = Transform::from_xyz(0., 0., 998.);
    let texture_handle = asset_server.load("darkness.png");

    let texture_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        texture_handle,
        Vec2 { x: 640., y: 320. },
        2,
        1,
    ));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: transform,
            ..default()
        })
        .insert(Name::new("Lighting"))
        .insert(Follow)
        .insert(Lighting(LightingMode::Dark))
        .insert(GlobalLight(true));
}

fn follow_player(
    players: Query<(&PlayerPosition, &LightDirection)>,
    mut follows: Query<&mut Transform, With<Follow>>,
) {
    for (player_position, light_direction) in &players {
        for mut follow_transform in &mut follows {
            follow_transform.translation.x = player_position.x + light_direction.0.x * 10.;
            follow_transform.translation.y = player_position.y + light_direction.0.y * 10.;
        }
    }
}

fn global_light_trigger(
    mut query: Query<&mut Lighting, Changed<GlobalLight>>,
    lantern_query: Query<&Lantern>,
) {
    for mut lighting in &mut query {
        for lantern in &lantern_query {
            match lighting.0 {
                LightingMode::Light => {
                    lighting.0 = if lantern.0 {
                        LightingMode::Lantern
                    } else {
                        LightingMode::Dark
                    }
                }
                LightingMode::Dark => lighting.0 = LightingMode::Light,
                LightingMode::Lantern => lighting.0 = LightingMode::Light,
            }
        }
    }
}

fn lantern_light_trigger(mut query: Query<&mut Lighting>, lantern_query: Query<&Lantern, Changed<Lantern>>) {
    for mut lighting in &mut query {
        for _ in &lantern_query {
            match lighting.0 {
                LightingMode::Dark => lighting.0 = LightingMode::Lantern,
                LightingMode::Lantern => lighting.0 = LightingMode::Dark,
                LightingMode::Light => (),
            };
        }
    }
}

fn update_lighting_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &mut Lighting, &mut Visibility), Changed<Lighting>>,
) {
    for (mut sprite, lighting, mut visibility) in &mut query {
        match lighting.0 {
            LightingMode::Dark => {
                sprite.index = 1;
                visibility.is_visible = true;
            }
            LightingMode::Lantern => {
                sprite.index = 0;
                visibility.is_visible = true;
            }
            LightingMode::Light => {
                visibility.is_visible = false;
            }
        }
    }
}

fn debug_global(input: Res<Input<KeyCode>>, mut query: Query<&mut GlobalLight>) {
    if input.just_pressed(KeyCode::G) {
        for mut global_light in &mut query {
            global_light.0 = !global_light.0;
        }
    }
}

pub struct LightingPlugin;

impl Plugin for LightingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(follow_player)
            .add_system(debug_global)
            .add_system(global_light_trigger)
            .add_system(lantern_light_trigger)
            .add_system(update_lighting_sprite)
            .register_inspectable::<Lighting>();
    }
}
