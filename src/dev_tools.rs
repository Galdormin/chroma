//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
};

use crate::screens::Screen;

const COLLIDER_COLOR: Color = Color::srgb(0.87, 0.55, 0.17);

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsDebugPlugin);
    app.insert_gizmo_config(
        PhysicsGizmos::colliders(COLLIDER_COLOR),
        GizmoConfig {
            enabled: false,
            ..default()
        },
    );

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        (
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_UI_KEY)),
            toggle_debug_collider.run_if(input_just_pressed(TOGGLE_COLLIDER_KEY)),
        ),
    );
}

const TOGGLE_UI_KEY: KeyCode = KeyCode::Insert;
const TOGGLE_COLLIDER_KEY: KeyCode = KeyCode::F2;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_debug_collider(mut gizmo_store: ResMut<GizmoConfigStore>) {
    let (gizmo_config, _) = gizmo_store.config_mut::<PhysicsGizmos>();
    gizmo_config.enabled = !gizmo_config.enabled;
}
