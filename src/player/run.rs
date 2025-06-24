use crate::*;

pub fn get_horizontal_input(
    current_direction: Query<&MovementState, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut direction: ResMut<Direction>,
) {
    let left_pressed = keyboard_input.pressed(KeyCode::ArrowLeft);
    let right_pressed = keyboard_input.pressed(KeyCode::ArrowRight);

    direction.0 = match (left_pressed, right_pressed) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        (true, true) => current_direction
            .single()
            .expect("Player not found!")
            .velocity
            .x
            .clamp(-1.0, 1.0),
        _ => 0.0, // No relevant keys pressed
    };
}

pub fn move_horizontal(
    mut movement_state: Query<&mut MovementState, With<Player>>,
    direction: Res<Direction>,
    movement_config: Res<MovementConfig>,
) {
    movement_state
        .single_mut()
        .expect("Could not find player!")
        .velocity
        .x += movement_config.horizontal * direction.0;
}

pub fn decellerate(
    mut movement_state: Query<&mut MovementState, With<Player>>,
    movement_config: Res<MovementConfig>,
) {
    let velocity = &mut movement_state
        .single_mut()
        .expect("Could not find player!")
        .velocity
        .x;
    *velocity = match *velocity {
        // not just 0.0 to prevent overcorrecting
        -2.0..2.0 => 0.0,
        0.0.. => *velocity - movement_config.decelleration,
        ..0.0 => *velocity + movement_config.decelleration,
        _ => unreachable!(),
    };
    dbg!(velocity);
}

pub fn check_decellerate(
    direction: Res<Direction>,
) -> bool {
    // if the keys aren't being pressed
    direction.0 == 0.0
}
