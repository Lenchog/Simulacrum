use bevy_enhanced_input::input_context::input_modifier::linear_accelerate::LinearAccelerate;
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

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct SelectLeft;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct SelectRight;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct WeaponOne;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct WeaponTwo;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct WeaponThree;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct WeaponFour;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct WeaponFive;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct WeaponSix;

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
        .with_modifiers((DeadZone::default(), LinearAccelerate::new(0.1)));
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
    actions.bind::<SelectLeft>().to(KeyCode::KeyQ);
    actions.bind::<WeaponOne>().to(KeyCode::Digit1);
    actions.bind::<WeaponTwo>().to(KeyCode::Digit2);
    actions.bind::<WeaponThree>().to(KeyCode::Digit3);
    actions.bind::<WeaponFour>().to(KeyCode::Digit4);
    actions.bind::<WeaponFive>().to(KeyCode::Digit5);
    actions.bind::<WeaponSix>().to(KeyCode::Digit6);
    actions.bind::<SelectRight>().to(KeyCode::KeyE);
    actions
        .bind::<Dash>()
        .to((KeyCode::KeyF, GamepadButton::West));
}
