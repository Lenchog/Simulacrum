use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::render::camera::ScalingMode;
use bevy::{
    core_pipeline::{
        bloom::Bloom,
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
};

use crate::robot::PhysicsLayers;
use crate::robot::player::Player;

pub mod general_movement;
pub mod robot;

#[derive(Component, PartialEq)]
pub struct Floor;

#[derive(Resource)]
pub struct MouseCoordinates(pub Vec2);

#[derive(Component)]
pub struct Despawnable;

#[derive(Component)]
pub struct Recoil;

pub fn add_floor(asset_server: &AssetServer) -> impl Bundle {
    let layers = CollisionLayers::new(
        PhysicsLayers::Ground,
        [
            PhysicsLayers::Enemy,
            PhysicsLayers::Player,
            PhysicsLayers::PlayerProjectile,
            PhysicsLayers::EnemyProjectile,
        ],
    );
    (
        layers,
        Sprite::from_image(asset_server.load("placeholder_floor.png")),
        Floor,
        RigidBody::Static,
        Collider::rectangle(1920.0, 500.0),
        Transform::from_xyz(0.0, -500.0, 0.0),
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

pub fn move_camera(
    q_camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player: Single<&Transform, With<Player>>,
    time: Res<Time<Fixed>>,
) {
    let player = q_player.into_inner().translation;
    q_camera.into_inner().translation = q_camera
        .translation
        .lerp(player, 0.1 * time.delta_secs() * 62.5);
}
