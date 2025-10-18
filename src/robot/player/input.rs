use crate::{
    prelude::*,
    weapons::{attack::attack, weapon_input::*},
};
use bevy_enhanced_input::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin)
            .add_input_context::<Player>()
            .add_observer(jump)
            .add_observer(dash)
            .add_observer(attack)
            .add_observer(select_left)
            .add_observer(select_right)
            .add_observer(equip_sword)
            .add_observer(equip_gun)
            .add_observer(equip_fast_gun)
            .add_observer(equip_power_gun)
            .add_observer(equip_rocket_launcher)
            .add_observer(equip_grappling_hook)
            .insert_resource(EquippedWeapons {
                left: None,
                right: None,
            })
            .add_systems(FixedUpdate, (equip_weapon, update_mouse_coords))
            .insert_resource(MouseCoordinates(Vec2::default()))
            .insert_resource(SelectedHand::Left)
            .insert_resource(EquippedWeapons {
                left: None,
                right: None,
            })
            .add_message::<EquipMessage>();
    }
}

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
#[action_output(bool)]
pub struct Heal;

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

pub fn bind(add: On<Add, Player>, mut commands: Commands) {
    commands.entity(add.event().event_target()).insert(actions!(Player[
        (
            Action::<MoveAction>::new(),
            DeadZone::default(),
            Bindings::spawn((
                Cardinal::wasd_keys(),
                Axial::left_stick(),
                Cardinal::arrows(),
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
                (Action::<Attack>::new(),
                bindings![
                    MouseButton::Left,
                    MouseButton::Right,
                    GamepadButton::LeftTrigger,
                    GamepadButton::RightTrigger,
                ],
            ),
            (Action::<Heal>::new(), bindings![KeyCode::ShiftLeft]),
            (Action::<SelectLeft>::new(), bindings![KeyCode::KeyQ]),
            (Action::<SelectRight>::new(), bindings![KeyCode::KeyE]),
            (Action::<WeaponOne>::new(), bindings![KeyCode::Digit1]),
            (Action::<WeaponTwo>::new(), bindings![KeyCode::Digit2]),
            //(Action::<WeaponThree>::new(), bindings![KeyCode::Digit3]),
            (Action::<WeaponFour>::new(), bindings![KeyCode::Digit4]),
            (Action::<WeaponFive>::new(), bindings![KeyCode::Digit5]),
            (Action::<WeaponSix>::new(), bindings![KeyCode::Digit6]),
    ]));
}
