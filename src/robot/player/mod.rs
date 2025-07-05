use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Actions;

use crate::robot::{
    player::{
        input::NormalMovement,
        weapons::{RotationCenter, WeaponBundle, WeaponTip},
    },
    robot, robot_collider, 
    health::*,
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

#[derive(Component, PartialEq)]
pub struct PlayerCollider;
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
