use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkIntCell, app::LdtkIntCellAppExt};

use crate::GameLayer;

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_int_cell::<WallBundle>(1);
    app.register_ldtk_int_cell::<WallBundle>(2);
    app.register_ldtk_int_cell::<WallBundle>(3);
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
    collider: Collider,
    collision_layers: CollisionLayers,
    body: RigidBody,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            collider: Collider::rectangle(16.0, 16.0),
            collision_layers: CollisionLayers::new(GameLayer::Ground, [GameLayer::Player]),
            body: RigidBody::Static,
        }
    }
}
