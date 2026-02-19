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

const TILE_SIZE: u32 = 16;

#[derive(Default, Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Wall;

type RectShape = (u32, u32, u32, u32);

#[derive(Debug, serde::Deserialize)]
pub enum WallCollider {
    Top(u32),
    Bottom(u32),
    Left(u32),
    Right(u32),
    TopLeft(u32, u32),
    TopRight(u32, u32),
    BottomLeft(u32, u32),
    BottomRight(u32, u32),
    Custom(Vec<RectShape>),
}

impl WallCollider {
    fn into_coords(self) -> Vec<RectShape> {
        match self {
            WallCollider::Top(height) => vec![(0, 0, TILE_SIZE, height)],
            WallCollider::Bottom(height) => vec![(0, TILE_SIZE - height, TILE_SIZE, TILE_SIZE)],
            WallCollider::Left(length) => vec![(0, 0, length, TILE_SIZE)],
            WallCollider::Right(length) => vec![(TILE_SIZE - length, 0, TILE_SIZE, TILE_SIZE)],
            WallCollider::TopLeft(height, length) => {
                vec![(0, 0, TILE_SIZE, height), (0, height, length, TILE_SIZE)]
            }
            WallCollider::TopRight(height, length) => {
                vec![
                    (0, 0, TILE_SIZE, height),
                    (TILE_SIZE - length, height, TILE_SIZE, TILE_SIZE),
                ]
            }
            WallCollider::BottomLeft(height, length) => {
                vec![
                    (0, TILE_SIZE - height, TILE_SIZE, TILE_SIZE),
                    (0, 0, length, TILE_SIZE - height),
                ]
            }
            WallCollider::BottomRight(height, length) => {
                vec![
                    (0, TILE_SIZE - height, TILE_SIZE, TILE_SIZE),
                    (TILE_SIZE - length, 0, TILE_SIZE, TILE_SIZE - height),
                ]
            }
            WallCollider::Custom(items) => items,
        }
    }

    fn into_collider(self) -> Collider {
        let shapes = self
            .to_coords()
            .iter()
            .copied()
            .map(|(a, b, c, d)| {
                let rect = Rect::new(a as f32, b as f32, c as f32, d as f32);
                let shape = Collider::rectangle(rect.width(), rect.height());
                let position = rect.center() * vec2(1.0, -1.0) + vec2(-8.0, 8.0);

                (position, Rotation::default(), shape)
            })
            .collect::<Vec<_>>();

        Collider::compound(shapes)
    }
}

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
            match ron::from_str::<WallCollider>(&metadata.data) {
                Ok(wall_collider) => WallBundle {
                    collider: wall_collider.to_collider(),
                    ..default()
                },
                Err(err) => {
                    warn!(
                        "Could not deserialize '{}' as WallCollider because of {}",
                        metadata.data, err
                    );
                    WallBundle::default()
                }
            }
        } else {
            WallBundle::default()
        };

        commands.entity(entity).insert((bundle, tint));
    }
}
