use crate::{
    general_ranged::*, grappling_hook::grappling_hook, melee::*, rocket_launcher::rocket_launcher,
    *,
};

#[derive(Resource)]
pub enum SelectedHand {
    Left,
    Right,
}

pub fn select_left(_: Trigger<Fired<SelectLeft>>, mut selected_hand: ResMut<SelectedHand>) {
    *selected_hand = SelectedHand::Left;
}
pub fn select_right(_: Trigger<Fired<SelectRight>>, mut selected_hand: ResMut<SelectedHand>) {
    *selected_hand = SelectedHand::Right;
}
#[derive(Event)]
pub struct EquipEvent(pub WeaponType);

pub fn equip_sword(_: Trigger<Fired<WeaponOne>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::Sword));
}
pub fn equip_gun(_: Trigger<Fired<WeaponTwo>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::Gun));
}
pub fn equip_fast_gun(_: Trigger<Fired<WeaponThree>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::FastGun));
}
pub fn equip_power_gun(_: Trigger<Fired<WeaponFour>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::PowerGun));
}
pub fn equip_rocket_launcher(
    _: Trigger<Fired<WeaponFive>>,
    mut ev_weapon: EventWriter<EquipEvent>,
) {
    ev_weapon.write(EquipEvent(WeaponType::RocketLauncher));
}
pub fn equip_grappling_hook(_: Trigger<Fired<WeaponSix>>, mut ev_weapon: EventWriter<EquipEvent>) {
    ev_weapon.write(EquipEvent(WeaponType::GrappleHook));
}

pub fn equip_weapon(
    mut ev_weapon: EventReader<EquipEvent>,
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
