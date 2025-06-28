use crate::{general_movement::Grounded, player::{input::{Jump, Move}, *}, *};
use avian2d::prelude::*;
use bevy_enhanced_input::prelude::{Fired, Started};

pub fn jump(
    _: Trigger<Started<Jump>>,
    mut query: Query<(&mut LinearVelocity, Option<&Grounded>), With<Player>>,
    movement_config: Res<MovementConfig>,
    mut double_jump: ResMut<DoubleJump>,
) {
    let (mut velocity, grounded) = query.single_mut().expect("No player found!");
    // only jump if you're either grounded or have a double jump
    if !(grounded.is_some() || double_jump.0) {
        return;
    }
    double_jump.0 = grounded.is_some();
    velocity.y = movement_config.jump;
}

pub fn hold_jump(
    _: Trigger<Fired<Jump>>,
    mut velocity: Query<&mut LinearVelocity, With<Player>>,
    movement_config: Res<MovementConfig>,
) {
    let velocity = &mut velocity.single_mut().expect("Player not found!").y;
    if *velocity <= 0.0 {
        return;
    };
    *velocity += movement_config.hold_jump;
}

pub fn move_horizontal(
    trigger: Trigger<Fired<Move>>,
    movement_config: Res<MovementConfig>,
    mut velocity: Query<&mut LinearVelocity, With<Player>>,
) {
    let direction = trigger.value.extend(0.0).x;
    velocity.single_mut().expect("Player not found!").x =
        direction * movement_config.acceleration;
}
