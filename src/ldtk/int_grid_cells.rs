use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

use super::ColliderBundle;

pub struct IntGridCellPlugin;
impl Plugin for IntGridCellPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_ldtk_int_cell::<WallBundle>(1);
    }
}

#[derive(Component, Default)]
struct Wall;

#[derive(Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
    #[from_int_grid_cell]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        let rotation_constraints = RotationConstraints::lock();

        if int_grid_cell.value == 1 {
            ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rotation_constraints,
                rigid_body: RigidBody::Static,
                ..Default::default()
            }
        } else {
            ColliderBundle::default()
        }
    }
}
