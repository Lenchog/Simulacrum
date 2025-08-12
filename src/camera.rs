use crate::prelude::*;
use bevy::core_pipeline::{
    bloom::Bloom,
    tonemapping::{DebandDither, Tonemapping},
};
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
    }
}
pub fn add_camera() -> impl Bundle {
    (
        Camera2d,
        Light2d {
            ambient_light: AmbientLight2d {
                brightness: 0.7,
                ..default()
            },
        },
        Shake::default(),
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
    q_player: Single<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) {
    let player = q_player.into_inner().translation();
    const CAMERA_SPEED: f32 = 0.15;
    const STANDARD_FPS: f32 = 60.0;
    q_camera.into_inner().translation = q_camera
        .translation
        .lerp(player, CAMERA_SPEED * time.delta_secs() * STANDARD_FPS);
}
