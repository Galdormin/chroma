//! Code for the player character

use bevy::prelude::*;

use crate::{
    PausableSystems,
    ldtk::GameColor,
    player::{
        movement::CharacterMovementBundle, physics::CharacterPhysicsBundle,
        visual::CharacterVisualBundle,
    },
    screens::Screen,
};

pub mod movement;
pub mod physics;
pub mod visual;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((movement::plugin, physics::plugin, visual::plugin));

    app.add_systems(OnEnter(Screen::Gameplay), spawn_character);
    app.add_systems(
        Update,
        (
            movement::apply_gravity,
            physics::run_move_and_slide,
            physics::update_grounded,
            movement::update_coyote_timer,
        )
            .chain()
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    )
    .add_systems(Update, movement::apply_movement);
}

#[derive(Component)]
pub struct Player;

pub fn spawn_character(
    mut commands: Commands,
    material_asets: ResMut<Assets<ColorMaterial>>,
    mesh_assets: ResMut<Assets<Mesh>>,
    maybe_player: Option<Single<Entity, With<Player>>>,
) {
    let shape = Capsule2d::new(8.0, 10.0);

    let bundle = (
        Player,
        CharacterVisualBundle::new(shape, GameColor::Grey, mesh_assets, material_asets),
        CharacterMovementBundle::new(10.0, 4.0, 0.3, 0.3),
        CharacterPhysicsBundle::new(shape),
        Transform::from_xyz(765.0, -130.0, 3.0),
    );

    if let Some(player) = maybe_player {
        commands.entity(*player).insert(bundle);
    } else {
        commands.spawn(bundle);
    }
}
