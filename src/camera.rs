use crate::prelude::*;
use bevy::{
    camera::ScalingMode,
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::bloom::Bloom,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera);
        app.add_systems(Startup, add_camera);
        app.add_observer(tp_camera);
    }
}
fn tp_camera(
    add: On<Add, Player>,
    q_transform: Query<&GlobalTransform, Without<Camera>>,
    q_camera: Single<&mut Transform, With<Camera>>,
) {
    let player_translation = q_transform.get(add.event().event_target()).unwrap().translation();
    q_camera.into_inner().translation = player_translation;
}

fn add_camera(mut commands: Commands) {
    let camera = (
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
        Camera::default(),
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
    );
    commands.spawn(camera);
}

pub fn move_camera(
    q_camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
    q_player: Single<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) {
    let player_translation = q_player.into_inner().translation();
    let camera_translation = &mut q_camera.into_inner().translation;
    if *camera_translation == Vec3::ZERO {
        *camera_translation = player_translation;
    }
    const CAMERA_SPEED: f32 = 0.15;
    const STANDARD_FPS: f32 = 60.0;
    *camera_translation = camera_translation.lerp(
        player_translation,
        CAMERA_SPEED * time.delta_secs() * STANDARD_FPS,
    );
}
