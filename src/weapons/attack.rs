use crate::{prelude::*, weapons::prelude::*};
use bevy_enhanced_input::prelude::Fired;

pub fn weapon_cooldown(
    q_weapon: Query<(Entity, &mut UseTime, &mut CooldownFinished)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut timer, mut cooldown_finished) in q_weapon {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).insert(Visibility::Hidden);
            commands.entity(entity).remove::<CollisionEventsEnabled>();
            *cooldown_finished = CooldownFinished(true);
            timer.0.reset();
        }
    }
}

pub fn attack(
    // this is related to the input system, just runs when you click
    _: Trigger<Fired<Attack>>,
    // this grabs information about all the weapons that are spawned
    mut q_weapon: Query<(
        Entity,
        &Weapon,
        &mut CooldownFinished,
        Option<&ProjectileBuilder>,
        Option<&Swingable>,
    )>,
    // the way weapon rotation works, is there is a fixed point on the player that rotates, and all weapons are attached to it
    q_rotation_center: Single<Entity, (Without<SwingRotation>, With<RotationCenter>)>,
    // this stores the entities of which weapons are equipped
    res_equipped_weapons: Res<EquippedWeapons>,
    // this is which buttons are pressed, so we can distinguish between left and right
    buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    // this allows us to tell the gun to shoot
    mut ev_shoot: EventWriter<ShootEvent>,
    mut commands: Commands,
) {
    // this checks whether you're pressing right or left click, and selects the weapon respectively
    let Some(weapon_entity) = (if buttons.pressed(MouseButton::Right) {
        res_equipped_weapons.right
    } else {
        res_equipped_weapons.left
    }) else {
        return;
    };
    for (weapon, _, _, _, _) in q_weapon.iter() {
        commands.entity(weapon).remove::<Equipped>();
    }
    let (entity, weapon_type, mut cooldown, projectile_builder, swingable) = q_weapon
        .get_mut(weapon_entity)
        .expect("could not get active weapon");
    commands.entity(weapon_entity).insert(Visibility::Visible);
    commands.entity(entity).insert(Equipped);
    if !cooldown.0 {
        return;
    };
    cooldown.0 = false;

    commands.spawn((
        SamplePlayer::new(asset_server.load(match weapon_type.0 {
            WeaponType::Gun | WeaponType::FastGun => {
                "audio/LASRGun_Particle Compressor Fire_01.wav"
            }
            WeaponType::PowerGun => "audio/GUNTech_Sci Fi Shotgun Fire_04.wav",
            WeaponType::RocketLauncher => "audio/GUNArtl_Rocket Launcher Fire_02.wav",
            _ => "audio/WHOOSH_ARM_SWING_01.wav",
        })),
        RandomPitch::new(0.2),
    ));
    if projectile_builder.is_some() {
        ev_shoot.write(ShootEvent(weapon_entity));
    } else if swingable.is_some() {
        commands.entity(weapon_entity).remove::<ColliderDisabled>();
        commands
            .entity(q_rotation_center.into_inner())
            .insert(SwingRotation(0.0));
    }
}
