use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;
use bevy_ecs_ldtk::{IntGridCell, LdtkIntCell, TileEnumTags};

use crate::{
    GameLayer,
    theme::palette::{BROWN, GREEN, GREY, WHITE},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, add_tint_to_wall);
}

#[derive(Default, Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tint {
    #[default]
    White,
    Grey,
    Green,
    Brown,
}

impl TryFrom<String> for Tint {
    type Error = String;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        match value.as_ref() {
            "White" => Ok(Self::White),
            "Grey" => Ok(Self::Grey),
            "Green" => Ok(Self::Green),
            "Brown" => Ok(Self::Brown),
            _ => Err(format!("Cannot parse {value} as Tint.")),
        }
    }
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
        if let Ok(tint) = Tint::try_from(tile_enum.tags[0].clone()) {
            commands
                .entity(entity)
                .insert((WallBundle::default(), tint));
        }
    }
}
