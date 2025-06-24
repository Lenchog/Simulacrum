use crate::{general_movement::touching_floor, *};

pub fn jump(
    mut player_state: Query<&mut MovementState, With<Player>>,
    movement_config: Res<MovementConfig>,
    mut double_jump: ResMut<DoubleJump>,
) {
    let mut player_state = player_state.single_mut().expect("No player found!");
    let touching_floor = touching_floor(player_state.position);
    if !touching_floor {
        double_jump.0 = false;
    }
    player_state.velocity.y = movement_config.jump;
}

pub fn hold_jump(
    mut player_state: Query<&mut MovementState, With<Player>>,
    movement_config: Res<MovementConfig>,
) {
    player_state
        .single_mut()
        .expect("Player not found!")
        .velocity
        .y += movement_config.hold_jump;
}

#[must_use]
pub fn check_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_state: Query<&MovementState, With<Player>>,
    double_jump: Res<DoubleJump>,
) -> bool {
    keyboard_input.just_pressed(KeyCode::Space)
        && (touching_floor(player_state.single().expect("Player not found!").position)
            || double_jump.0)
}

#[must_use]
pub fn check_hold_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_state: Query<&MovementState, With<Player>>,
) -> bool {
    keyboard_input.pressed(KeyCode::Space)
        && player_state.single().expect("Player not found!").velocity.y > 0.0
}
