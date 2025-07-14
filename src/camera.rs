use crate::robot::player::Player;
use bevy::render::camera::ScalingMode;
use bevy::{
    core_pipeline::{
        bloom::Bloom,
        tonemapping::{DebandDither, Tonemapping},
    },
    prelude::*,
};
use bevy_simple_subsecond_system::hot;

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

#[hot]
pub fn move_camera(
    q_camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player: Single<&Transform, With<Player>>,
    time: Res<Time<Fixed>>,
) {
    let player = q_player.into_inner().translation;
    q_camera.into_inner().translation = q_camera
        .translation
        .lerp(player, 0.2 * time.delta_secs() * 62.5);
}
