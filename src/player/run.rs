use crate::{general_movement::touching_floor, *};

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

pub fn accelerate(
    mut movement_state: Query<&mut MovementState, With<Player>>,
    direction: Res<Direction>,
    movement_config: Res<MovementConfig>,
) {
    movement_state
        .single_mut()
        .expect("Could not find player!")
        .velocity
        .x += movement_config.acceleration * direction.0;
}

pub fn friction(
    mut movement_state: Query<&mut MovementState, With<Player>>,
    movement_config: Res<MovementConfig>,
) {
    let velocity = &mut movement_state
        .single_mut()
        .expect("Could not find player!")
        .velocity
        .x;
    *velocity -= *velocity * movement_config.friction;
}

pub fn player_grounded(movement_state: Query<&MovementState, With<Player>>) -> bool {
    touching_floor(movement_state.single().expect("Could not find player!").position)
}

pub fn decellerate(
    mut movement_state: Query<&mut MovementState, With<Player>>,
    movement_config: Res<MovementConfig>,
) {
    let decelleration = if touching_floor(movement_state.single_mut().expect("Could not find player!").position) {
        movement_config.decelleration
    } else {
        movement_config.decelleration / 3.0
    };
    let velocity = &mut movement_state
        .single_mut()
        .expect("Could not find player!")
        .velocity
        .x;
    *velocity = match *velocity {
        // not just 0.0 to prevent overcorrecting
        -2.0..2.0 => 0.0,
        0.0.. => *velocity - decelleration,
        ..0.0 => *velocity + decelleration,
        _ => unreachable!(),
    };
}

pub fn check_decellerate(
    direction: Res<Direction>,
    movement_state: Query<&MovementState, With<Player>>,
) -> bool {
    let controls = direction.0;
    let velocity = movement_state.single().expect("Could not find player!").velocity.x;
    // if the keys aren't being pressed
    controls == 0.0 && velocity != 0.0 ||
        // if control direction is not the same as velocity
        (controls >= 0.0) != (velocity >= 0.0)
}
