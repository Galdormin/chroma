use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;
use bevy_ecs_ldtk::TileEnumTags;

use crate::{
    GameLayer,
    ldtk::{GameColor, Tint},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, add_tint_to_wall);
}

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
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

fn add_tint_to_wall(
    mut commands: Commands,
    walls: Query<(Entity, &TileEnumTags), Added<TileEnumTags>>,
) {
    for (entity, tile_enum) in walls {
        let tint = tile_enum
            .tags
            .iter()
            .filter_map(|tag| tag.parse::<GameColor>().ok())
            .collect::<Tint>();

        commands
            .entity(entity)
            .insert((WallBundle::default(), tint));
    }
}
