use std::str::FromStr;

use avian2d::{
    math::TAU,
    prelude::{
        Collider, CollisionEventsEnabled, CollisionLayers, CollisionStart, RigidBody, Sensor,
    },
};
use bevy::prelude::*;

use bevy_ecs_ldtk::prelude::*;

use crate::{GameLayer, ldtk::Tint, player::Player};

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<ObjectBundle>("Object");

    app.add_systems(Update, (register_initial_position, levitate_object).chain());
    app.add_systems(Update, register_collision_observer);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ObjectLevitation {
    initial_position: Option<Vec2>,
    /// Oscillation amplitude of the movement in pixel.
    pub amplitude: f32,
    /// Oscillation period of the movement in sec.
    pub period: f32,
}

impl Default for ObjectLevitation {
    fn default() -> Self {
        Self {
            initial_position: None,
            amplitude: 8.0,
            period: 5.0,
        }
    }
}

impl ObjectLevitation {
    pub fn next_position(&self, time: f32) -> Option<Vec2> {
        let delta = self.amplitude / 2.0 * (time * TAU / self.period).sin() * Vec2::new(0., 1.);
        self.initial_position.map(|pos| pos + delta)
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub enum ObjectType {
    Book,
    Feather,
}

impl FromStr for ObjectType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Book" => Ok(Self::Book),
            "Feather" => Ok(Self::Feather),
            _ => Err(format!("Cannot parse {s} as ObjectType")),
        }
    }
}

impl From<&EntityInstance> for ObjectType {
    fn from(instance: &EntityInstance) -> Self {
        let object_type = instance.get_enum_field("type").unwrap();
        object_type.parse::<Self>().unwrap()
    }
}

#[derive(Bundle, LdtkEntity)]
struct ObjectBundle {
    #[sprite_sheet]
    sprite: Sprite,
    #[from_entity_instance]
    object_type: ObjectType,
    #[with(Tint::from_colors_field)]
    tints: Tint,
    #[default]
    levitation: ObjectLevitation,
    #[default]
    physics: ObjectPhysicalBundle,
}

#[derive(Bundle)]
struct ObjectPhysicalBundle {
    body: RigidBody,
    collider: Collider,
    collision_event: CollisionEventsEnabled,
    collision_layer: CollisionLayers,
}

impl Default for ObjectPhysicalBundle {
    fn default() -> Self {
        Self {
            body: RigidBody::Static,
            collider: Collider::rectangle(16.0, 16.0),
            collision_event: CollisionEventsEnabled,
            collision_layer: CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        }
    }
}

fn register_initial_position(
    objects: Query<(&mut ObjectLevitation, &Transform), Added<ObjectLevitation>>,
) {
    for (mut levitation, transform) in objects {
        levitation.initial_position = Some(transform.translation.truncate());
    }
}

fn levitate_object(objects: Query<(&ObjectLevitation, &mut Transform)>, time: Res<Time>) {
    for (levitation, mut transform) in objects {
        let Some(next_position) = levitation.next_position(time.elapsed_secs()) else {
            warn!("ObjectLevitation was not initialized.");
            continue;
        };

        transform.translation.y = next_position.y;
    }
}

fn register_collision_observer(mut commands: Commands, objects: Query<Entity, Added<ObjectType>>) {
    for object in objects {
        commands.entity(object).observe(detect_object_pickup);
    }
}

fn detect_object_pickup(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    mut objects: Query<(&mut Visibility, &Tint), With<ObjectType>>,
    player_tint: Single<&Tint, With<Player>>,
) {
    let Ok((mut object_visibility, object_tint)) = objects.get_mut(trigger.event_target()) else {
        return;
    };

    if object_tint.share_color_with(&player_tint) {
        // TODO: Change this to despawn. Simple despawn panic with physics engine.
        *object_visibility = Visibility::Hidden;
        commands.entity(trigger.event_target()).insert(Sensor);
    }
}
