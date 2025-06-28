#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use avian2d::prelude::*;
use bevy::{prelude::*, window::PresentMode};
use bevy_enhanced_input::prelude::*;
use no_mouth::{
    general_movement::*,
    player::{input, movement, *},
    *,
};

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
            bevy_framepace::FramepacePlugin,
            EnhancedInputPlugin,
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
            (movement::jump
                .run_if(input::check_jump)
                .run_if(check_actionable),),
        )
        .add_systems(
            FixedUpdate,
            (
                update_grounded,
                (
                    input::get_horizontal_input,
                    movement::move_horizontal,
                    movement::hold_jump.run_if(input::check_hold_jump),
                )
                    .run_if(check_actionable),
            ),
        )
        .run();
}
