use avian2d::prelude::LinearVelocity;

use crate::*;

pub fn move_horizontal(
    mut velocity_query: Query<&mut LinearVelocity, With<Player>>,
    movement_config: Res<MovementConfig>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction: ResMut<Direction>,
) {
    let left_pressed = keyboard_input.pressed(KeyCode::ArrowLeft);
    let right_pressed = keyboard_input.pressed(KeyCode::ArrowRight);
    let velocity = &mut velocity_query
        .single_mut()
        .expect("could not find player!")
        .x;
    let current_direction = velocity.clamp(-1.0, 1.0);

    direction.0 = match (left_pressed, right_pressed) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        (true, true) => current_direction,
        _ => 0.0, // No relevant keys pressed
    };
    *velocity = direction.0 * movement_config.acceleration;
}
