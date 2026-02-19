use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ecs_ldtk::prelude::*;

use crate::{asset_collection::LevelAssets, player::Player, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GridLevelSelection>();

    app.add_systems(
        Update,
        populate_gridvania_levels.run_if(resource_added::<LevelAssets>),
    )
    .add_systems(
        Update,
        update_level_selection.run_if(resource_changed::<GridLevelSelection>),
    )
    .add_systems(
        Update,
        level_selection_follow_player.run_if(in_state(Screen::Gameplay)),
    );
}

pub const LEVEL_SIZE: IVec2 = IVec2::new(512, 288);

/// Position of a Level in the GridVania world.
///
/// The level at position (0, 0) is the one with the upper left corner at
/// (0, 0) in world coords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Default)]
pub struct GridCoords(IVec2);

impl GridCoords {
    pub fn new(coords: IVec2) -> Self {
        Self(coords)
    }

    pub fn get(&self) -> IVec2 {
        self.0
    }

    /// Convert a 2D Bevy position to [`Self`]
    pub fn from_world_position(position: Vec2) -> Self {
        let ldtk_position = position * vec2(1., -1.);
        let grid_pos = (ldtk_position / LEVEL_SIZE.as_vec2()).floor().as_ivec2();
        Self(grid_pos)
    }

    /// Return the 2D Bevy position of the center of the Level represented by the coordinates.
    pub fn center(&self) -> Vec2 {
        let ldtk_position = self.0 * LEVEL_SIZE + LEVEL_SIZE / 2;
        (ldtk_position * ivec2(1, -1)).as_vec2()
    }
}

impl<T: Into<IVec2>> From<T> for GridCoords {
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

/// Resource to select which Level to spawn based on the grid position.
/// Sycnhronized with [`LevelSelection`].
///
/// Must not be used with [`LevelSelection`]
#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct GridLevelSelection(pub GridCoords);

impl GridLevelSelection {
    pub fn new(coords: impl Into<GridCoords>) -> Self {
        Self(coords.into())
    }
}

/// Resource that maps a [`GridCoords`] to a [`LevelIid`].
///
/// Automatically filled and added when the [`LdtkProject`] is loaded.
#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct GridvaniaLevels(HashMap<GridCoords, LevelIid>);

impl GridvaniaLevels {
    pub fn get_level_at(&self, coords: impl Into<GridCoords>) -> Option<LevelIid> {
        self.0.get(&coords.into()).cloned()
    }
}

fn populate_gridvania_levels(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let ldtk_project = ldtk_project_assets
        .get(level_assets.world.id())
        .expect("Project should be loaded by then.");

    let level_map = ldtk_project
        .iter_raw_levels()
        .map(|l| {
            let world_coords = ivec2(l.world_x, -l.world_y).as_vec2();
            let grid_coords = GridCoords::from_world_position(world_coords);
            (grid_coords, LevelIid::new(l.iid.clone()))
        })
        .collect::<HashMap<_, _>>();

    info!("{} level loaded.", level_map.len());
    commands.insert_resource(GridvaniaLevels(level_map));
}

fn update_level_selection(
    grid_level_selection: Res<GridLevelSelection>,
    mut level_selection: ResMut<LevelSelection>,
    levels: If<Res<GridvaniaLevels>>,
) {
    if let Some(level_iid) = levels.get_level_at(grid_level_selection.0) {
        *level_selection = LevelSelection::Iid(level_iid);
    } else {
        warn!("Level at {} does not exists.", grid_level_selection.0.get())
    }
}

fn level_selection_follow_player(
    player: Single<&Transform, With<Player>>,
    mut grid_level_selection: ResMut<GridLevelSelection>,
) {
    let grid_pos = GridCoords::from_world_position(player.translation.truncate());

    if grid_pos != grid_level_selection.0 {
        *grid_level_selection = GridLevelSelection(grid_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_coords_from_world_position() {
        // Typical case
        assert_eq!(
            GridCoords::from_world_position(vec2(1279., -1012.)),
            GridCoords(ivec2(2, 3))
        );
        assert_eq!(
            GridCoords::from_world_position(vec2(767., -115.)),
            GridCoords(ivec2(1, 0))
        );
        assert_eq!(
            GridCoords::from_world_position(vec2(168., -115.)),
            GridCoords(ivec2(0, 0))
        );

        // Negative coords (#6)
        assert_eq!(
            GridCoords::from_world_position(vec2(325., 260.)),
            GridCoords(ivec2(0, -1))
        );
        assert_eq!(
            GridCoords::from_world_position(vec2(-325., -215.)),
            GridCoords(ivec2(-1, 0))
        );
        assert_eq!(
            GridCoords::from_world_position(vec2(-1322., 426.)),
            GridCoords(ivec2(-3, -2))
        );
    }
}
