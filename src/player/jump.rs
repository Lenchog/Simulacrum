use crate::*;
use avian2d::prelude::*;

pub fn jump(
    mut query: Query<(&mut LinearVelocity, Option<&Grounded>), With<Player>>,
    movement_config: Res<MovementConfig>,
    mut double_jump: ResMut<DoubleJump>,
) {
    let (mut velocity, grounded) = query.single_mut().expect("No player found!");
    if !grounded.is_some() {
        double_jump.0 = false;
    } else {
        double_jump.0 = true;
    }
    velocity.y = movement_config.jump;
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

pub fn hold_jump(
    mut velocity: Query<&mut LinearVelocity, With<Player>>,
    movement_config: Res<MovementConfig>,
) {
    velocity.single_mut().expect("Player not found!").y += movement_config.hold_jump;
}

#[must_use]
pub fn check_hold_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    velocity: Query<&LinearVelocity, With<Player>>,
) -> bool {
    keyboard_input.pressed(KeyCode::Space) && velocity.single().expect("Player not found!").y > 0.0
}
