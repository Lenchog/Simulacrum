use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::PresentMode};
use no_mouth::player::run::{
    accelerate, check_decellerate, decellerate, friction, get_horizontal_input,
};
use no_mouth::{Actionable, Direction, Physics};
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
        .insert_resource(Actionable(true))
        .insert_resource(Physics(true))
        .add_systems(Startup, setup::setup)
        .add_systems(
            Update,
            (
                smooth_movement,
                jump::jump.run_if(jump::check_jump).run_if(check_actionable),
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                (
                    gravity,
                    update_movement,
                    get_horizontal_input,
                    decellerate.run_if(check_decellerate),
                    friction,
                ).run_if(check_physics),
                (
                    jump::hold_jump.run_if(jump::check_hold_jump), 
                    accelerate
                ).run_if(check_actionable),
            ),
        )
        .run();
}
