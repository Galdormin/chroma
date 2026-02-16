//! Module with the definition of the camera
use bevy::prelude::*;

use bevy_modern_pixel_camera::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PixelCameraPlugin);

    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, update_camera_position);
}

pub const LEVEL_SIZE: IVec2 = IVec2::new(512, 288);
pub const LEVEL_CENTER: IVec2 = IVec2::new(LEVEL_SIZE.x / 2, -LEVEL_SIZE.y / 2);

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
        LevelPosition::new(0, 0),
        PixelZoom::FitSize {
            width: LEVEL_SIZE.x,
            height: LEVEL_SIZE.y,
        },
        Transform::from_translation(LEVEL_CENTER.extend(0).as_vec3()),
    ));
}

/// Change the position of the camera to match the level position
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct LevelPosition(pub IVec2);

impl LevelPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2 { x, y })
    }

    pub fn to_xy(&self) -> Vec2 {
        (self.0 * LEVEL_SIZE * IVec2::new(1, -1)).as_vec2()
    }
}

fn update_camera_position(
    time: Res<Time>,
    mut cameras: Query<(&LevelPosition, &mut Transform), With<Camera2d>>,
) {
    // Smoothing factor: higher = faster movement
    // ~10.0 gives approximately 0.5 seconds to reach the target
    let smoothing = 10.0;

    for (level, mut transform) in &mut cameras {
        let target_position = (level.to_xy() + LEVEL_CENTER.as_vec2()).extend(0.0);

        // Exponential smoothing towards target position
        transform.translation = transform.translation.lerp(
            target_position,
            1.0 - (-smoothing * time.delta_secs()).exp(),
        );
    }
}
