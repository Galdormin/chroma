use avian2d::prelude::{Collider, CollisionLayers, RigidBody, Rotation};
use bevy::prelude::*;
use bevy_ecs_ldtk::{TileEnumTags, TileMetadata};

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
    walls: Query<(Entity, &TileEnumTags, Option<&TileMetadata>), Added<TileEnumTags>>,
) {
    for (entity, tile_enum, maybe_metadata) in walls {
        let tint = tile_enum
            .tags
            .iter()
            .filter_map(|tag| tag.parse::<GameColor>().ok())
            .collect::<Tint>();

        let bundle = if let Some(metadata) = maybe_metadata {
            let coords: Vec<(i32, i32, i32, i32)> = ron::from_str(&metadata.data).unwrap();

            // Version Compound
            let shapes = coords
                .iter()
                .copied()
                .map(|(a, b, c, d)| {
                    let rect = Rect::new(a as f32, b as f32, c as f32, d as f32);
                    let shape = Collider::rectangle(rect.width(), rect.height());
                    let position = rect.center() * vec2(1.0, -1.0) + vec2(-8.0, 8.0);

                    (position, Rotation::default(), shape)
                })
                .collect::<Vec<_>>();

            let collider = Collider::compound(shapes);

            WallBundle {
                collider,
                ..default()
            }
        } else {
            WallBundle::default()
        };

        commands.entity(entity).insert((bundle, tint));
    }
}
