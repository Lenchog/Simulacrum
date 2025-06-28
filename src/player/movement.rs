use crate::{general_movement::Grounded, player::*, *};
use avian2d::prelude::*;

pub fn jump(
    mut query: Query<(&mut LinearVelocity, Option<&Grounded>), With<Player>>,
    movement_config: Res<MovementConfig>,
    mut double_jump: ResMut<DoubleJump>,
) {
    let (mut velocity, grounded) = query.single_mut().expect("No player found!");
    double_jump.0 = grounded.is_some();
    velocity.y = movement_config.jump;
}

pub fn hold_jump(
    mut velocity: Query<&mut LinearVelocity, With<Player>>,
    movement_config: Res<MovementConfig>,
) {
    velocity.single_mut().expect("Player not found!").y += movement_config.hold_jump;
}

pub fn move_horizontal(
    movement_config: Res<MovementConfig>,
    mut velocity: Query<&mut LinearVelocity, With<Player>>,
    direction: Res<Direction>,
) {
    velocity.single_mut().expect("Player not found!").x =
        direction.0 * movement_config.acceleration;
}
