use crate::player::weapons::{RotationCenter, WeaponBundle, WeaponTip};
use crate::player::{health::Health, input::NormalMovement};
use avian2d::prelude::{Collider, LockedAxes, PhysicsLayer, RigidBody};
use bevy::render::camera::ScalingMode;
use bevy::{
    core_pipeline::{
        bloom::Bloom,
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
};
use bevy_enhanced_input::prelude::Actions;

pub mod general_movement;
pub mod player;

#[derive(Component, PartialEq)]
pub struct Player;

#[derive(Component, PartialEq)]
pub struct Enemy;

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

pub fn add_floor(asset_server: AssetServer) -> impl Bundle {
    (
        Sprite::from_image(asset_server.load("placeholder_floor.png")),
        Floor,
        RigidBody::Static,
        Collider::rectangle(1920.0, 500.0),
        Transform::from_xyz(0.0, -500.0, 0.0),
    )
}

#[derive(Component)]
#[require(Health = Health(100))]
#[require(Transform = Transform::from_xyz(0.0, 500.0, 1.0))]
#[require(RigidBody = RigidBody::Dynamic)]
struct Robot;
fn robot(asset_server: &AssetServer) -> impl Bundle {
    (
        Robot,
        Sprite::from_image(asset_server.load("placeholder_robot.png")),
        LockedAxes::ROTATION_LOCKED,
        Transform::from_xyz(-250.0, 500.0, 1.0),
    )
}

#[derive(Component, PartialEq)]
pub struct EnemyCollider;
#[derive(Component, PartialEq)]
pub struct PlayerCollider;
pub fn robot_collider() -> impl Bundle {
    (
        Transform::from_xyz(8.0, -80.0, 1.0),
        Collider::capsule(50.0, 60.0),
    )
}
pub fn add_enemy(asset_server: &AssetServer) -> impl Bundle {
    (
        Enemy,
        (
            RigidBody::Dynamic,
            children![(EnemyCollider, robot_collider())],
        ),
        robot(asset_server),
    )
}
pub fn add_player(asset_server: &AssetServer) -> impl Bundle {
    (
        Player,
        Actions::<NormalMovement>::default(),
        Health(500),
        (
            robot(asset_server),
            (
                RigidBody::Dynamic,
                children![
                    (PlayerCollider, robot_collider()),
                    player_weapon_center(asset_server)
                ],
            ),
        ),
    )
}

pub fn player_weapon_center(asset_server: &AssetServer) -> impl Bundle {
    (
        Transform::default(),
        RotationCenter,
        //Visibility::Visible,
        children![(
            (
                WeaponTip,
                children![
                    WeaponBundle::default(),
                    Sprite::from_image(asset_server.load("placeholder_gun.png")),
                ]
            ),
            Transform::from_xyz(200.0, 0.0, 0.0)
        )],
    )
}
pub fn add_camera() -> impl Bundle {
    (
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 2160.0,
            },
            ..OrthographicProjection::default_2d()
        }),
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
    )
}

pub fn update_mouse_coords(
    mut coords: ResMut<MouseCoordinates>,
    q_window: Single<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    // idk how this works, copy-pasted from
    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    // and then fixed with clippy
    coords.0 = q_window
        .into_inner()
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.unwrap().origin.truncate())
        .unwrap_or_default();
}
