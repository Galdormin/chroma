use avian2d::{
    math::{AdjustPrecision, Scalar, Vector},
    prelude::*,
};
use bevy::prelude::*;

use crate::{GameLayer, ldtk::wall::Wall, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PhysicsSchedule,
        (update_grounded, kinematic_controller_collisions)
            .chain()
            .in_set(NarrowPhaseSystems::Last),
    );
}

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

    /// Use to detect if character is [`Grounded`]
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
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(3.0),
        }
    }
}

/// Updates the [`Grounded`] status for character controllers.
fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits), With<Player>>,
    walls: Query<Entity, With<Wall>>,
) {
    for (entity, hits) in &mut query {
        // // Filter hits on wall
        let hits = hits
            .iter()
            .filter(|hit_data| walls.contains(hit_data.entity))
            .collect::<Vec<&ShapeHitData>>();

        // The character is grounded if the shape caster has a hit
        if !hits.is_empty() {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

/// Kinematic bodies do not get pushed by collisions by default,
/// so it needs to be done manually.
///
/// This system handles collision response for kinematic character controllers
/// by pushing them along their contact normals by the current penetration depth,
/// and applying velocity corrections in order to snap to slopes, slide along walls,
/// and predict collisions using speculative contacts.
#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Collisions,
    bodies: Query<&RigidBody>,
    collider_rbs: Query<&ColliderOf, Without<Sensor>>,
    mut character_controllers: Query<
        (&mut Position, &mut LinearVelocity),
        (With<RigidBody>, With<Player>),
    >,
    time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([&ColliderOf { body: rb1 }, &ColliderOf { body: rb2 }]) =
            collider_rbs.get_many([contacts.collider1, contacts.collider2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, mut linear_velocity) =
            if let Ok(character) = character_controllers.get_mut(rb1) {
                is_first = true;
                character_rb = *bodies.get(rb1).unwrap();
                is_other_dynamic = bodies.get(rb2).is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = character_controllers.get_mut(rb2) {
                is_first = false;
                character_rb = *bodies.get(rb2).unwrap();
                is_other_dynamic = bodies.get(rb1).is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.normal
            } else {
                manifold.normal
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.points.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            if deepest_penetration > 0.0 {
                // The character is intersecting an unclimbable object, like a wall.
                // We want the character to slide along the surface, similarly to
                // a collide-and-slide algorithm.

                // Don't apply an impulse if the character is moving away from the surface.
                if linear_velocity.dot(normal) > 0.0 {
                    continue;
                }

                // Slide along the surface, rejecting the velocity along the contact normal.
                let impulse = linear_velocity.reject_from_normalized(normal);
                linear_velocity.0 = impulse;
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude =
                    normal_speed - (deepest_penetration / time.delta_secs_f64().adjust_precision());
                let mut impulse = impulse_magnitude * normal;

                // Avoid climbing up walls.
                impulse.y = impulse.y.max(0.0);
                linear_velocity.0 -= impulse;
            }
        }
    }
}
