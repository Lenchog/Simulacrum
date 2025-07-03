use crate::player::PlayerCollider;
use crate::player::input::NormalMovement;
use crate::player::weapons::{RotationCenter, WeaponBundle, WeaponTip};
use crate::{Enemy, EnemyCollider, Floor, Health, HealthBar, Player};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_enhanced_input::prelude::Actions;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 1080.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
    commands.spawn((
        Player,
        (
            RigidBody::Dynamic,
            children![
                (
                    PlayerCollider,
                    Transform::from_xyz(8.0, -80.0, 1.0),
                    Collider::capsule(50.0, 60.0)
                ),
                (
                    Transform::default(),
                    RotationCenter,
                    Visibility::Visible,
                    children![(
                        (
                            WeaponTip,
                            children![
                                WeaponBundle::default(),
                                Sprite::from_image(asset_server.load("placeholder_gun.png")),
                            ]
                        ),
                        Transform::from_xyz(200.0, 0.0, 0.0)
                    )]
                )
            ],
        ),
        LockedAxes::ROTATION_LOCKED,
        // main transform
        Transform::from_xyz(0.0, 500.0, 1.0),
        Sprite::from_image(asset_server.load("placeholder_robot.png")),
        Actions::<NormalMovement>::default(),
        Health(500),
    ));
    commands.spawn((
        Enemy,
        (
            RigidBody::Dynamic,
            children![(
                EnemyCollider,
                Transform::from_xyz(8.0, -80.0, 1.0),
                Collider::capsule(50.0, 60.0)
            ),],
        ),
        LockedAxes::ROTATION_LOCKED,
        // main transform
        Transform::from_xyz(-250.0, 500.0, 1.0),
        Sprite::from_image(asset_server.load("placeholder_robot.png")),
        Health(250),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("placeholder_floor.png")),
        Floor,
        RigidBody::Static,
        Collider::rectangle(1920.0, 500.0),
        Transform::from_xyz(0.0, -500.0, 0.0),
    ));
    commands.spawn((HealthBar, Text::new("hello world")));
}
