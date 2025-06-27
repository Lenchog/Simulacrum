#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use avian2d::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::PresentMode};
use no_mouth::player::run::move_horizontal;
use no_mouth::{Actionable, Direction, PhysicsEnabled};
use no_mouth::{DoubleJump, MovementConfig, general_movement::*, player::jump, setup};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default()
                .with_length_unit(20.0)
                .set(PhysicsInterpolationPlugin::interpolate_all()),
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            bevy_framepace::FramepacePlugin,
        ))
        .insert_resource(MovementConfig {
            jump: 1400.0,
            hold_jump: 120.0,
            acceleration: 900.0,
        })
        .insert_resource(Gravity(Vec2::NEG_Y * 12000.0))
        .insert_resource(DoubleJump(true))
        .insert_resource(Direction(0.0))
        .insert_resource(Actionable(true))
        .insert_resource(PhysicsEnabled(true))
        .add_systems(Startup, setup::setup)
        .add_systems(
            Update,
            (jump::jump.run_if(jump::check_jump).run_if(check_actionable),),
        )
        .add_systems(
            FixedUpdate,
            (
                update_grounded,
                (
                    move_horizontal,
                    jump::hold_jump.run_if(jump::check_hold_jump),
                )
                    .run_if(check_actionable),
            ),
        )
        .run();
}
