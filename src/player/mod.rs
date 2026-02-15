//! Code for the player character

use bevy::prelude::*;

use crate::{
    ldtk::GameColor,
    player::{
        movement::CharacterMovementBundle, physics::CharacterPhysicsBundle,
        visual::CharacterVisualBundle,
    },
};

pub mod movement;
pub mod physics;
pub mod visual;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, physics::plugin, visual::plugin));

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
pub struct Player;

pub fn spawn_character(
    mut commands: Commands,
    material_asets: ResMut<Assets<ColorMaterial>>,
    mesh_assets: ResMut<Assets<Mesh>>,
) {
    let shape = Capsule2d::new(8.0, 10.0);

    commands.spawn((
        Player,
        CharacterVisualBundle::new(shape, GameColor::White, mesh_assets, material_asets),
        CharacterMovementBundle::new(10.0, 4.0, 0.3, 0.3),
        CharacterPhysicsBundle::new(shape),
        Transform::from_xyz(765.0, -130.0, 3.0),
    ));
}
