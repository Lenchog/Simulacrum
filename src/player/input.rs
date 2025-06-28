use avian2d::prelude::LinearVelocity;
use bevy_enhanced_input::prelude::*;

use crate::{general_movement::Grounded, player::*, *};

pub fn get_horizontal_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction: ResMut<Direction>,
) {
    let left_pressed = keyboard_input.pressed(KeyCode::ArrowLeft);
    let right_pressed = keyboard_input.pressed(KeyCode::ArrowRight);

    direction.0 = match (left_pressed, right_pressed) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        (true, true) => direction.0,
        _ => 0.0, // No relevant keys pressed
    };
}

#[must_use]
pub fn check_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    grounded: Query<Option<&Grounded>, With<Player>>,
    double_jump: Res<DoubleJump>,
) -> bool {
    let grounded = grounded.single().expect("could not find player").is_some();
    keyboard_input.just_pressed(KeyCode::Space) && (grounded || double_jump.0)
}

#[must_use]
pub fn check_hold_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    velocity: Query<&LinearVelocity, With<Player>>,
) -> bool {
    keyboard_input.pressed(KeyCode::Space) && velocity.single().expect("Player not found!").y > 0.0
}
