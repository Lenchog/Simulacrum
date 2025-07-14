use bevy_enhanced_input::prelude::*;

use crate::robot::player::*;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct Jump;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct Dash;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct Attack;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct Move;

#[derive(InputContext)]
pub struct NormalMovement;

pub fn bind(
    trigger: Trigger<Bind<NormalMovement>>,
    mut players: Query<&mut Actions<NormalMovement>>,
) {
    let mut actions = players.get_mut(trigger.target()).unwrap();
    actions
        .bind::<Move>()
        .to((
            Cardinal::wasd_keys(),
            Axial::left_stick(),
            Cardinal::arrow_keys(),
        ))
        .with_modifiers((DeadZone::default(), SmoothNudge::default()));
    actions.bind::<Jump>().to((
        KeyCode::Space,
        KeyCode::ArrowUp,
        KeyCode::KeyW,
        GamepadButton::South,
    ));
    actions.bind::<Attack>().to((
        MouseButton::Left,
        MouseButton::Right,
        GamepadButton::LeftTrigger2,
        GamepadButton::RightTrigger2,
    ));
    actions
        .bind::<Dash>()
        .to((KeyCode::KeyF, GamepadButton::West));
}
