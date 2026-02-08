//! Spawn the main level.

use bevy::prelude::*;

use crate::{asset_collection::PlayerAssets, demo::player::player, screens::Screen};

pub(super) fn plugin(_app: &mut App) {}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![player(400.0, &player_assets),],
    ));
}
