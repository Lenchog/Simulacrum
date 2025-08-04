use crate::prelude::*;
use bevy_enhanced_input::prelude::*;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Jump;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Dash;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Attack;

#[derive(Debug, InputAction)]
#[action_output(Vec2)]
pub struct MoveAction;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct SelectLeft;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct SelectRight;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct WeaponOne;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct WeaponTwo;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct WeaponThree;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct WeaponFour;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct WeaponFive;

#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct WeaponSix;

pub fn bind(trigger: Trigger<OnAdd, Player>, mut commands: Commands) {
    commands.entity(trigger.target()).insert(actions!(Player[
        (
            Action::<MoveAction>::new(),
            DeadZone::default(),
            LinearStep::new(0.15, 0.15),
            Bindings::spawn((
                Cardinal::wasd_keys(),
                Axial::left_stick(),
                Cardinal::arrow_keys(),
            ))
        ),
        (
            Action::<Jump>::new(),
            bindings![
                KeyCode::Space,
                KeyCode::ArrowUp,
                KeyCode::KeyW,
                GamepadButton::South,
            ]
        ),
        (
            Action::<Dash>::new(),
            bindings![KeyCode::KeyF, GamepadButton::West]
        ),
        (
            Action::<Attack>::new(),
            bindings![
                MouseButton::Left,
                MouseButton::Right,
                GamepadButton::LeftTrigger2,
                GamepadButton::RightTrigger2,
            ]
        ),
        (Action::<SelectLeft>::new(), bindings![KeyCode::KeyQ]),
        (Action::<SelectRight>::new(), bindings![KeyCode::KeyE]),
        (Action::<WeaponOne>::new(), bindings![KeyCode::Digit1]),
        (Action::<WeaponTwo>::new(), bindings![KeyCode::Digit2]),
        (Action::<WeaponThree>::new(), bindings![KeyCode::Digit3]),
        (Action::<WeaponFour>::new(), bindings![KeyCode::Digit4]),
        (Action::<WeaponFive>::new(), bindings![KeyCode::Digit5]),
        (Action::<WeaponSix>::new(), bindings![KeyCode::Digit6]),
    ]));
}
