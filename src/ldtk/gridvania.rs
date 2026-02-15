use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    camera::{LEVEL_SIZE, LevelPosition, MainCamera},
    player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_level_selection);
}

/// Convert a Level Bevy translation to [`LevelPosition`].
///
/// LDtk position starts at top-left and bevy at bottom-left,
/// therefore  `- ivec2(0, 1)` is needed.
fn to_level_position(position: Vec3) -> LevelPosition {
    let ldtk_position = position.truncate().as_ivec2() * ivec2(1, -1);
    let grid_position = ldtk_position / LEVEL_SIZE - ivec2(0, 1);
    LevelPosition(grid_position)
}

fn update_level_selection(
    levels: Query<(&LevelIid, &Transform), (Without<Player>, Without<MainCamera>)>,
    player: Single<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Single<&mut LevelPosition, With<MainCamera>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_project_handle: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let ldtk_project = ldtk_project_assets
        .get(*ldtk_project_handle)
        .expect("Project should be loaded if level is spawned.");

    for (level_iid, level_transform) in &levels {
        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project.");

        let level_bounds = Rect {
            min: level_transform.translation.truncate(),
            max: level_transform.translation.truncate()
                + Vec2::new(level.px_wid as f32, level.px_hei as f32),
        };

        if !level_selection.is_match(&LevelIndices::default(), level)
            && level_bounds.contains(player.translation.truncate())
        {
            *level_selection = LevelSelection::iid(level.iid.clone());
            **camera = to_level_position(level_transform.translation);
        }
    }
}
