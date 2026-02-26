//! Code for the player character

use bevy::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;

use crate::{
    PausableSystems,
    asset_collection::LevelAssets,
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
    level_assets: Res<LevelAssets>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    // Find the unique Spawn entity
    let ldtk_project = ldtk_project_assets
        .get(level_assets.world.id())
        .expect("Project should be loaded by then.");

    let spawn_pos = ldtk_project
        .json_data()
        .toc
        .iter()
        .filter(|entry| entry.identifier == "Spawn")
        .map(|entry| {
            entry
                .instances_data
                .first()
                .map(|i| IVec2::new(i.world_x, -i.world_y).as_vec2())
        })
        .next()
        .flatten()
        .unwrap_or(Vec2::new(445., -200.));

    // Create the character bundle
    let shape = Capsule2d::new(8.0, 10.0);
    let bundle = (
        Player,
        CharacterVisualBundle::new(shape, GameColor::Grey, mesh_assets, material_asets),
        CharacterMovementBundle::new(10.0, 4.0, 0.3, 0.3),
        CharacterPhysicsBundle::new(shape),
        Transform::from_translation(spawn_pos.extend(3.0)),
    );

    if let Some(player) = maybe_player {
        commands.entity(*player).insert(bundle);
    } else {
        commands.spawn(bundle);
    }
}
