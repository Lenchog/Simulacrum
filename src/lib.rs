use avian2d::prelude::{Collider, CollisionLayers, LockedAxes, PhysicsLayer, RigidBody};
use bevy::{
    core_pipeline::{
        bloom::Bloom,
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
};
use bevy_enhanced_input::prelude::Actions;
use bevy::render::camera::ScalingMode;
use crate::player::{health::Health, input::NormalMovement};

pub mod general_movement;
pub mod player;
pub mod setup;

#[derive(Component, PartialEq)]
pub struct Player;

#[derive(Component, PartialEq)]
pub struct Enemy;

#[derive(Component, PartialEq)]
pub struct EnemyCollider;

#[derive(Component, PartialEq)]
pub struct Floor;

#[derive(Resource)]
pub struct MouseCoordinates(pub Vec2);

#[derive(PhysicsLayer, Default)]
pub enum PhysicsLayers {
    #[default]
    Default,
    Ground,
    Player,
    Enemy,
    PlayerProjectile,
    EnemyProjectile,
}

#[derive(PartialEq, Default)]
pub enum RobotType {
    #[default]
    Enemy,
    Player,
}

#[derive(Bundle)]
pub struct FloorBundle {
    sprite: Sprite,
    floor: Floor,
    rigid_body: RigidBody,
    collider: Collider,
    transform: Transform,
}
impl FloorBundle {
    fn new(asset_server: AssetServer) -> Self {
        Self {
            sprite: Sprite::from_image(asset_server.load("placeholder_floor.png")),
            floor: Floor,
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(1920.0, 500.0),
            transform: Transform::from_xyz(0.0, -500.0, 0.0),
        }
    }
}

#[derive(Bundle)]
pub struct RobotBundle {
    sprite: Sprite,
    locked_axes: LockedAxes,
    transform: Transform,
    health: Health,
}
impl RobotBundle {
    fn new(asset_server: AssetServer) -> Self {
        Self {
            sprite: Sprite::from_image(asset_server.load("placeholder_robot.png")),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            transform: Transform::from_xyz(-250.0, 500.0, 1.0),
            health: Health(250),
        }
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    robot: RobotBundle,
    enemy: Enemy,
    layer: CollisionLayers,
}
impl EnemyBundle {
    fn new(asset_server: AssetServer) -> Self {
        let enemy_layer = CollisionLayers::new(
            PhysicsLayers::Enemy,
            [
                PhysicsLayers::Ground,
                PhysicsLayers::Player,
                PhysicsLayers::Enemy,
                PhysicsLayers::PlayerProjectile,
            ],
        );
        Self {
            enemy: Enemy,
            layer: enemy_layer,
            robot: RobotBundle {
                ..RobotBundle::new(asset_server)
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    robot: RobotBundle,
    player: Player,
    layer: CollisionLayers,
    actions: Actions<NormalMovement>,
}
impl PlayerBundle {
    fn new(asset_server: AssetServer) -> Self {
        let player_layer = CollisionLayers::new(
            PhysicsLayers::Player,
            [
                PhysicsLayers::Ground,
                PhysicsLayers::Enemy,
                PhysicsLayers::EnemyProjectile,
            ],
        );
        Self {
            player: Player,
            layer: player_layer,
            robot: RobotBundle {
                transform: Transform::from_xyz(0.0, 500.0, 0.0),
                health: Health(500),
                ..RobotBundle::new(asset_server.clone())
            },
            actions: Actions::<NormalMovement>::default(),
        }
    }
}
#[derive(Bundle)]
pub struct CameraBundle {
    camera2d: Camera2d,
    projection: Projection,
    camera: Camera,
    tonemapping: Tonemapping,
    bloom: Bloom,
    deband_dither: DebandDither,
}
impl CameraBundle {
    fn new() -> Self {
        Self {
            camera2d: Camera2d,
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: 2160.0,
                },
                ..OrthographicProjection::default_2d()
            }),
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            bloom: Bloom::default(),
            deband_dither: DebandDither::Enabled,
        }
    }
}
