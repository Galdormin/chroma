use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionLayers, CollisionStart, RigidBody, Sensor,
};
use bevy::prelude::*;

use bevy_ecs_ldtk::prelude::*;

use crate::{
    GameLayer,
    asset_collection::AudioAssets,
    audio::{AudioSettings, sound_effect},
    ldtk::{GameColor, Tint, entities::object::ObjectLevitation},
    player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_entity::<DropletBundle>("Droplet");

    app.add_systems(Update, register_droplet_observer);
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Droplet(pub GameColor);

impl From<&EntityInstance> for Droplet {
    fn from(instance: &EntityInstance) -> Self {
        let color = instance
            .get_enum_field("color")
            .unwrap()
            .parse::<GameColor>()
            .unwrap();

        Self(color)
    }
}

#[derive(Bundle, LdtkEntity)]
struct DropletBundle {
    #[sprite_sheet]
    sprite: Sprite,
    #[from_entity_instance]
    droplet: Droplet,
    #[default]
    levitation: ObjectLevitation,
    #[default]
    droplet_sensor: DropletSensorBundle,
}

#[derive(Bundle)]
struct DropletSensorBundle {
    body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    collision_event: CollisionEventsEnabled,
    collision_layer: CollisionLayers,
}

impl Default for DropletSensorBundle {
    fn default() -> Self {
        Self {
            body: RigidBody::Static,
            collider: Collider::circle(8.0),
            sensor: Sensor,
            collision_event: CollisionEventsEnabled,
            collision_layer: CollisionLayers::new(GameLayer::Sensor, [GameLayer::Player]),
        }
    }
}

fn register_droplet_observer(mut commands: Commands, droplets: Query<Entity, Added<Droplet>>) {
    for droplet in droplets {
        commands.entity(droplet).observe(detect_droplet_pickup);
    }
}

fn detect_droplet_pickup(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    droplets: Query<&Droplet>,
    mut player_tint: Single<&mut Tint, With<Player>>,
    audio_assets: Res<AudioAssets>,
    audio_settings: Res<AudioSettings>,
) {
    let Ok(droplet) = droplets.get(trigger.event_target()) else {
        return;
    };

    **player_tint = Tint::from_color(droplet.0);
    commands.spawn(sound_effect(
        audio_assets.paint_spray.clone(),
        &audio_settings,
    ));
}
