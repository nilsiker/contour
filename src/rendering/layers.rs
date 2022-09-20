use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub const GROUND_LAYER: f32 = f32::MIN + 1.0;
pub const PAWN_LAYER: f32 = 4.0;
pub const ABOVE_LAYER: f32 = f32::MAX - 1.0;

#[derive(Component, Inspectable, Clone, Default)]
pub struct Layer(pub f32);
