//! Code for the player character

use bevy::prelude::*;

use crate::{
    player::{movement::CharacterMovementBundle, physics::CharacterPhysicsBundle},
    theme::palette::WHITE,
};

pub mod movement;
pub mod physics;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, physics::plugin));

    app.add_systems(
        FixedUpdate,
        (
            movement::apply_gravity,
            physics::run_move_and_slide,
            physics::update_grounded,
            movement::update_coyote_timer,
        )
            .chain(),
    )
    .add_systems(Update, movement::apply_movement);
}

#[derive(Component)]
struct Player;

pub fn spawn_character(
    mut commands: Commands,
    mut material_asets: ResMut<Assets<ColorMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    let shape = Capsule2d::new(8.0, 10.0);

    commands.spawn((
        Player,
        Mesh2d(mesh_assets.add(shape)),
        MeshMaterial2d(material_asets.add(Color::from(WHITE))),
        CharacterMovementBundle::new(10.0, 4.0, 0.3, 0.3),
        CharacterPhysicsBundle::new(shape),
        Transform::from_xyz(765.0, -130.0, 1.0),
    ));
}
