//! Module with the definition of the camera
use bevy::prelude::*;

use bevy_modern_pixel_camera::prelude::*;

use crate::ldtk::gridvania::{GridLevelSelection, LEVEL_SIZE};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PixelCameraPlugin);

    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, update_camera_position);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        MainCamera,
        PixelViewport,
        WithUiScaling,
        FollowLevelSelection,
        PixelZoom::FitSize {
            width: LEVEL_SIZE.x,
            height: LEVEL_SIZE.y,
        },
    ));
}

/// Change the position of the camera to match the level position
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct FollowLevelSelection;

fn update_camera_position(
    time: Res<Time>,
    mut cameras: Query<&mut Transform, With<FollowLevelSelection>>,
    level_selection: Res<GridLevelSelection>,
) {
    // Smoothing factor: higher = faster movement
    // ~10.0 gives approximately 0.5 seconds to reach the target
    let smoothing = 10.0;

    for mut transform in &mut cameras {
        let target_position = level_selection.0.center().extend(0.0);

        // Exponential smoothing towards target position
        transform.translation = transform.translation.lerp(
            target_position,
            1.0 - (-smoothing * time.delta_secs()).exp(),
        );
    }
}
