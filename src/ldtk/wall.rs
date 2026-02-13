use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;
use bevy_ecs_ldtk::{IntGridCell, LdtkIntCell, app::LdtkIntCellAppExt};

use crate::{
    GameLayer,
    theme::palette::{BROWN, GREEN, GREY, WHITE},
};

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.register_ldtk_int_cell::<WallBundle>(2);
    app.register_ldtk_int_cell::<WallBundle>(3);
}

#[derive(Default, Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tint {
    #[default]
    White,
    Grey,
    Green,
    Brown,
}

impl From<IntGridCell> for Tint {
    fn from(value: IntGridCell) -> Self {
        match value.value {
            1 => Self::Grey,
            2 => Self::Green,
            3 => Self::Brown,
            _ => Self::White,
        }
    }
}

impl Tint {
    pub fn color(&self) -> Color {
        match self {
            Tint::White => WHITE,
            Tint::Grey => GREY,
            Tint::Green => GREEN,
            Tint::Brown => BROWN,
        }
    }
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
    #[from_int_grid_cell]
    color: Tint,
    collider: Collider,
    collision_layers: CollisionLayers,
    body: RigidBody,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            color: Tint::White,
            collider: Collider::rectangle(16.0, 16.0),
            collision_layers: CollisionLayers::new(GameLayer::Ground, [GameLayer::Player]),
            body: RigidBody::Static,
        }
    }
}
