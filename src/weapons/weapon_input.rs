use crate::{prelude::*, weapons::prelude::*};
use bevy_enhanced_input::prelude::*;

#[derive(Resource)]
pub enum SelectedHand {
    Left,
    Right,
}

pub fn select_left(_: On<Fire<SelectLeft>>, mut selected_hand: ResMut<SelectedHand>) {
    *selected_hand = SelectedHand::Left;
}
pub fn select_right(_: On<Fire<SelectRight>>, mut selected_hand: ResMut<SelectedHand>) {
    *selected_hand = SelectedHand::Right;
}
#[derive(Message)]
pub struct EquipMessage(WeaponType);

pub fn equip_sword(_: On<Fire<WeaponOne>>, mut ev_weapon: MessageWriter<EquipMessage>) {
    ev_weapon.write(EquipMessage(WeaponType::Sword));
}
pub fn equip_gun(_: On<Fire<WeaponTwo>>, mut ev_weapon: MessageWriter<EquipMessage>) {
    ev_weapon.write(EquipMessage(WeaponType::Gun));
}
pub fn equip_fast_gun(_: On<Fire<WeaponThree>>, mut ev_weapon: MessageWriter<EquipMessage>) {
    ev_weapon.write(EquipMessage(WeaponType::FastGun));
}
pub fn equip_power_gun(_: On<Fire<WeaponFour>>, mut ev_weapon: MessageWriter<EquipMessage>) {
    ev_weapon.write(EquipMessage(WeaponType::PowerGun));
}
pub fn equip_rocket_launcher(_: On<Fire<WeaponFive>>, mut ev_weapon: MessageWriter<EquipMessage>) {
    ev_weapon.write(EquipMessage(WeaponType::RocketLauncher));
}
pub fn equip_grappling_hook(_: On<Fire<WeaponSix>>, mut ev_weapon: MessageWriter<EquipMessage>) {
    ev_weapon.write(EquipMessage(WeaponType::GrappleHook));
}

pub fn equip_weapon(
    mut ev_weapon: MessageReader<EquipMessage>,
    mut r_weapons: ResMut<EquippedWeapons>,
    tip_entity: Single<Entity, With<WeaponTip>>,
    asset_server: Res<AssetServer>,
    selected_hand: Res<SelectedHand>,
    mut commands: Commands,
) {
    for event in ev_weapon.read() {
        let entity = match event.0 {
            WeaponType::Sword => commands.spawn(sword(&asset_server, *tip_entity)).id(),
            WeaponType::Gun => commands.spawn(lazer_gun(&asset_server, *tip_entity)).id(),
            WeaponType::FastGun => commands.spawn(faster_gun(&asset_server, *tip_entity)).id(),
            WeaponType::PowerGun => commands.spawn(power_gun(&asset_server, *tip_entity)).id(),
            WeaponType::RocketLauncher => commands
                .spawn(rocket_launcher(&asset_server, *tip_entity))
                .id(),
            WeaponType::GrappleHook => commands
                .spawn(grappling_hook(&asset_server, *tip_entity))
                .id(),
        };
        let (selected_weapon, other_weapon) = match *selected_hand {
            SelectedHand::Left => (r_weapons.left, r_weapons.right),
            SelectedHand::Right => (r_weapons.right, r_weapons.left),
        };
        if let Some(equipped_weapon) = selected_weapon {
            commands.entity(equipped_weapon).despawn();
        }
        match *selected_hand {
            SelectedHand::Left => r_weapons.left = Some(entity),
            SelectedHand::Right => r_weapons.right = Some(entity),
        }
        if selected_weapon == other_weapon
            && let Some(other_weapon) = other_weapon
        {
            commands.entity(other_weapon).despawn()
        }
    }
}
