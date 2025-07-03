use crate::player::PlayerCollider;
use crate::player::health::HealthBar;
use crate::player::weapons::{RotationCenter, WeaponBundle, WeaponTip};
use crate::{CameraBundle, EnemyBundle, EnemyCollider, FloorBundle, PlayerBundle};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CameraBundle::new());
    commands.spawn((
        PlayerBundle::new(asset_server.clone()),
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
    ));
    commands.spawn((
        EnemyBundle::new(asset_server.clone()),
        (
            RigidBody::Dynamic,
            children![(
                EnemyCollider,
                Transform::from_xyz(8.0, -80.0, 1.0),
                Collider::capsule(50.0, 60.0)
            )],
        ),
    ));
    commands.spawn(FloorBundle::new(asset_server.clone()));
    commands.spawn((HealthBar, Text::default()));
}
