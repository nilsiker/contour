use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use serde::Deserialize;

use crate::pawn::{player::Lantern, MoveDirection};

#[derive(Deserialize, Clone)]
pub struct Clip {
    current: usize,
    start: usize,
    end: usize,
}

#[derive(Component, Clone)]
pub struct Animations {
    pub active: bool,
    clips: HashMap<String, Clip>,
}
impl Animations {
    pub fn from_file(filename: &str) -> Self {
        let work_dir = std::env::current_dir().unwrap();
        let path = work_dir.join("assets/animations").join(filename);
        let file = std::fs::read(path).expect("valid anim ron file");

        let active = true;
        let clips: HashMap<String, Clip> =
            ron::de::from_bytes(&file[..]).expect("deserialization of anim ron file");


        Animations { active, clips }
    }
}

impl Clip {
    pub const fn new(start: usize, end: usize) -> Self {
        Clip {
            current: start,
            start,
            end,
        }
    }

    pub fn step(&mut self) -> usize {
        let temp = self.current;
        self.current += 1;
        if self.current > self.end {
            self.current = self.start;
        }

        temp
    }
}

struct AnimationTimer(Timer);

pub struct AnimPlugin;
impl Plugin for AnimPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AnimationTimer(Timer::new(Duration::from_millis(120), true)))
            .add_system(tick_anim_timer)
            .add_system(sprite_flipping)
            .add_system(sprite_animation.run_if(on_anim_timer));
    }
}

fn tick_anim_timer(mut timer: ResMut<AnimationTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

fn on_anim_timer(timer: Res<AnimationTimer>) -> bool {
    timer.0.just_finished()
}

fn sprite_flipping(mut query: Query<(&mut TextureAtlasSprite, &MoveDirection)>) {
    for (mut sprite, movement) in &mut query {
        if movement.0.x < 0. {
            sprite.flip_x = true
        } else if movement.0.x > 0. {
            sprite.flip_x = false
        }
    }
}

fn sprite_animation(
    mut query: Query<(
        &mut Animations,
        &mut TextureAtlasSprite,
        &MoveDirection,
        &Lantern,
    )>,
) {
    for (mut animations, mut sprite, direction, flashlight) in &mut query {
        let clips = &mut animations.clips;
        if direction.0.length_squared() > 0. {
            if flashlight.0 {
                sprite.index = clips
                    .get_mut("walk_light")
                    .expect("existing walk_light anim")
                    .step();
            } else {
                sprite.index = clips.get_mut("walk").expect("existing walk anim").step();
            }
        } else if flashlight.0 {
            sprite.index = clips
                .get_mut("idle_light")
                .expect("existing idle_light anim")
                .step();
        } else {
            sprite.index = clips.get_mut("idle").expect("existing idle anim").step();
        }
    }
}
