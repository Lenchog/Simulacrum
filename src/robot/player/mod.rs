use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Actions;

use crate::{
    Recoil,
    robot::{
        PhysicsLayers,
        health::*,
        player::{
            input::NormalMovement,
            weapons::{RotationCenter, WeaponTip},
        },
        robot, robot_collider,
    },
};

pub mod input;
pub mod movement;
pub mod weapons;

#[derive(Component, PartialEq)]
pub struct Player;

#[derive(Resource)]
pub struct MovementConfig {
    pub jump: f32,
    pub hold_jump: f32,
    pub acceleration: f32,
}

#[derive(Resource)]
pub struct DoubleJump(pub bool);

#[derive(Resource)]
pub struct Direction(pub f32);

#[derive(Resource)]
pub struct EquippedWeapons {
    pub left: Option<Entity>,
    pub right: Option<Entity>,
}

#[derive(Component, PartialEq)]
pub struct PlayerCollider;
pub fn add_player(asset_server: &AssetServer) -> impl Bundle {
    let layers = CollisionLayers::new(
        PhysicsLayers::Player,
        [
            PhysicsLayers::Ground,
            PhysicsLayers::EnemyProjectile,
            PhysicsLayers::Enemy,
        ],
    );
    (
        Player,
        Actions::<NormalMovement>::default(),
        Health(500),
        Recoil,
        (
            robot(asset_server),
            (
                RigidBody::Dynamic,
                children![
                    (layers, PlayerCollider, robot_collider()),
                    player_weapon_center()
                ],
            ),
        ),
    )
}

pub fn player_weapon_center() -> impl Bundle {
    let weapon_tip = (
        Visibility::Inherited,
        WeaponTip,
        Transform::from_xyz(200.0, 0.0, 0.0),
    );
    (
        Transform::default(),
        RotationCenter,
        Visibility::Inherited,
        children![weapon_tip],
    )
}
