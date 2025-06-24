use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::PresentMode};
use no_mouth::Direction;
use no_mouth::player::run::{accelerate, check_decellerate, decellerate, friction, get_horizontal_input, player_grounded};
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
            jump: 21.0,
            hold_jump: 1.55,
            acceleration: 1.4,
            decelleration: 0.7,
            friction: 0.07,
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
                accelerate,
                jump::hold_jump.run_if(jump::check_hold_jump),
                decellerate.run_if(check_decellerate),
                friction/* .run_if(player_grounded) */,
            ),
        )
        .run();
}
