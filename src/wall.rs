use crate::{Hitbox, prelude::*};

#[derive(Default, Component)]
pub struct Respawnable;

#[derive(Component, Default)]
#[require(
    CollisionLayers::new(
        PhysicsLayers::Ground,
        [
            PhysicsLayers::Enemy,
            PhysicsLayers::Player,
            PhysicsLayers::PlayerHitbox,
            PhysicsLayers::EnemyHitbox,
        ],
    ),
    RigidBody::Static,
    Collider::rectangle(128.0, 128.0),
)]
pub struct Wall;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct WallBundle {
    wall: Wall,
    respawnable: Respawnable,
}

#[derive(Component, Default)]
#[require(
    Wall,
    Hitbox,
    CollisionLayers::new(
        PhysicsLayers::Spikes,
        [PhysicsLayers::Enemy, PhysicsLayers::Player]
    )
)]
pub struct Spike;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct SpikeBundle {
    spike: Spike,
}

#[derive(Component, Default)]
#[require(
    Wall,
    TnuaGhostPlatform,
    CollisionLayers::new(PhysicsLayers::Ground, PhysicsLayers::Default)
)]
pub struct Platform;

#[derive(Bundle, LdtkIntCell, Default)]
pub struct PlatformBundle {
    platform: Platform,
}

pub fn tnua_platforms(mut q_tnua: Query<(&mut TnuaProximitySensor, &TnuaGhostSensor)>) {
    const MIN_PROXIMITY: f32 = 3.0;
    for (mut proximity_sensor, ghost_sensor) in q_tnua.iter_mut() {
        for ghost_platform in ghost_sensor.iter() {
            if MIN_PROXIMITY <= ghost_platform.proximity {
                proximity_sensor.output = Some(ghost_platform.clone());
                break;
            }
        }
    }
}
