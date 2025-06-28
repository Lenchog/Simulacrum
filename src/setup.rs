use crate::player::input::NormalMovement;
use crate::{Floor, Player};
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
        RigidBody::Dynamic,
        Collider::capsule(12.5, 20.0),
        LockedAxes::ROTATION_LOCKED,
        Transform::from_xyz(0.0, 500.0, 0.0),
        Sprite::from_image(asset_server.load("placeholder_robot.png")),
        Actions::<NormalMovement>::default(),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("placeholder_floor.png")),
        Floor,
        RigidBody::Static,
        Collider::rectangle(1920.0, 940.0),
        //TODO make this scale properly
        Transform::from_xyz(0.0, -500.0, 0.0),
    ));
}
