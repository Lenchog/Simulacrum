use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::PresentMode};
use no_mouth::Direction;
use no_mouth::player::run::{check_decellerate, decellerate, get_horizontal_input, move_horizontal};
use no_mouth::{DoubleJump, MovementConfig, general_movement::*, player::jump, setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(MovementConfig {
            gravity: 150.0,
            jump: 18.0,
            hold_jump: 1.55,
            horizontal: 1.0,
            decelleration: 2.5,
        })
        .insert_resource(DoubleJump(true))
        .insert_resource(Direction(0.0))
        .add_systems(Startup, setup::setup)
        .add_systems(
            Update,
            (smooth_movement, jump::jump.run_if(jump::check_jump)),
        )
        .add_systems(
            FixedUpdate,
            (
                gravity,
                update_movement,
                get_horizontal_input,
                move_horizontal,
                jump::hold_jump.run_if(jump::check_hold_jump),
                decellerate.run_if(check_decellerate)
            ),
        )
        .run();
}
