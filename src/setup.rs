use crate::{MovementState, Player};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

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
        Sprite::from_image(asset_server.load("placeholder_robot.png")),
        Transform::from_scale(Vec3 {
            x: 0.07,
            y: 0.07,
            z: 0.07,
        }),
        MovementState {
            position: Vec2 { x: 0.0, y: 440.0 },
            velocity: Vec2 { x: 0.0, y: 0.0 },
        },
        Player,
    ));
}
