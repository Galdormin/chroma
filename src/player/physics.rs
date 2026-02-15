use avian2d::{
    math::{AdjustPrecision, AsF32, Vector},
    prelude::*,
};
use bevy::prelude::*;

use crate::{
    GameLayer,
    ldtk::{Tint, wall::Wall},
    player::Player,
};

pub(super) fn plugin(_app: &mut App) {}

/// Marker component indicating that a [`Player`] is grounded.
#[derive(Component, Debug, Default)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Bundle)]
pub struct CharacterPhysicsBundle {
    // Default Avian move_and_slide components
    collider: Collider,
    body: RigidBody,
    transform_interp: TransformInterpolation,
    colliding_entities: CollidingEntities,
    collision_layer: CollisionLayers,
    position_integration: CustomPositionIntegration,

    /// Shape cast used to detect ground contacts for [`Grounded`].
    ground_caster: ShapeCaster,
}

impl CharacterPhysicsBundle {
    pub fn new(capsule: Capsule2d) -> Self {
        let collider = Collider::from(capsule);

        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            collider,
            body: RigidBody::Kinematic,
            transform_interp: TransformInterpolation,
            colliding_entities: CollidingEntities::default(),
            collision_layer: CollisionLayers::new(
                GameLayer::Player,
                [GameLayer::Ground, GameLayer::Sensor],
            ),
            position_integration: CustomPositionIntegration,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(3.0)
                .with_max_hits(7),
        }
    }
}

/// Updates the [`Grounded`] status for character controllers.
pub(super) fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits, &Tint), With<Player>>,
    walls: Query<&Tint, With<Wall>>,
) {
    for (entity, hits, player_tint) in &mut query {
        // // Filter hits on wall
        let hits = hits
            .iter()
            .filter(|hit_data| {
                walls
                    .get(hit_data.entity)
                    .is_ok_and(|hit_tint| !player_tint.share_color_with(hit_tint))
            })
            .collect::<Vec<&ShapeHitData>>();

        // The character is grounded if the shape caster has a hit
        if !hits.is_empty() {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// System to run the move and slide algorithm, updating the player's transform and velocity.
///
/// This replaces Avian's default "position integration" that moves kinematic bodies based on their
/// velocity without any collision handling.
pub(super) fn run_move_and_slide(
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut LinearVelocity,
            &Collider,
            &Tint,
        ),
        With<Player>,
    >,
    walls: Query<(Entity, &Tint), With<Wall>>,
    move_and_slide: MoveAndSlide,
    time: Res<Time>,
) {
    for (entity, mut transform, mut lin_vel, collider, player_tint) in &mut query {
        let tint_walls = walls
            .iter()
            .filter(|(_, wall_tint)| player_tint.share_color_with(*wall_tint))
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

        // Perform move and slide
        let MoveAndSlideOutput {
            position,
            projected_velocity,
        } = move_and_slide.move_and_slide(
            collider,
            transform.translation.xy().adjust_precision(),
            transform
                .rotation
                .to_euler(EulerRot::XYZ)
                .2
                .adjust_precision(),
            lin_vel.0,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &SpatialQueryFilter::from_excluded_entities(tint_walls.iter().copied().chain([entity])),
            |_| MoveAndSlideHitResponse::Accept,
        );

        // Update transform and velocity
        transform.translation = position.extend(transform.translation.z).f32();
        lin_vel.0 = projected_velocity;
    }
}
