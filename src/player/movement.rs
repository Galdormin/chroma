use avian2d::{math::Scalar, prelude::LinearVelocity};
use bevy::{math::FloatPow, prelude::*};

use crate::player::{Player, physics::Grounded};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_coyote_timer, apply_movement, apply_gravity).chain(),
    );
}

const BLOCK_SIZE: Scalar = 16.0;

/// The speed used for character movement.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MovementSpeed(pub Scalar);

/// The velocity impulse for the jump.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct JumpImpulse(pub Scalar);

/// The coyote timer of the Jump
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct CoyoteTimer(Timer);

impl Default for CoyoteTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

impl CoyoteTimer {
    fn can_jump(&self) -> bool {
        !self.0.is_finished()
    }

    fn reset_timer(&mut self) {
        self.0.reset();
    }
}

/// The gravitational acceleration used for a character controller.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct GravityController {
    /// Gravity applied when the character is jumping
    pub jump_gravity: Scalar,
    /// Gravity applied when the character is falling
    pub fall_gravity: Scalar,
    /// Maximal vertical velocity when falling
    pub terminal_velocity: Scalar,
}

impl Default for GravityController {
    fn default() -> Self {
        Self {
            jump_gravity: 300.0,
            fall_gravity: 300.0,
            terminal_velocity: 200.0,
        }
    }
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct CharacterMovementBundle {
    speed: MovementSpeed,
    jump_impulse: JumpImpulse,
    coyote_timer: CoyoteTimer,
    gravity: GravityController,
}

impl CharacterMovementBundle {
    /// `speed` in block/sec and `jump_height` in blocks and `jump_time` in sec
    pub fn new(speed: Scalar, jump_height: Scalar, jump_time: Scalar, fall_time: Scalar) -> Self {
        let jump_gravity = (2.0 * jump_height * BLOCK_SIZE) / jump_time.squared();
        let fall_gravity = (2.0 * jump_height * BLOCK_SIZE) / fall_time.squared();
        let jump_velocity = jump_gravity * jump_time;

        info!("gravity: {jump_gravity}, jump: {jump_velocity}");

        Self {
            speed: MovementSpeed(speed * BLOCK_SIZE),
            jump_impulse: JumpImpulse(jump_velocity),
            coyote_timer: CoyoteTimer::default(),
            gravity: GravityController {
                jump_gravity,
                fall_gravity,
                terminal_velocity: 25.0 * BLOCK_SIZE,
            },
        }
    }
}

/// Applies he gravity to character controllers.
fn apply_gravity(
    time: Res<Time>,
    mut controllers: Query<(&GravityController, &mut LinearVelocity), Without<Grounded>>,
) {
    let delta_time = time.delta_secs();
    for (gravity, mut linear_velocity) in &mut controllers {
        let gravity_force = if linear_velocity.y > 0.0 {
            gravity.jump_gravity
        } else {
            gravity.fall_gravity
        };

        linear_velocity.y -= gravity_force * delta_time;

        if linear_velocity.y < -gravity.terminal_velocity {
            linear_velocity.y = -gravity.terminal_velocity;
        }
    }
}

/// Responds to [`Action`] events and moves character controllers accordingly.
fn apply_movement(
    input: Res<ButtonInput<KeyCode>>,
    controller: Single<
        (
            &MovementSpeed,
            &mut LinearVelocity,
            &JumpImpulse,
            &CoyoteTimer,
            Has<Grounded>,
        ),
        With<Player>,
    >,
) {
    let (movement_speed, mut linear_velocity, jump_impulse, coyote_timer, is_grounded) =
        controller.into_inner();

    if input.just_pressed(KeyCode::Space) && (is_grounded || coyote_timer.can_jump()) {
        linear_velocity.y = jump_impulse.0;
    }

    let mut direction = 0;
    if input.pressed(KeyCode::KeyA) {
        direction += -1
    }
    if input.pressed(KeyCode::KeyD) {
        direction += 1
    }

    linear_velocity.x = (direction as Scalar) * movement_speed.0;
}

/// Update the coyote timer every frame
fn update_coyote_timer(time: Res<Time>, players: Query<(&mut CoyoteTimer, Has<Grounded>)>) {
    for (mut coyote_timer, is_grounded) in players {
        if is_grounded {
            coyote_timer.reset_timer();
        } else {
            coyote_timer.0.tick(time.delta());
        }
    }
}
