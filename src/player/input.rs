use avian2d::prelude::LinearVelocity;
use bevy_enhanced_input::prelude::*;

use crate::{general_movement::Grounded, player::*, *};

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct Jump;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct Move;

#[derive(InputContext)]
pub struct NormalMovement;

pub fn bind(trigger: Trigger<Bind<NormalMovement>>, mut players: Query<&mut Actions<NormalMovement>>) {
     let mut actions = players.get_mut(trigger.target()).unwrap();
     actions
         .bind::<Move>()
         .to((Cardinal::wasd_keys(), Axial::left_stick(), Cardinal::arrow_keys()))
         .with_modifiers((
                 DeadZone::default(),
                 SmoothNudge::default(),
         ));
     actions.bind::<Jump>().to((KeyCode::Space, KeyCode::ArrowUp, KeyCode::KeyW));
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
